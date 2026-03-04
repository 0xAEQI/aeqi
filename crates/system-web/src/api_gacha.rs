use axum::extract::{Json, State};
use std::path::PathBuf;
use std::sync::Arc;

use crate::AppState;
use crate::auth::AuthTenant;
use crate::error::AppError;
use crate::types::*;
use system_companions::{Companion, CompanionStore, GachaEngine};
use system_tenants::config::PlatformConfig;
use system_tenants::provision;

/// Spawn async persona generation in background.
pub(crate) fn spawn_persona_gen(
    data_dir: PathBuf,
    companion: Companion,
    platform: PlatformConfig,
    event_tx: tokio::sync::broadcast::Sender<system_tenants::TenantEvent>,
    store: Arc<CompanionStore>,
    parents: Option<(Companion, Companion)>,
) {
    tokio::spawn(async move {
        if let Err(e) = provision::materialize_companion_persona(
            &data_dir, &companion, &platform, parents,
            &event_tx, &store,
        ).await {
            tracing::warn!(
                companion = %companion.name,
                error = %e,
                "async persona generation failed"
            );
        }
    });
}

/// Spawn async portrait generation in background with retry (up to 3 attempts).
pub(crate) fn spawn_portrait_gen(
    data_dir: PathBuf,
    companion: Companion,
    platform: PlatformConfig,
    store: Arc<CompanionStore>,
    event_tx: tokio::sync::broadcast::Sender<system_tenants::TenantEvent>,
) {
    tokio::spawn(async move {
        let max_attempts = 3;
        for attempt in 1..=max_attempts {
            match provision::materialize_companion_portrait(
                &data_dir, &companion, &platform, &store, &event_tx,
            ).await {
                Ok(()) => return,
                Err(e) => {
                    if attempt == max_attempts {
                        tracing::warn!(
                            companion = %companion.name,
                            error = %e,
                            attempts = max_attempts,
                            "portrait generation failed after all retries"
                        );
                    } else {
                        tracing::info!(
                            companion = %companion.name,
                            error = %e,
                            attempt = attempt,
                            "portrait generation failed, retrying"
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempt as u32))).await;
                    }
                }
            }
        }
    });
}

pub async fn pull(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
) -> Result<Json<PullResponse>, AppError> {
    // Check economy: spend 1 summon.
    {
        let db = state.manager.db().await;
        let can_spend = system_tenants::economy::spend_summons(&db, &tenant.id.0, 1, &tenant.tier)?;
        if !can_spend {
            return Err(AppError::PaymentRequired("insufficient summons".to_string()));
        }
    }

    // Check companion limit.
    let stats = tenant.companion_store.collection_stats()?;
    if stats.total_companions >= tenant.tier.max_companions {
        return Err(AppError::Forbidden("companion limit reached".to_string()));
    }

    // Load pity state and pull.
    let mut pity = tenant.companion_store.load_pity()?;
    let engine = GachaEngine::default();
    let companion = engine.pull(&mut pity);
    let pity_count = pity.pulls_since_s_or_above;

    // Check if new (not already in collection).
    let existing = tenant.companion_store.get_companion(&companion.id)?;
    let is_new = existing.is_none();

    // Save companion + pity.
    tenant.companion_store.save_companion(&companion)?;
    tenant.companion_store.save_pity(&pity)?;
    tenant.companion_store.record_pull(&companion)?;

    // Materialize on disk (sync — fast).
    if is_new {
        let _ = provision::materialize_companion(
            &tenant.data_dir, &state.platform.template_dir(), &companion,
        );
        spawn_persona_gen(
            tenant.data_dir.clone(), companion.clone(), state.platform.clone(),
            tenant.event_tx.clone(), tenant.companion_store.clone(), None,
        );
        spawn_portrait_gen(
            tenant.data_dir.clone(), companion.clone(), state.platform.clone(),
            tenant.companion_store.clone(), tenant.event_tx.clone(),
        );
    }

    Ok(Json(PullResponse {
        companion: CompanionInfo::from_companion(&companion),
        is_new,
        pity_count,
    }))
}

pub async fn pull10(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Pull10Response>, AppError> {
    // Check economy: spend 10 summons.
    {
        let db = state.manager.db().await;
        let can_spend = system_tenants::economy::spend_summons(&db, &tenant.id.0, 10, &tenant.tier)?;
        if !can_spend {
            return Err(AppError::PaymentRequired("insufficient summons (need 10)".to_string()));
        }
    }

    let mut results = Vec::with_capacity(10);
    let mut pity = tenant.companion_store.load_pity()?;
    let engine = GachaEngine::default();

    for _ in 0..10 {
        let companion = engine.pull(&mut pity);
        let existing = tenant.companion_store.get_companion(&companion.id)?;
        let is_new = existing.is_none();

        tenant.companion_store.save_companion(&companion)?;
        tenant.companion_store.record_pull(&companion)?;

        if is_new {
            let _ = provision::materialize_companion(
                &tenant.data_dir, &state.platform.template_dir(), &companion,
            );
            spawn_persona_gen(
                tenant.data_dir.clone(), companion.clone(), state.platform.clone(),
                tenant.event_tx.clone(), tenant.companion_store.clone(), None,
            );
            spawn_portrait_gen(
                tenant.data_dir.clone(), companion.clone(), state.platform.clone(),
                tenant.companion_store.clone(), tenant.event_tx.clone(),
            );
        }

        results.push(PullResponse {
            companion: CompanionInfo::from_companion(&companion),
            is_new,
            pity_count: pity.pulls_since_s_or_above,
        });
    }

    tenant.companion_store.save_pity(&pity)?;

    Ok(Json(Pull10Response { results }))
}

pub async fn fuse(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
    Json(req): Json<FuseRequest>,
) -> Result<Json<FuseResponse>, AppError> {
    if req.names.len() != 4 {
        return Err(AppError::BadRequest("fusion requires exactly 4 companions".into()));
    }

    // Load all 4 companions
    let mut companions = Vec::new();
    for name in &req.names {
        let c = tenant.companion_store.get_companion_by_name(name)?
            .ok_or_else(|| AppError::NotFound(format!("companion '{}' not found", name)))?;
        companions.push(c);
    }

    let refs: Vec<&system_companions::Companion> = companions.iter().collect();
    let result = system_companions::fuse_multi(&refs)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Save the new fused companion.
    tenant.companion_store.save_companion(&result)?;

    // Record fusion using first two as "parents" for lineage tracking.
    tenant.companion_store.record_fusion(&companions[0], &companions[1], &result)?;

    // Clone parents before removal.
    let parent_a = companions[0].clone();
    let parent_b = companions[1].clone();

    // Remove all 4 source companions (consumed by fusion).
    for c in &companions {
        tenant.companion_store.remove_companion(&c.id)?;
    }

    // Materialize fused companion on disk.
    let _ = provision::materialize_companion(
        &tenant.data_dir, &state.platform.template_dir(), &result,
    );

    // Spawn async persona generation with fusion lineage context.
    spawn_persona_gen(
        tenant.data_dir.clone(), result.clone(), state.platform.clone(),
        tenant.event_tx.clone(), tenant.companion_store.clone(),
        Some((parent_a, parent_b)),
    );
    spawn_portrait_gen(
        tenant.data_dir.clone(), result.clone(), state.platform.clone(),
        tenant.companion_store.clone(), tenant.event_tx.clone(),
    );

    Ok(Json(FuseResponse {
        companion: CompanionInfo::from_companion(&result),
    }))
}
