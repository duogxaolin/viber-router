use deadpool_redis::Pool;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::Settings;

fn default_settings() -> Settings {
    Settings {
        telegram_bot_token: None,
        telegram_chat_ids: vec![],
        alert_status_codes: vec![500, 502, 503],
        alert_cooldown_mins: 5,
    }
}

/// Escape special characters for Telegram MarkdownV2.
fn escape_md(s: &str) -> String {
    let special = ['_', '*', '[', ']', '(', ')', '~', '`', '>', '#', '+', '-', '=', '|', '{', '}', '.', '!'];
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        if special.contains(&c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

pub struct AlertContext {
    pub db: PgPool,
    pub redis: Pool,
    pub http_client: reqwest::Client,
    pub server_id: Uuid,
    pub server_name: String,
    pub group_name: String,
    pub status_code: u16,
    pub latency_ms: i32,
}

pub async fn maybe_alert(ctx: AlertContext) {
    // Load settings fresh from DB
    let settings = match sqlx::query_as::<_, Settings>(
        "SELECT telegram_bot_token, telegram_chat_ids, alert_status_codes, alert_cooldown_mins \
         FROM settings WHERE id = 1",
    )
    .fetch_optional(&ctx.db)
    .await
    {
        Ok(row) => row.unwrap_or_else(default_settings),
        Err(e) => {
            tracing::warn!("telegram_notifier: failed to load settings: {}", e);
            return;
        }
    };

    // Skip if no token configured
    let token = match settings.telegram_bot_token {
        Some(t) => t,
        None => return,
    };

    // Skip if status code not in alert list
    if !settings.alert_status_codes.contains(&(ctx.status_code as i32)) {
        return;
    }

    // Check Redis cooldown key
    let cooldown_key = format!("tg:cooldown:{}:{}", ctx.server_id, ctx.status_code);
    let ttl_secs = settings.alert_cooldown_mins * 60;

    let should_send = match check_and_set_cooldown(&ctx.redis, &cooldown_key, ttl_secs).await {
        Ok(acquired) => acquired,
        Err(e) => {
            tracing::warn!("telegram_notifier: Redis cooldown check failed (fail open): {}", e);
            true // fail open
        }
    };

    if !should_send {
        return;
    }

    // Build message
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let text = format!(
        "🚨 *Upstream Error*\n*Server:* {}\n*Group:* {}\n*Status:* {}\n*Latency:* {}ms\n*Time:* {}",
        escape_md(&ctx.server_name),
        escape_md(&ctx.group_name),
        escape_md(&ctx.status_code.to_string()),
        escape_md(&ctx.latency_ms.to_string()),
        escape_md(&now.to_string()),
    );

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    // Send to all chat IDs in parallel
    let futures: Vec<_> = settings
        .telegram_chat_ids
        .iter()
        .map(|chat_id| {
            let client = ctx.http_client.clone();
            let url = url.clone();
            let text = text.clone();
            let chat_id = chat_id.clone();
            async move {
                let result = client
                    .post(&url)
                    .json(&serde_json::json!({
                        "chat_id": chat_id,
                        "text": text,
                        "parse_mode": "MarkdownV2"
                    }))
                    .send()
                    .await;
                (chat_id, result)
            }
        })
        .collect();

    let results = futures_util::future::join_all(futures).await;
    for (chat_id, result) in results {
        match result {
            Ok(resp) if resp.status().is_success() => {}
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                tracing::warn!(
                    "telegram_notifier: delivery failed to chat_id={} status={} body={}",
                    chat_id,
                    status,
                    body
                );
            }
            Err(e) => {
                tracing::warn!(
                    "telegram_notifier: delivery error to chat_id={}: {}",
                    chat_id,
                    e
                );
            }
        }
    }
}

/// Returns true if the cooldown key was newly set (i.e., alert should be sent).
/// Returns false if the key already existed (cooldown active).
async fn check_and_set_cooldown(
    redis: &Pool,
    key: &str,
    ttl_secs: i32,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = redis.get().await?;
    let result: Option<String> = deadpool_redis::redis::cmd("SET")
        .arg(key)
        .arg("1")
        .arg("NX")
        .arg("EX")
        .arg(ttl_secs)
        .query_async(&mut conn)
        .await?;
    Ok(result.is_some())
}
