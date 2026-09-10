use axum::{
       Router,
         routing::{get, post},
     };
 use tokio::net::TcpListener;
 use crate::routes::{health_check, subscribe};
 use sqlx::PgConnection;
 use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    conn: Arc<Mutex<PgConnection>>,
}

    pub async fn run(listener: TcpListener, pool: PgConnection) -> Result<(), std::io::Error> {

        let state = AppState {
            conn: Arc::new(Mutex::new(pool)),
        };
       let router = Router::new()
           .route("/check_health", get(health_check))
            .route("/subscriptions", post(subscribe)).with_state(state.conn);
       tokio::spawn(async move {
           axum::serve(listener, router).await.unwrap();
       });
       Ok(())
   }

