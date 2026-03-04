use anyhow::Result;
use std::path::Path;
use tracing::info;

use crate::tenant::TenantId;
use system_core::{AgentRole, AgentVoice, ExecutionMode, PeerAgentConfig};

/// Provision a new tenant's on-disk structure from templates.
pub fn provision_tenant(
    data_dir: &Path,
    template_dir: &Path,
    tenant_id: &TenantId,
    display_name: &str,
    tier: &str,
) -> Result<()> {
    // Create base directories.
    std::fs::create_dir_all(data_dir.join("agents/shared"))?;
    std::fs::create_dir_all(data_dir.join("projects/chat/.quests"))?;
    std::fs::create_dir_all(data_dir.join("projects/chat/.sigil"))?;

    // Write tenant metadata.
    let meta = crate::tenant::TenantMeta {
        id: tenant_id.0.clone(),
        display_name: display_name.to_string(),
        email: None,
        tier: tier.to_string(),
        created_at: chrono::Utc::now(),
        active_project: None,
        projects_source: None,
    };
    let meta_toml = toml::to_string_pretty(&meta)?;
    std::fs::write(data_dir.join("tenant.toml"), meta_toml)?;

    // Copy shared workflow from templates.
    let shared_workflow = template_dir.join("agents/shared/WORKFLOW.md");
    if shared_workflow.exists() {
        std::fs::copy(&shared_workflow, data_dir.join("agents/shared/WORKFLOW.md"))?;
    } else {
        // Minimal default.
        std::fs::write(
            data_dir.join("agents/shared/WORKFLOW.md"),
            "# Companion Workflow\n\nYou are a companion at gacha.agency. Be helpful, stay in character.\n",
        )?;
    }

    // Copy chat project template.
    let chat_template = template_dir.join("projects/chat/AGENTS.md");
    if chat_template.exists() {
        std::fs::copy(&chat_template, data_dir.join("projects/chat/AGENTS.md"))?;
    } else {
        std::fs::write(
            data_dir.join("projects/chat/AGENTS.md"),
            "# Chat Project\n\nYou are a conversational companion. Respond in character with personality.\n",
        )?;
    }

    // Write chat project KNOWLEDGE.md so agents know about available projects.
    let knowledge_template = template_dir.join("projects/chat/KNOWLEDGE.md");
    if knowledge_template.exists() {
        std::fs::copy(&knowledge_template, data_dir.join("projects/chat/KNOWLEDGE.md"))?;
    } else {
        std::fs::write(
            data_dir.join("projects/chat/KNOWLEDGE.md"),
            "# Chat Knowledge\n\n\
            ## Available Projects\n\
            - entity-legal: Legal entity formation and compliance\n\
            - algostaking: HFT trading system (Rust microservices)\n\
            - riftdecks-shop: TCG marketplace (Next.js)\n\
            - gacha-agency: Agent orchestration framework (Rust)\n\n\
            ## Your Role\n\
            You are a companion in the user's agency. Help them navigate their projects,\n\
            answer questions, and assist with tasks. If they mention a project, acknowledge\n\
            it exists and help them work on it.\n",
        )?;
    }

    info!(tenant = %tenant_id, dir = %data_dir.display(), "tenant provisioned");
    Ok(())
}

/// Derive a `PeerAgentConfig` from a companion's traits.
pub fn companion_to_agent_config(companion: &system_companions::Companion) -> PeerAgentConfig {
    PeerAgentConfig {
        name: companion.name.clone(),
        prefix: "cmp".to_string(),
        model: Some(companion.rarity.default_model().to_string()),
        role: AgentRole::Advisor,
        voice: AgentVoice::Vocal,
        execution_mode: ExecutionMode::Agent,
        max_workers: 1,
        max_turns: None,
        max_budget_usd: Some(companion.rarity.default_budget_usd()),
        default_repo: None,
        expertise: companion.archetype.default_expertise(),
        capabilities: vec![],
        telegram_token_secret: None,
    }
}

