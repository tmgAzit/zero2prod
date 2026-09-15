use axum::{
       Router,
         routing::{get, post},
     };
 use tokio::net::TcpListener;
 use crate::routes::{health_check, subscribe};
 use sqlx::PgPool;


 pub async fn run(listener: TcpListener, pool:PgPool) -> Result<(), std::io::Error> {

       let router = Router::new()
           .route("/check_health", get(health_check))
            .route("/subscriptions", post(subscribe)).with_state(pool);
       tokio::spawn(async move {
           if let Err(e) = axum::serve(listener, router).await {
               eprintln!("server error: {e}");
           }
       });
       Ok(())
   }

