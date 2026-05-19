use std::env;

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{any, get},
};

mod api;
use api::*;
mod handlers;
use handlers::*;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    let app = router().nest("/api", api_handler());

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
