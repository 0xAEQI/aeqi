/// Events broadcast to all active WebSocket connections for a tenant.
#[derive(Debug, Clone)]
pub enum TenantEvent {
    PortraitReady { companion_name: String, success: bool },
    PersonaReady { companion_name: String, success: bool },
}
