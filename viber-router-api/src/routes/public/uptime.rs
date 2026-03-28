use axum::{
    Json,
    extract::{ConnectInfo, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::rate_limiter;
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct UptimeParams {
    key: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicUptimeResponse {
    status: String,
    uptime_percent: f64,
    buckets: Vec<ChainBucket>,
}

#[derive(Debug, Serialize)]
pub struct ChainBucket {
    timestamp: i64,
    total_requests: i64,
    successful_requests: i64,
}

fn err(status: StatusCode, msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(serde_json::json!({"error": msg})))
}

fn extract_client_ip(headers: &HeaderMap, addr: SocketAddr) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

// PLACEHOLDER_HANDLER

pub async fn public_uptime(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Query(params): Query<UptimeParams>,
) -> impl IntoResponse {
    let client_ip = extract_client_ip(&headers, addr);

    // Rate limit check
    if rate_limiter::is_ip_rate_limited(&state.redis, &client_ip).await {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [(header::RETRY_AFTER, "60")],
            Json(serde_json::json!({"error": "Too many requests"})),
        )
            .into_response();
    }

    let Some(key) = params.key.filter(|k| !k.is_empty()) else {
        return err(StatusCode::BAD_REQUEST, "key parameter is required").into_response();
    };

    rate_limiter::increment_ip_rate_limit(&state.redis, &client_ip).await;

    // Lookup sub-key + group
    #[derive(sqlx::FromRow)]
    struct KeyInfo {
        group_id: uuid::Uuid,
    }

    let key_info = sqlx::query_as::<_, KeyInfo>(
        "SELECT gk.group_id \
         FROM group_keys gk JOIN groups g ON g.id = gk.group_id \
         WHERE gk.api_key = $1 AND gk.is_active = true",
    )
    .bind(&key)
    .fetch_optional(&state.db)
    .await;

    let key_info = match key_info {
        Ok(Some(info)) => info,
        Ok(None) => {
            return err(StatusCode::FORBIDDEN, "Invalid or inactive key").into_response();
        }
        Err(_) => {
            return err(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response();
        }
    };

    // Generate 90 bucket timestamps
    let now_epoch = chrono::Utc::now().timestamp();
    let bucket_size: i64 = 1800;
    let current_bucket = (now_epoch / bucket_size) * bucket_size;
    let bucket_timestamps: Vec<i64> = (0..90).rev().map(|i| current_bucket - i * bucket_size).collect();

    let cutoff = chrono::DateTime::from_timestamp(bucket_timestamps[0], 0)
        .unwrap_or_default();

    // Chain-level aggregation: a request is successful if ANY attempt has 2xx
    #[derive(sqlx::FromRow)]
    struct RawChainBucket {
        bucket: i64,
        total_requests: i64,
        successful_requests: i64,
    }

    let raw_buckets = sqlx::query_as::<_, RawChainBucket>(
        "WITH bucketed AS ( \
           SELECT request_id, \
             (floor(extract(epoch from created_at) / 1800) * 1800)::bigint as bucket, \
             bool_or(status_code BETWEEN 200 AND 299) as any_success \
           FROM uptime_checks \
           WHERE group_id = $1 AND created_at >= $2 \
           GROUP BY request_id, bucket \
         ) \
         SELECT bucket, \
           COUNT(*)::bigint as total_requests, \
           COUNT(*) FILTER (WHERE any_success)::bigint as successful_requests \
         FROM bucketed \
         GROUP BY bucket",
    )
    .bind(key_info.group_id)
    .bind(cutoff)
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    // Build buckets
    let mut buckets = Vec::with_capacity(90);
    for &ts in &bucket_timestamps {
        let matching = raw_buckets.iter().find(|b| b.bucket == ts);
        buckets.push(ChainBucket {
            timestamp: ts,
            total_requests: matching.map_or(0, |b| b.total_requests),
            successful_requests: matching.map_or(0, |b| b.successful_requests),
        });
    }

    // Derive status from the most recent bucket
    let last_bucket = buckets.last().unwrap();
    let (status_text, uptime_percent) = if last_bucket.total_requests == 0 {
        ("no_data".to_string(), 0.0)
    } else {
        let pct = last_bucket.successful_requests as f64 / last_bucket.total_requests as f64 * 100.0;
        let status = if pct > 95.0 {
            "operational"
        } else if pct >= 50.0 {
            "degraded"
        } else {
            "down"
        };
        (status.to_string(), pct)
    };

    Json(PublicUptimeResponse {
        status: status_text,
        uptime_percent,
        buckets,
    })
    .into_response()
}
