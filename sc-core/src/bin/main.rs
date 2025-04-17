use std::time::SystemTime;
use std::env;
use serde::Deserialize;
use tracing_subscriber::prelude::*;     
use sqlx::SqlitePool;                 
use uuid::Uuid;                      
use axum::{
    extract::State, http::StatusCode, routing::get, Json, Router
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // build our application with a single route
    let app = Router::new().route("/", get(message_handler)).with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Deserialize)]
struct Message {
    id: Uuid,
    send_id: Uuid,
    recv_id:Uuid,
    status: Uuid,
    sent_at: SystemTime, 
}

async fn message_handler(State(pool): State<SqlitePool>) ->   Result<Json<Vec<Message>>, (StatusCode, String)>{
 let recs = sqlx::query!(
        r#"
SELECT *
FROM message_data
ORDER BY status
        "#
    )
    .fetch_all(&pool)
     .await
     .unwrap();

    Ok(Json(recs))
}

