use axum::{ extract::{Form, State }, http::StatusCode, response::{Response, IntoResponse}};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData{
    email: String,
    name: String, 
}

#[derive(Debug)]
pub struct AppError (
    pub sqlx::Error
        );

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("Internal server error: {:?}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

pub async fn subscribe(State(pool): State<PgPool>, form: Form<FormData>) -> Result<StatusCode, AppError> {
        sqlx::query!(
         r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at) 
            VALUES ( $1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now()
         )
        .execute(&pool)
        .await
        .map_err(AppError)?;
   Ok(StatusCode::OK)
}
