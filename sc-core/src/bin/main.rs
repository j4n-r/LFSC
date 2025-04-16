use std::time::SystemTime;
use std::env;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // build our application with a single route
    let app = Router::new().route("/", get(message_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct Message {
    send_id: UUID,
    recv_id:UUID,
    status: UUID,
    sent_at: SystemTime, 
}

async fn message_handler(Json(payload): Json<Message>) ->  Result<String, (StatusCode, String)>{
    
}