/// Materialize a companion as a full agent on disk (synchronous — fast, no LLM).
/// Writes fallback SOUL.md + IDENTITY.md immediately.
pub fn materialize_companion(
    data_dir: &Path,
    template_dir: &Path,
    companion: &system_companions::Companion,
) -> Result<std::path::PathBuf> {
    let agent_dir = data_dir.join("agents").join(&companion.name);
    std::fs::create_dir_all(agent_dir.join(".sigil"))?;

    // SOUL.md from companion personality + archetype template.
    let archetype_slug = format!("{:?}", companion.archetype).to_lowercase();
    let template_path = template_dir.join("agents/archetypes").join(format!("{archetype_slug}.md"));
    let archetype_template = if template_path.exists() {
        std::fs::read_to_string(&template_path).unwrap_or_default()
    } else {
        String::new()
    };

    let soul = format!(
        "# {}\n\n{}\n\n---\n\n{}",
        companion.name,
        companion.system_prompt_fragment(),
        archetype_template,
    );
    std::fs::write(agent_dir.join("SOUL.md"), &soul)?;

    // IDENTITY.md
    let full_name = companion.full_name();
    let identity = format!(
        "# Identity: {full_name}\n\nRarity: {} | Archetype: {} | Aesthetic: {} | Region: {}\nBond Level: {}\n",
        companion.rarity,
        companion.archetype.title(),
        companion.aesthetic,
        companion.region,
        companion.bond_level,
    );
    std::fs::write(agent_dir.join("IDENTITY.md"), &identity)?;

    // PREFERENCES.md (empty -- filled by interactions)
    std::fs::write(agent_dir.join("PREFERENCES.md"), "# Preferences\n\n*No preferences recorded yet.*\n")?;

    // MEMORY.md (empty)
    std::fs::write(agent_dir.join("MEMORY.md"), "# Memory\n\n*No memories recorded yet.*\n")?;

    // Emotional state (new)
    let emo = system_orchestrator::EmotionalState::new(&companion.name);
    emo.save(&system_orchestrator::EmotionalState::path_for_agent(&agent_dir))?;

    // agent.toml — execution config derived from companion traits.
    let agent_config = companion_to_agent_config(companion);
    let agent_toml = toml::to_string_pretty(&agent_config)
        .unwrap_or_else(|_| format!("name = \"{}\"\n", companion.name));
    std::fs::write(agent_dir.join("agent.toml"), agent_toml)?;

    info!(companion = %companion.name, dir = %agent_dir.display(), "companion materialized (sync)");
    Ok(agent_dir)
}

/// Async portrait generation — calls image generation API to produce portrait.png.
/// Writes the image to the companion's agent directory and updates portrait_status.
/// Fires a `TenantEvent::PortraitReady` on completion or failure.
pub async fn materialize_companion_portrait(
    data_dir: &Path,
    companion: &system_companions::Companion,
    platform: &crate::config::PlatformConfig,
    companion_store: &system_companions::CompanionStore,
    event_tx: &tokio::sync::broadcast::Sender<crate::events::TenantEvent>,
) -> Result<()> {
    let agent_dir = data_dir.join("agents").join(&companion.name);
    std::fs::create_dir_all(&agent_dir)?;

    // Update status to generating.
    let _ = companion_store.update_companion(&companion.id, |c| {
        c.portrait_status = system_companions::PortraitStatus::Generating;
    });

    // Build provider — use OpenRouter. Model is determined by portrait_gen::DEFAULT_IMAGE_MODEL.
    let provider = if let Some(ref openrouter) = platform.providers.openrouter {
        system_providers::OpenRouterProvider::new(
            openrouter.api_key.clone(),
            "".to_string(),
        )
    } else {
        let _ = companion_store.update_companion(&companion.id, |c| {
            c.portrait_status = system_companions::PortraitStatus::Failed;
        });
        let _ = event_tx.send(crate::events::TenantEvent::PortraitReady {
            companion_name: companion.name.clone(),
            success: false,
        });
        anyhow::bail!("no OpenRouter provider configured for portrait generation");
    };

    let model = ""; // Uses DEFAULT_IMAGE_MODEL (Gemini Flash) in portrait_gen

    match crate::portrait_gen::generate_portrait(companion, &provider, model).await {
        Ok(bytes) => {
            // Write portrait image.
            std::fs::write(agent_dir.join("portrait.png"), &bytes)?;

            // Update status to complete.
            companion_store.update_companion(&companion.id, |c| {
                c.portrait_status = system_companions::PortraitStatus::Complete;
            })?;

            let _ = event_tx.send(crate::events::TenantEvent::PortraitReady {
                companion_name: companion.name.clone(),
                success: true,
            });

            info!(companion = %companion.name, bytes = bytes.len(), "portrait written (async)");
            Ok(())
        }
        Err(e) => {
            let _ = companion_store.update_companion(&companion.id, |c| {
                c.portrait_status = system_companions::PortraitStatus::Failed;
            });
            let _ = event_tx.send(crate::events::TenantEvent::PortraitReady {
                companion_name: companion.name.clone(),
                success: false,
            });
            Err(e)
        }
    }
}

