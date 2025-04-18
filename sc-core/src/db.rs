use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct MessageData {
    id: String,
    message: Msg,
    status: Status,
    sent_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
struct Msg {
    send_id: String,
    recv_id: String,
    text: String,
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

pub async fn save_message(pool: Arc<SqlitePool>, msg: Msg) ->Result<MessageData, sqlx::Error>{
    let msg_data = {
        id: Uuid::new_v4(),
        msg: msg,"smt",
        status: Status::Received,
        sent_at: chrono::naive::NaiveDateTime::new(date, time)
        

    }
}

