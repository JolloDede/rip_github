use std::{env, str::Bytes, time::Duration};

use axum::{
    Router,
    body::HttpBody,
    extract::MatchedPath,
    http::{HeaderMap, Request, StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
use api::*;
mod handlers;
use handlers::*;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=info",
                    // "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false),
        )
        .init();

    let app = router().nest("/api", api_handler()).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.

                info_span!("server")
            })
            .on_request(|request: &Request<_>, _span: &Span| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str)
                    .unwrap_or_else(|| "/");

                tracing::info!("--> {} {}", request.method(), matched_path);
            })
            .on_response(|response: &Response, latency: Duration, _span: &Span| {
                let hint = response.body().size_hint();
                let size = hint.exact().or(hint.upper()).unwrap_or_else(|| 0);
                size_conversions(size);

                tracing::info!(
                    // status = %response.status(),
                    // latency_ms = %latency.as_millis(),
                    // "request finished"
                    "<-- {} {} {} (<-> {}ms)",
                    response.status(),
                    size_to_string(size_conversions(size)),
                    content_type(response).unwrap_or(""),
                    latency.as_millis()
                );
            })
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    tracing::error!("something went wrong")
                },
            ),
    );

    let port = env::var("API_PORT").unwrap_or("8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Err(err) = result {
        println!("Error: {:?}", err);
    }
}

#[derive(Debug)]
enum AppError {
    NotFound,
    DatabaseError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found").into_response(),
            AppError::DatabaseError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
            }
        }
    }
}

fn size_conversions(bytes: u64) -> (u64, f64, f64, f64, f64) {
    let bits = bytes * 8;
    let kb = bytes as f64 / 1_000.0;
    let mb = bytes as f64 / 1_000_000.0;
    let kib = bytes as f64 / 1024.0;
    let mib = bytes as f64 / 1_048_576.0;

    (bits, kb, mb, kib, mib)
}

fn size_to_string(size: (u64, f64, f64, f64, f64)) -> String {
    let bytes = size.0 / 8;

    if size.4 >= 1.0 {
        format!("{:.2} MiB", size.4)
    } else if size.3 >= 1.0 {
        format!("{:.2} KiB", size.3)
    } else if size.2 >= 1.0 {
        format!("{:.2} MB", size.2)
    } else if size.1 >= 1.0 {
        format!("{:.2} KB", size.1)
    } else {
        format!("{:.2} B", bytes as f64)
    }
}

fn content_type(response: &Response) -> Option<&str> {
    response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
}
