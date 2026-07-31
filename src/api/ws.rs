use crate::models::{LogMessage, TrafficMessage};
use anyhow::{Context, Result};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn stream_traffic(
    base_url: &str,
    secret: Option<&str>,
    tx: mpsc::Sender<TrafficMessage>,
) -> Result<()> {
    let ws_url = convert_to_ws_url(base_url, "/traffic", secret);
    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .context("Failed to connect to Mihomo traffic WebSocket")?;

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(traffic) = serde_json::from_str::<TrafficMessage>(&text) {
                    if tx.send(traffic).await.is_err() {
                        break; // Receiver dropped
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    Ok(())
}

pub async fn stream_logs(
    base_url: &str,
    secret: Option<&str>,
    log_level: &str,
    tx: mpsc::Sender<LogMessage>,
) -> Result<()> {
    let path = format!("/logs?level={}", log_level);
    let ws_url = convert_to_ws_url(base_url, &path, secret);
    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .context("Failed to connect to Mihomo logs WebSocket")?;

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(log_entry) = serde_json::from_str::<LogMessage>(&text) {
                    if tx.send(log_entry).await.is_err() {
                        break; // Receiver dropped
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    Ok(())
}

fn convert_to_ws_url(base_url: &str, path: &str, secret: Option<&str>) -> String {
    let mut ws_base = base_url
        .replace("http://", "ws://")
        .replace("https://", "wss://");

    if ws_base.ends_with('/') {
        ws_base.pop();
    }

    let mut full_path = path.to_string();
    if let Some(sec) = secret {
        if !sec.trim().is_empty() {
            let separator = if full_path.contains('?') { "&" } else { "?" };
            full_path = format!("{}{}{}{}", full_path, separator, "token=", urlencoding::encode(sec));
        }
    }

    format!("{}{}", ws_base, full_path)
}
