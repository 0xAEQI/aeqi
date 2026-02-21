use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sigil_core::traits::{Channel, IncomingMessage, OutgoingMessage};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

const TELEGRAM_API: &str = "https://api.telegram.org";

/// Telegram Bot API channel.
pub struct TelegramChannel {
    client: Client,
    token: String,
    /// Chat IDs allowed to interact (empty = all).
    allowed_chats: Vec<i64>,
    shutdown: tokio::sync::watch::Sender<bool>,
    shutdown_rx: tokio::sync::watch::Receiver<bool>,
}

impl TelegramChannel {
    pub fn new(token: String, allowed_chats: Vec<i64>) -> Self {
        let (shutdown, shutdown_rx) = tokio::sync::watch::channel(false);
        Self {
            client: Client::new(),
            token,
            allowed_chats,
            shutdown,
            shutdown_rx,
        }
    }

    fn api_url(&self, method: &str) -> String {
        format!("{}/bot{}/{}", TELEGRAM_API, self.token, method)
    }
}

#[derive(Deserialize)]
struct TelegramResponse<T> {
    ok: bool,
    result: Option<T>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct TelegramUpdate {
    update_id: i64,
    message: Option<TelegramMessage>,
}

#[derive(Deserialize)]
struct TelegramMessage {
    message_id: i64,
    chat: TelegramChat,
    from: Option<TelegramUser>,
    text: Option<String>,
}

#[derive(Deserialize)]
struct TelegramChat {
    id: i64,
}

#[derive(Deserialize)]
struct TelegramUser {
    _id: i64,
    first_name: String,
    username: Option<String>,
}

#[derive(Serialize)]
struct SendMessage {
    chat_id: i64,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
}

#[async_trait]
impl Channel for TelegramChannel {
    async fn start(&self) -> Result<mpsc::Receiver<IncomingMessage>> {
        let (tx, rx) = mpsc::channel(100);
        let client = self.client.clone();
        let token = self.token.clone();
        let allowed_chats = self.allowed_chats.clone();
        let mut shutdown_rx = self.shutdown_rx.clone();

        tokio::spawn(async move {
            let mut offset: Option<i64> = None;
            info!("Telegram polling started");

            loop {
                // Check shutdown.
                if *shutdown_rx.borrow() {
                    break;
                }

                let url = format!("{}/bot{}/getUpdates", TELEGRAM_API, token);
                let mut params = serde_json::json!({ "timeout": 30 });
                if let Some(off) = offset {
                    params["offset"] = serde_json::json!(off);
                }

                let result = tokio::select! {
                    _ = shutdown_rx.changed() => break,
                    r = client.post(&url).json(&params).send() => r,
                };

                match result {
                    Ok(response) => {
                        if let Ok(body) = response.json::<TelegramResponse<Vec<TelegramUpdate>>>().await {
                            if body.ok {
                                for update in body.result.unwrap_or_default() {
                                    offset = Some(update.update_id + 1);

                                    if let Some(msg) = update.message {
                                        // Check allowed chats.
                                        if !allowed_chats.is_empty() && !allowed_chats.contains(&msg.chat.id) {
                                            debug!(chat_id = msg.chat.id, "ignoring message from unauthorized chat");
                                            continue;
                                        }

                                        if let Some(text) = msg.text {
                                            let sender = msg.from.map(|u| {
                                                u.username.unwrap_or(u.first_name)
                                            }).unwrap_or_else(|| "unknown".to_string());

                                            let incoming = IncomingMessage {
                                                channel: "telegram".to_string(),
                                                sender,
                                                text,
                                                metadata: serde_json::json!({
                                                    "chat_id": msg.chat.id,
                                                    "message_id": msg.message_id,
                                                }),
                                            };

                                            if tx.send(incoming).await.is_err() {
                                                break; // Receiver dropped.
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "Telegram polling error");
                        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                    }
                }
            }
            info!("Telegram polling stopped");
        });

        Ok(rx)
    }

    async fn send(&self, message: OutgoingMessage) -> Result<()> {
        let chat_id = message.metadata.get("chat_id")
            .and_then(|v| v.as_i64())
            .context("missing chat_id in metadata")?;

        let send_msg = SendMessage {
            chat_id,
            text: message.text,
            parse_mode: Some("Markdown".to_string()),
        };

        let response = self.client
            .post(&self.api_url("sendMessage"))
            .json(&send_msg)
            .send()
            .await
            .context("failed to send Telegram message")?;

        let body: TelegramResponse<serde_json::Value> = response.json().await?;
        if !body.ok {
            anyhow::bail!("Telegram sendMessage failed: {}", body.description.unwrap_or_default());
        }

        Ok(())
    }

    fn name(&self) -> &str { "telegram" }

    async fn stop(&self) -> Result<()> {
        let _ = self.shutdown.send(true);
        Ok(())
    }
}
