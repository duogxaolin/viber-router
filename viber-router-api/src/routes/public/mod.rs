pub mod ttft;
pub mod uptime;
pub mod usage;

use axum::Router;

use crate::routes::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/usage", axum::routing::get(usage::public_usage))
        .route("/ttft", axum::routing::get(ttft::public_ttft))
        .route("/uptime", axum::routing::get(uptime::public_uptime))
}
