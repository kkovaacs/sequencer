use axum::{
    Json, Router,
    extract::Query,
    routing::{get, post},
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(serde_derive::Deserialize)]
struct EthToStrkOracleQuery {
    timestamp: u64,
}

async fn eth_to_strk_oracle_get_price(
    Query(query): Query<EthToStrkOracleQuery>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "timestamp": query.timestamp,
        "price": "0x3635c9adc5dea00000", // 10^21
        "decimals": 18,
    }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let format = tracing_subscriber::fmt::format().compact();
    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener_address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "[::]:8080".to_owned());
    tracing::info!(%listener_address, "Starting up");
    let listener = tokio::net::TcpListener::bind(listener_address).await?;

    let router = Router::new()
        .route("/cende_recorder/write_blob", post(move || async { "" }))
        .route(
            "/cende_recorder/write_pre_confirmed_block",
            post(move || async { "" }),
        )
        .route("/eth_to_strk_oracle", get(eth_to_strk_oracle_get_price));

    axum::serve(listener, router.layer(TraceLayer::new_for_http())).await
}
