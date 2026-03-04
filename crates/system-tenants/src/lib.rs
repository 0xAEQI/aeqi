//! Multi-tenant SaaS infrastructure for Sigil.
//!
//! Manages tenant lifecycle ([`TenantManager`]), JWT authentication with email+password
//! and TOTP ([`auth`]), tiered billing via Stripe ([`stripe`]), summon/mana economy
//! ([`economy`]), companion materialization ([`provision`]), and async persona/portrait
//! generation. Each [`Tenant`] has isolated storage, companion collection, and project registry.

pub mod config;
pub mod events;
pub mod tenant;
pub mod manager;
pub mod provision;
pub mod persona_gen;
pub mod portrait_gen;
pub mod storage;
pub mod auth;
pub mod email;
pub mod economy;
pub mod stripe;
pub mod project_meta;

pub use config::{PlatformConfig, TierConfig};
pub use events::TenantEvent;
pub use tenant::{Tenant, TenantId};
pub use manager::TenantManager;
pub use auth::SessionToken;
pub use project_meta::TenantProjectMeta;
