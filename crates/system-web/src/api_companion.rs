use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;

use crate::AppState;
use crate::auth::AuthTenant;
use crate::error::AppError;
use crate::types::*;

pub async fn list_companions(
    AuthTenant(tenant): AuthTenant,
) -> Result<Json<Vec<CompanionInfo>>, AppError> {
    let companions = tenant.companion_store.list_all()?;
    let infos: Vec<CompanionInfo> = companions.iter().map(CompanionInfo::from_companion).collect();
    Ok(Json(infos))
}

pub async fn get_companion(
    AuthTenant(tenant): AuthTenant,
    Path(name): Path<String>,
) -> Result<Json<CompanionInfo>, AppError> {
    let companion = tenant.companion_store.get_companion_by_name(&name)?
        .ok_or_else(|| AppError::NotFound("companion not found".to_string()))?;
    Ok(Json(CompanionInfo::from_companion(&companion)))
}

pub async fn set_familiar(
    AuthTenant(tenant): AuthTenant,
    Json(req): Json<SetFamiliarRequest>,
) -> Result<Json<CompanionInfo>, AppError> {
    let companion = tenant.companion_store.get_companion_by_name(&req.name)?
        .ok_or_else(|| AppError::NotFound("companion not found".to_string()))?;

    tenant.companion_store.set_familiar(&companion.id)?;

    let mut updated = companion.clone();
    updated.is_familiar = true;
    Ok(Json(CompanionInfo::from_companion(&updated)))
}

pub async fn get_familiar(
    AuthTenant(tenant): AuthTenant,
) -> Result<Json<CompanionInfo>, AppError> {
    let familiar = tenant.companion_store.get_familiar()?
        .ok_or_else(|| AppError::NotFound("no familiar set".to_string()))?;
    Ok(Json(CompanionInfo::from_companion(&familiar)))
}

#[derive(serde::Deserialize)]
pub struct PortraitQuery {
    pub token: Option<String>,
}

/// Serve portrait PNG. Accepts auth via `?token=` query param
/// (needed because `<img src>` can't send Authorization headers).
pub async fn get_portrait(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
    Query(query): Query<PortraitQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let token = query
        .token
        .ok_or((StatusCode::UNAUTHORIZED, "missing token query param".to_string()))?;

    let tenant = state
        .manager
        .resolve_by_session(&token)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid token".to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "tenant not found".to_string()))?;

    let portrait_path = tenant.data_dir.join("agents").join(&name).join("portrait.png");

    if !portrait_path.exists() {
        return Err((StatusCode::NOT_FOUND, "portrait not found".to_string()));
    }

    let bytes = std::fs::read(&portrait_path)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        [(axum::http::header::CONTENT_TYPE, "image/png"),
         (axum::http::header::CACHE_CONTROL, "public, max-age=86400")],
        bytes,
    ))
}

/// Backfill portraits for all companions missing one.
pub async fn backfill_portraits(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let companions = tenant.companion_store.list_all()?;

    let mut spawned = 0u32;
    for companion in &companions {
        let portrait_path = tenant.data_dir.join("agents").join(&companion.name).join("portrait.png");
        if portrait_path.exists() {
            continue;
        }

        crate::api_gacha::spawn_portrait_gen(
            tenant.data_dir.clone(), companion.clone(), state.platform.clone(),
            tenant.companion_store.clone(), tenant.event_tx.clone(),
        );
        spawned += 1;
    }

    Ok(Json(serde_json::json!({ "spawned": spawned })))
}

pub async fn get_relationships(
    AuthTenant(tenant): AuthTenant,
    Path(name): Path<String>,
) -> Result<Json<Vec<RelationshipInfo>>, AppError> {
    let companions = tenant.companion_store.list_all()?;

    let target = companions.iter()
        .find(|c| c.name == name)
        .ok_or_else(|| AppError::NotFound("companion not found".to_string()))?;

    // Build relationships with all other companions (lazy seed).
    let mut relationships = Vec::new();
    for other in &companions {
        if other.id == target.id {
            continue;
        }
        let rel = tenant.companion_store.get_or_seed_relationship(target, other)?;

        relationships.push(RelationshipInfo {
            companion_a: rel.agent_a.clone(),
            companion_b: rel.agent_b.clone(),
            respect: rel.respect,
            affinity: rel.affinity,
            trust: rel.trust,
            rivalry: rel.rivalry,
            synergy: rel.synergy,
            label: rel.relationship_label().to_string(),
            compatibility: rel.overall_compatibility(),
        });
    }

    Ok(Json(relationships))
}
