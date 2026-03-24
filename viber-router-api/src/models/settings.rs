use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Settings {
    pub telegram_bot_token: Option<String>,
    pub telegram_chat_ids: Vec<String>,
    pub alert_status_codes: Vec<i32>,
    pub alert_cooldown_mins: i32,
}
