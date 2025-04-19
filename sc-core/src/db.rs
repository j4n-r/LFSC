use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct MessageData {
    id: String,
    sender_id:String,
    target_type: TargetType,
    target_id: String,
    status: Status,
    content: String,
    sent_from_client: String,
    sent_from_server: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub message_type: MessageType,
    pub payload: Payload,
    pub meta: Meta
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub target_type: TargetType,
    pub target_id: String,
    pub content: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub message_id: String,
    pub sender_id: String,
    pub timestamp: String, // sent_from_client
}

#[derive(sqlx::Type,Deserialize, Serialize, Debug)]
#[sqlx(type_name = "TEXT")]       
#[sqlx(rename_all = "lowercase")]  
#[serde(rename_all = "camelCase")]
pub enum Status {
    Send,
    Received,
    Bufferred,
}

#[derive(sqlx::Type,Deserialize, Serialize, Debug, Clone)]
#[sqlx(type_name = "TEXT")]       
#[sqlx(rename_all = "lowercase")]  
#[serde(rename_all = "camelCase")]
pub enum TargetType {
    User,
    Group
}

#[derive(sqlx::Type,Deserialize, Serialize, Debug, Clone)]
#[sqlx(type_name = "TEXT")]       
#[sqlx(rename_all = "lowercase")]  
pub enum MessageType {
    ChatMessage,
    // ChatTyping,
    // File,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    created_at: String,
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

pub async fn save_message(
    pool: &SqlitePool,
    msg: WsMessage,
) -> Result<MessageData, sqlx::Error> {

    let msg_data = MessageData {
        id: Uuid::new_v4().to_string(),
        sender_id: msg.meta.sender_id,
        target_type: msg.payload.target_type,
        target_id: msg.payload.target_id,
        status: Status::Received,
        content: msg.payload.content,
        sent_from_client: msg.meta.timestamp,
        sent_from_server: Local::now().naive_utc().to_string()
    };

    sqlx::query!(
        r#"
        INSERT INTO messages (
            id, sender_id, target_type, target_id, status, content, sent_from_client, sent_from_server
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        msg_data.id,
        msg_data.sender_id,
        msg_data.target_type,
        msg_data.target_id,
        msg_data.status,
        msg_data.content,
        msg_data.sent_from_client,
        msg_data.sent_from_server,
    )
    .execute(pool)
    .await?;

    Ok(msg_data)
}
