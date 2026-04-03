//! Streaming chat WebSocket endpoint.
//!
//! Accepts a user message, submits it to the daemon via chat_full, then
//! streams ChatStreamEvents back in real-time until the agent completes.
//!
//! Protocol:
//! - Client sends: `{"message": "...", "project": "...", "agent": "..."}`
//! - Server sends: sequence of ChatStreamEvent JSON objects
//! - Server sends final `{"type": "Complete", ...}` and closes

use axum::{
    extract::{Query, State, WebSocketUpgrade},
    response::Response,
};
use serde::Deserialize;
use tracing::info;

use crate::auth;
use crate::server::AppState;

#[derive(Deserialize, Default)]
pub struct ChatWsQuery {
    token: Option<String>,
}

pub async fn handler(
    State(state): State<AppState>,
    Query(q): Query<ChatWsQuery>,
    ws: WebSocketUpgrade,
) -> Response {
    let secret = state.auth_secret.as_deref().unwrap_or("");
    if !secret.is_empty() {
        let token = q.token.as_deref().unwrap_or("");
        if auth::validate_token(token, secret).is_err() {
            return axum::response::IntoResponse::into_response((
                axum::http::StatusCode::UNAUTHORIZED,
                "invalid or missing token",
            ));
        }
    }

    ws.on_upgrade(move |socket| handle_chat_socket(socket, state))
}

async fn handle_chat_socket(mut socket: axum::extract::ws::WebSocket, state: AppState) {
    use axum::extract::ws::Message;

    info!("Chat WebSocket client connected");

    // Wait for the client's first message (the chat request).
    let request = match socket.recv().await {
        Some(Ok(Message::Text(text))) => match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(v) => v,
            Err(e) => {
                let _ = socket
                        .send(Message::Text(
                            serde_json::json!({"type": "Error", "message": e.to_string(), "recoverable": false}).to_string().into(),
                        ))
                        .await;
                return;
            }
        },
        _ => return,
    };

    let message = request
        .get("message")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if message.is_empty() {
        let _ = socket
            .send(Message::Text(
                serde_json::json!({"type": "Error", "message": "empty message", "recoverable": false}).to_string().into(),
            ))
            .await;
        return;
    }

    // Submit to daemon via session_send for direct LLM call.
    let agent = request.get("agent").and_then(|v| v.as_str()).unwrap_or("");

    let session_req = serde_json::json!({
        "message": message,
        "agent": agent,
    });

    let response = match state.ipc.cmd_with("session_send", session_req).await {
        Ok(resp) => resp,
        Err(e) => {
            let _ = socket
                .send(Message::Text(
                    serde_json::json!({"type": "Error", "message": e.to_string(), "recoverable": false}).to_string().into(),
                ))
                .await;
            return;
        }
    };

    if let Some(text) = response.get("text").and_then(|v| v.as_str()) {
        let _ = socket
            .send(Message::Text(
                serde_json::json!({"type": "TextDelta", "text": text})
                    .to_string()
                    .into(),
            ))
            .await;
        let _ = socket
            .send(Message::Text(
                serde_json::json!({"type": "Complete", "stop_reason": "end_turn", "total_prompt_tokens": 0, "total_completion_tokens": 0, "iterations": 0, "cost_usd": 0.0}).to_string().into(),
            ))
            .await;
    } else if let Some(error) = response.get("error").and_then(|v| v.as_str()) {
        let _ = socket
            .send(Message::Text(
                serde_json::json!({"type": "Error", "message": error, "recoverable": false})
                    .to_string()
                    .into(),
            ))
            .await;
    }

    info!("chat session_send completed");
}
