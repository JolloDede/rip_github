use std::fmt::format;

use axum::{
    Router,
    extract::{Path, Request},
    http::Method,
    response::IntoResponse,
};

pub fn api_handler() -> Router {
    Router::new()
}
