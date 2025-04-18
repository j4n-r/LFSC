use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct Message_data {
    id: String,
    send_id: String,
    recv_id: String,
    status: Status,
    sent_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
enum Status {
    Send,
    Received,
    Bufferred,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    created_at: NaiveDateTime,
}

pub async fn get_user(pool: Arc<SqlitePool>) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
            SELECT id, username, created_at
            FROM users
            ORDER BY created_at DESC
            "#
    )
    .fetch_one(&*pool)
    .await?;

    Ok(user)
}
