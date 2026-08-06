use axum::{
    Router,
    extract::Form,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn subscribe(Form(_user): Form<FormData>) -> StatusCode {
    if _user.email.is_empty() || _user.name.is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    StatusCode::OK
}

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    let router = Router::new()
        .route("/check_health", get(health_check))
        .route("/subscriptions", post(subscribe));
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    Ok(())
}
