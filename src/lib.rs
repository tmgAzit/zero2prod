use axum::{http::StatusCode, Router, routing::get};
use tokio::net::TcpListener;

async fn health_check()-> StatusCode{
    StatusCode::OK
}

pub async fn run(listener:TcpListener) -> Result<(), std::io::Error> {
   let router = Router::new().route("/check_health", get(health_check));
    tokio::spawn(async move { axum::serve(listener, router).await.unwrap();});
   Ok(())
}
