use axum::{ extract::Form, http::StatusCode};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String, 
}

pub async fn subscribe(_form:Form<FormData>, _conn: PgConnection) -> StatusCode {
    StatusCode::OK
}
