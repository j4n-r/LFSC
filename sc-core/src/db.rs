use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MessageData {
    id: String,
    sender_id: String,
    conversation_id: String,
    status: Status,
    content: String,
    sent_from_client: String,
    sent_from_server: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub message_type: MessageType,
    pub payload: Payload,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdMessage {
    pub message_type: MessageType,
    pub sender_id: String,
    pub timestamp: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub conversation_id: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub message_id: String,
    pub sender_id: String,
    pub timestamp: String, // sent_from_client
}

#[derive(sqlx::Type, Deserialize, Serialize, Debug)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Send,
    Received,
    Bufferred,
}

#[derive(sqlx::Type, Deserialize, Serialize, Debug)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum ConversationType {
    Dm,
    Group,
}

#[derive(sqlx::Type, Deserialize, Serialize, Debug, Clone)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
pub enum MessageType {
    ChatMessage,
    IdMessage,
    // ChatTyping,
    // File,
}
#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    created_at: String,
}

#[derive(sqlx::FromRow , Deserialize, Serialize, Debug)]
pub struct Conversation {
    id: String,
    r#type: ConversationType,
    owner_id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    image: Option<String>,
    created_at: String,
    updated_at: String,
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

pub async fn save_message(pool: &SqlitePool, msg: WsMessage) -> Result<MessageData, sqlx::Error> {
    let msg_data = MessageData {
        id: Uuid::new_v4().to_string(),
        sender_id: msg.meta.sender_id,
        conversation_id: msg.payload.conversation_id,
        status: Status::Received,
        content: msg.payload.content,
        sent_from_client: msg.meta.timestamp,
        sent_from_server: Local::now().naive_utc().to_string(),
    };

    sqlx::query!(
        r#"
        INSERT INTO messages (
            id, sender_id,  conversation_id, status, content, sent_from_client, sent_from_server
        ) VALUES (?, ?, ?,  ?, ?, ?, ?)
        "#,
        msg_data.id,
        msg_data.sender_id,
        msg_data.conversation_id,
        msg_data.status,
        msg_data.content,
        msg_data.sent_from_client,
        msg_data.sent_from_server,
    )
    .execute(pool)
    .await?;

    Ok(msg_data)
}

pub async fn find_conversation(
    pool: &SqlitePool,
    conversation_id: String,
) -> Result<Option<Conversation>, sqlx::Error> {
    let conversation = sqlx::query_as::<_,Conversation>(
        r#"
SELECT id, type, owner_id, name, description, image, created_at, updated_at
FROM conversations
WHERE id = $1
"#,
    )
    .bind(conversation_id)
    .fetch_optional(pool)
        .await;
    conversation
}

pub async fn find_conversation_members(pool: &SqlitePool, conversation_id:String) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT conversation_id from conversation_members where conversation_id = ?"
        ,conversation_id)
        .fetch_all(pool)
        .await?;
    let members = rows.into_iter().map(|row| row.conversation_id).collect();
    Ok(members)
}