/// Async persona generation — calls LLM to generate PERSONA.md.
/// Identity::load() prefers PERSONA.md over SOUL.md, so this automatically
/// takes precedence once written.
/// Fires a `TenantEvent::PersonaReady` on completion or failure.
pub async fn materialize_companion_persona(
    data_dir: &Path,
    companion: &system_companions::Companion,
    platform: &crate::config::PlatformConfig,
    parents: Option<(system_companions::Companion, system_companions::Companion)>,
    event_tx: &tokio::sync::broadcast::Sender<crate::events::TenantEvent>,
    companion_store: &system_companions::CompanionStore,
) -> Result<()> {
    use system_core::traits::Provider;

    let agent_dir = data_dir.join("agents").join(&companion.name);

    // Update persona status to generating.
    let _ = companion_store.update_companion(&companion.id, |c| {
        c.persona_status = system_companions::PersonaStatus::Generating;
    });

    // Build provider — use OpenRouter with MiniMax M2.5 (cheap, high quality).
    let (provider, model): (Box<dyn Provider>, String) =
        if let Some(ref openrouter) = platform.providers.openrouter {
            (
                Box::new(system_providers::OpenRouterProvider::new(
                    openrouter.api_key.clone(),
                    "minimax/minimax-m2.5".to_string(),
                )),
                "minimax/minimax-m2.5".to_string(),
            )
        } else if let Some(ref anthropic) = platform.providers.anthropic {
            (
                Box::new(system_providers::AnthropicProvider::new(
                    anthropic.api_key.clone(),
                    "claude-haiku-4-5".to_string(),
                )),
                "claude-haiku-4-5".to_string(),
            )
        } else {
            let _ = companion_store.update_companion(&companion.id, |c| {
                c.persona_status = system_companions::PersonaStatus::Failed;
            });
            let _ = event_tx.send(crate::events::TenantEvent::PersonaReady {
                companion_name: companion.name.clone(),
                success: false,
            });
            anyhow::bail!("no provider configured for persona generation");
        };

    let parent_refs = parents.as_ref().map(|(a, b)| (a, b));
    match crate::persona_gen::generate_persona(companion, provider.as_ref(), &model, parent_refs).await {
        Ok(persona_text) => {
            // Write PERSONA.md — this takes precedence over SOUL.md.
            std::fs::create_dir_all(&agent_dir)?;
            std::fs::write(agent_dir.join("PERSONA.md"), &persona_text)?;

            // Update persona status to complete.
            let _ = companion_store.update_companion(&companion.id, |c| {
                c.persona_status = system_companions::PersonaStatus::Complete;
            });

            let _ = event_tx.send(crate::events::TenantEvent::PersonaReady {
                companion_name: companion.name.clone(),
                success: true,
            });

            info!(companion = %companion.name, "persona written (async)");
            Ok(())
        }
        Err(e) => {
            let _ = companion_store.update_companion(&companion.id, |c| {
                c.persona_status = system_companions::PersonaStatus::Failed;
            });
            let _ = event_tx.send(crate::events::TenantEvent::PersonaReady {
                companion_name: companion.name.clone(),
                success: false,
            });
            Err(e)
        }
    }
}
