use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

/// Client for the AEQI daemon's Unix socket IPC.
/// Protocol: one JSON line in → one JSON line out.
#[derive(Debug, Clone)]
pub struct IpcClient {
    socket_path: PathBuf,
}

impl IpcClient {
    pub fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    /// Derive socket path from a data directory.
    pub fn from_data_dir(data_dir: &Path) -> Self {
        Self::new(data_dir.join("rm.sock"))
    }

    /// Send a JSON request and get a JSON response.
    pub async fn request(&self, request: &serde_json::Value) -> Result<serde_json::Value> {
        if !self.socket_path.exists() {
            anyhow::bail!(
                "IPC socket not found: {}. Is the daemon running?",
                self.socket_path.display()
            );
        }

        let stream = tokio::net::UnixStream::connect(&self.socket_path)
            .await
            .with_context(|| {
                format!(
                    "failed to connect to IPC socket: {}",
                    self.socket_path.display()
                )
            })?;

        let (reader, mut writer) = stream.into_split();
        let mut req_bytes = serde_json::to_vec(request)?;
        req_bytes.push(b'\n');
        writer.write_all(&req_bytes).await?;

        let mut lines = BufReader::new(reader).lines();
        let Some(line) = lines.next_line().await? else {
            anyhow::bail!("IPC socket closed without response");
        };

        let response: serde_json::Value = serde_json::from_str(&line)?;
        Ok(response)
    }

    /// Convenience: send a simple command with no extra params.
    pub async fn cmd(&self, cmd: &str) -> Result<serde_json::Value> {
        self.request(&serde_json::json!({"cmd": cmd})).await
    }

    /// Convenience: send a command with params merged in.
    pub async fn cmd_with(
        &self,
        cmd: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let mut req = params;
        req["cmd"] = serde_json::Value::String(cmd.to_string());
        self.request(&req).await
    }
}
