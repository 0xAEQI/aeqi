use axum::extract::{Query, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::StreamExt;
use std::sync::Arc;
use tracing::{info, warn, debug};

use system_core::Identity;
use system_core::config::{PeerAgentConfig, AgentRole, AgentVoice, ExecutionMode};
use system_orchestrator::AgentRouter;
use crate::AppState;
use crate::chat::{
    archetype_expertise, build_companion_tools, enrich_identity_with_project,
    execute_advisor_chat, execute_agent_chat, execute_chat, resolve_workspace, send_ws,
};
use crate::types::{WsClientMessage, WsServerMessage};

#[derive(serde::Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    // Authenticate before upgrading.
    let tenant = match state.manager.resolve_by_session(&query.token).await {
        Ok(Some(t)) => t,
        _ => {
            return axum::http::Response::builder()
                .status(401)
                .body(axum::body::Body::from("unauthorized"))
                .unwrap()
                .into_response();
        }
    };

    ws.on_upgrade(move |socket| handle_ws(socket, tenant, state))
        .into_response()
}

async fn handle_ws(socket: WebSocket, tenant: Arc<system_tenants::Tenant>, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    // Load roster and leader. Fall back to familiar for backward compat.
    let roster = tenant.companion_store.get_roster().unwrap_or_default();
    let leader = match tenant.companion_store.get_leader() {
        Ok(Some(l)) => l,
        _ => {
            // Fall back to familiar.
            match tenant.companion_store.get_familiar() {
                Ok(Some(f)) => f,
                _ => {
                    let _ = send_ws(&mut sender, &WsServerMessage::Error {
                        message: "no familiar set — pull a companion first".to_string(),
                    }).await;
                    return;
                }
            }
        }
    };

    let leader_name = leader.name.clone();
    let squad_names: Vec<String> = if roster.is_empty() {
        vec![leader_name.clone()]
    } else {
        roster.iter().map(|c| c.name.clone()).collect()
    };
    let advisors: Vec<String> = squad_names.iter()
        .filter(|n| **n != leader_name)
        .cloned()
        .collect();

    // Create agent router for advisor filtering (if OpenRouter key available).
    let mut router = state.platform.providers.openrouter.as_ref().map(|or| {
        AgentRouter::new(or.api_key.clone(), 30)
    });

    info!(tenant = %tenant.id, leader = %leader_name, squad = ?squad_names, "websocket connected");

    // Send party info.
    let _ = send_ws(&mut sender, &WsServerMessage::Party {
        leader: leader_name.clone(),
        squad: squad_names.clone(),
    }).await;

    // Send welcome from leader.
    let _ = send_ws(&mut sender, &WsServerMessage::Message {
        companion: leader_name.clone(),
        content: format!("*{} is here.* How can I help you?", leader_name),
        timestamp: chrono::Utc::now().timestamp(),
    }).await;

    let mut event_rx = tenant.event_tx.subscribe();

    loop {
        let msg = tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(t))) => t,
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Err(e)) => {
                        warn!(error = %e, "websocket error");
                        break;
                    }
                    _ => continue,
                }
            }
            event = event_rx.recv() => {
                match event {
                    Ok(ev) => {
                        let (companion, status_type, success) = match ev {
                            system_tenants::TenantEvent::PortraitReady { companion_name, success } =>
                                (companion_name, "portrait", success),
                            system_tenants::TenantEvent::PersonaReady { companion_name, success } =>
                                (companion_name, "persona", success),
                        };
                        let status = if success { "complete" } else { "failed" };
                        let _ = send_ws(&mut sender, &WsServerMessage::CompanionStatus {
                            companion,
                            status_type: status_type.to_string(),
                            status: status.to_string(),
                            timestamp: chrono::Utc::now().timestamp(),
                        }).await;
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        debug!(lagged = n, "event broadcast lagged, skipping old events");
                        continue;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                }
            }
        };

        let client_msg: WsClientMessage = match serde_json::from_str(&msg) {
            Ok(m) => m,
            Err(_) => {
                let _ = send_ws(&mut sender, &WsServerMessage::Error {
                    message: "invalid message format".to_string(),
                }).await;
                continue;
            }
        };

        match client_msg {
            WsClientMessage::Ping => {
                let _ = send_ws(&mut sender, &WsServerMessage::Pong).await;
            }
            WsClientMessage::Message { content } => {
                tenant.touch();

                // Check mana balance before executing chat.
                {
                    let db = state.manager.db().await;
                    let balance = system_tenants::economy::get_balance(&db, &tenant.id.0, &tenant.tier);
                    if let Ok(bal) = balance
                        && bal.mana <= 0
                    {
                        let _ = send_ws(&mut sender, &WsServerMessage::Error {
                            message: "no mana remaining — wait for daily reset or upgrade your tier".to_string(),
                        }).await;
                        continue;
                    }
                }

                // Record user message.
                let _ = tenant.conversation_store.record(0, "User", &content).await;

                // Send typing indicator for leader.
                let _ = send_ws(&mut sender, &WsServerMessage::Typing {
                    companion: leader_name.clone(),
                }).await;

                // Build leader's identity context.
                let agent_dir = tenant.data_dir.join("agents").join(&leader_name);
                let project_dir = tenant.data_dir.join("projects/chat");
                let mut identity = Identity::load(&agent_dir, Some(&project_dir))
                    .unwrap_or_default();

                // Inject active project knowledge into companion context.
                enrich_identity_with_project(&mut identity, &tenant).await;

                // Build leader's relationship context with squad.
                let leader_rel_ctx = {
                    let all_companions = tenant.companion_store.list_all().unwrap_or_default();
                    let leader_comp = all_companions.iter().find(|c| c.name == leader_name);
                    if let Some(leader) = leader_comp {
                        let mut lines = Vec::new();
                        for advisor_name in &advisors {
                            if let Some(other) = all_companions.iter().find(|c| c.name == *advisor_name)
                                && let Ok(rel) = tenant.companion_store.get_or_seed_relationship(leader, other)
                            {
                                lines.push(format!(
                                    "- **{}** ({:?} {:?}): {} — respect: {:.1}, affinity: {:.1}, rivalry: {:.1}",
                                    other.name, other.dere_type, other.archetype,
                                    rel.relationship_label(),
                                    rel.respect, rel.affinity, rel.rivalry,
                                ));
                            }
                        }
                        if lines.is_empty() {
                            String::new()
                        } else {
                            format!("\n\n## Your Relationships with Squad Members\n{}", lines.join("\n"))
                        }
                    } else {
                        String::new()
                    }
                };

                // Get recent conversation history.
                let history = tenant.conversation_store
                    .context_string(0, 20)
                    .await
                    .unwrap_or_default();

                // Execute leader chat with relationship context appended to history.
                let enriched_history = format!("{history}{leader_rel_ctx}");
                let active_project_name = tenant.active_project().await;
                let workspace = resolve_workspace(&tenant, active_project_name.as_deref());
                let response = if let Some(ref ws_path) = workspace {
                    let tools = build_companion_tools(ws_path);
                    execute_agent_chat(
                        &identity, &enriched_history, &content,
                        &tenant.tier.model, &state.platform, 1024,
                        tools, &mut sender, &leader_name,
                    ).await
                } else {
                    execute_chat(
                        &identity, &enriched_history, &content,
                        &tenant.tier.model, &state.platform, 1024,
                    ).await
                };

                let mut total_tokens: u32 = 0;

                let leader_response_text = match response {
                    Ok((text, token_count)) => {
                        total_tokens += token_count;

                        // Record leader response.
                        let _ = tenant.conversation_store.record(0, &leader_name, &text).await;

                        // Award bond XP to leader (25 per message).
                        if let Ok(Some(mut comp)) = tenant.companion_store.get_companion_by_name(&leader_name) {
                            comp.add_bond_xp(25);
                            let _ = tenant.companion_store.save_companion(&comp);
                        }

                        // Send leader response.
                        let _ = send_ws(&mut sender, &WsServerMessage::Message {
                            companion: leader_name.clone(),
                            content: text.clone(),
                            timestamp: chrono::Utc::now().timestamp(),
                        }).await;

                        text
                    }
                    Err(e) => {
                        warn!(error = %e, "leader chat execution failed");
                        let error_text = format!("*{} seems distracted...* (System error: {})", leader_name, e);
                        let _ = tenant.conversation_store.record(0, &leader_name, &error_text).await;
                        let _ = send_ws(&mut sender, &WsServerMessage::Message {
                            companion: leader_name.clone(),
                            content: error_text,
                            timestamp: chrono::Utc::now().timestamp(),
                        }).await;
                        // Don't run advisors if leader failed.
                        let mana_cost = ((total_tokens as f64) / 1000.0).ceil() as i64;
                        if mana_cost > 0 {
                            let db = state.manager.db().await;
                            let _ = system_tenants::economy::spend_mana(&db, &tenant.id.0, mana_cost, &tenant.tier);
                        }
                        continue;
                    }
                };

                // Build relationship context for advisors.
                let relationship_contexts: std::collections::HashMap<String, String> = {
                    let all_companions = tenant.companion_store.list_all().unwrap_or_default();
                    let mut ctx_map = std::collections::HashMap::new();
                    for advisor_name in &advisors {
                        let advisor_comp = all_companions.iter().find(|c| c.name == *advisor_name);
                        if let Some(advisor) = advisor_comp {
                            let mut lines = Vec::new();
                            for other in &all_companions {
                                if other.id == advisor.id {
                                    continue;
                                }
                                if let Ok(rel) = tenant.companion_store.get_or_seed_relationship(advisor, other) {
                                    lines.push(format!(
                                        "- **{}** ({:?} {:?}): {} — respect: {:.1}, affinity: {:.1}, rivalry: {:.1}",
                                        other.name, other.dere_type, other.archetype,
                                        rel.relationship_label(),
                                        rel.respect, rel.affinity, rel.rivalry,
                                    ));
                                }
                            }
                            if !lines.is_empty() {
                                ctx_map.insert(
                                    advisor_name.clone(),
                                    format!("\n## Your Relationships with Squad Members\n{}", lines.join("\n")),
                                );
                            }
                        }
                    }
                    ctx_map
                };

                // Route to relevant advisors (or skip all if no router).
                let routed_advisors: Vec<String> = if !advisors.is_empty() {
                    if let Some(ref mut router) = router {
                        let peer_configs: Vec<_> = advisors.iter().filter_map(|name| {
                            let agent_dir = tenant.data_dir.join("agents").join(name);
                            if let Ok(config) = system_core::load_agent_config(&agent_dir) {
                                Some(config)
                            } else {
                                tenant.companion_store.get_companion_by_name(name).ok().flatten().map(|comp| {
                                    PeerAgentConfig {
                                        name: name.clone(),
                                        prefix: "cmp".to_string(),
                                        model: None,
                                        role: AgentRole::Advisor,
                                        voice: AgentVoice::Vocal,
                                        execution_mode: ExecutionMode::Agent,
                                        max_workers: 1,
                                        max_turns: None,
                                        max_budget_usd: None,
                                        default_repo: None,
                                        expertise: archetype_expertise(&comp.archetype),
                                        capabilities: vec![],
                                        telegram_token_secret: None,
                                    }
                                })
                            }
                        }).collect();

                        let peer_refs: Vec<&PeerAgentConfig> = peer_configs.iter().collect();
                        match router.classify(&content, &peer_refs, 0).await {
                            Ok(decision) => {
                                debug!(
                                    category = %decision.category,
                                    advisors = ?decision.advisors,
                                    ms = decision.classify_ms,
                                    "router classified"
                                );
                                decision.advisors
                            }
                            Err(e) => {
                                debug!(error = %e, "router classification failed, skipping advisors");
                                vec![]
                            }
                        }
                    } else {
                        // No router available — leader-only.
                        vec![]
                    }
                } else {
                    vec![]
                };

                // Squad advisor loop — parallel.
                if !routed_advisors.is_empty() {
                    let advisor_futures: Vec<_> = routed_advisors.iter().map(|advisor_name| {
                        let advisor_name = advisor_name.clone();
                        let tenant = tenant.clone();
                        let state = state.clone();
                        let content = content.clone();
                        let leader_name = leader_name.clone();
                        let leader_response = leader_response_text.clone();
                        let rel_ctx = relationship_contexts.get(&advisor_name).cloned().unwrap_or_default();

                        async move {
                            let agent_dir = tenant.data_dir.join("agents").join(&advisor_name);
                            let project_dir = tenant.data_dir.join("projects/chat");
                            let mut identity = Identity::load(&agent_dir, Some(&project_dir))
                                .unwrap_or_default();

                            // Inject active project knowledge for advisors too.
                            enrich_identity_with_project(&mut identity, &tenant).await;

                            let result = execute_advisor_chat(
                                &identity, &content, &leader_name, &leader_response,
                                &tenant.tier.model, &state.platform, &rel_ctx,
                            ).await;

                            (advisor_name, result)
                        }
                    }).collect();

                    let results = futures::future::join_all(advisor_futures).await;

                    for (advisor_name, result) in results {
                        match result {
                            Ok((text, token_count)) => {
                                total_tokens += token_count;
                                debug!(advisor = %advisor_name, tokens = token_count, "advisor responded");

                                // Award bond XP to advisor (10 per interaction).
                                if let Ok(Some(mut comp)) = tenant.companion_store.get_companion_by_name(&advisor_name) {
                                    comp.add_bond_xp(10);
                                    let _ = tenant.companion_store.save_companion(&comp);
                                }

                                let _ = send_ws(&mut sender, &WsServerMessage::AdvisorMessage {
                                    companion: advisor_name,
                                    content: text,
                                    timestamp: chrono::Utc::now().timestamp(),
                                }).await;
                            }
                            Err(e) => {
                                // Advisor failures are non-fatal — skip silently.
                                debug!(advisor = %advisor_name, error = %e, "advisor chat failed, skipping");
                            }
                        }
                    }
                }

                // Spend total mana (leader + all advisors).
                let mana_cost = ((total_tokens as f64) / 1000.0).ceil() as i64;
                if mana_cost > 0 {
                    let db = state.manager.db().await;
                    let _ = system_tenants::economy::spend_mana(&db, &tenant.id.0, mana_cost, &tenant.tier);
                }
            }
        }
    }

    info!(tenant = %tenant.id, leader = %leader_name, "websocket disconnected");
}
