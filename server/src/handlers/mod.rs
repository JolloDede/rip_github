use axum::{Router, response::Html, routing::get};
use templating::html;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Ok" }))
        .route("/aha", get(|| async { Html("<h1>Test</h1>") }))
        .route(
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
