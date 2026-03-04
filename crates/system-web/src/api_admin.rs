use axum::extract::State;
use axum::extract::Json;
use std::sync::Arc;

use crate::AppState;
use crate::auth::AuthTenant;
use crate::error::AppError;
use crate::types::*;

fn check_admin(tenant_id: &str, state: &AppState) -> Result<(), AppError> {
    if state.platform.platform.admin_tenant_ids.contains(&tenant_id.to_string()) {
        Ok(())
    } else {
        Err(AppError::Forbidden("admin access required".to_string()))
    }
}

pub async fn list_users(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    check_admin(&tenant.id.0, &state)?;

    let count = state.manager.active_count().await;
    Ok(Json(serde_json::json!({
        "active_count": count,
    })))
}

pub async fn stats(
    AuthTenant(tenant): AuthTenant,
    State(state): State<Arc<AppState>>,
) -> Result<Json<AdminStatsResponse>, AppError> {
    check_admin(&tenant.id.0, &state)?;

    let active = state.manager.active_count().await;
    let cost = state.manager.global_cost().await;

    Ok(Json(AdminStatsResponse {
        active_tenants: active,
        global_cost_today_usd: cost,
    }))
}
