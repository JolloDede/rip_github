use axum::{Router, http::StatusCode, response::Html, routing::get};
use templating::html;
use tower_http::trace::TraceLayer;

pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Ok" }))
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
        .route("/login", get(get_login_form))
}

fn base_layout() {
    html!(
        @Body
    )
}

async fn get_login_form() -> Result<Html<String>, StatusCode> {
    Ok(html!(
        @layout(base_layout) {
        form {
            label {
                class: "",
                span {
                    "Username"
                }
                input {
                    r#type: "text",
                    id: "username",
                    name: "username",
                }
            }
        }
        }
    ))
}
