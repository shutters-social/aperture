use aws_config::BehaviorVersion;
use axum::{Router, extract::MatchedPath, http::Request};
use state::AppState;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

mod atproto;
mod aws;
mod errors;
mod manip;
mod presets;
mod route;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME")
            )
            .into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let aws_cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .load()
        .await;
    let app_state = AppState::new(aws_cfg);

    let app = Router::new()
        .route(
            "/{preset}/{did}/{cid}/{format}",
            axum::routing::get(route::get_blob),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
