use axum::{Router, response::Html, routing::get};
use templating::html;
use tower_http::trace::TraceLayer;

pub fn router() -> Router {
    Router::new().route("/", get(|| async { "Ok" })).route(
        "/test",
        get(|| async {
            html!(
                h1 {
                    "Test"
                }
            )
        }),
    )
}
