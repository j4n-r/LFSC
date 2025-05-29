use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::messaging;

#[derive(sqlx::Type, Deserialize, Serialize, Debug)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Sent,
    Delivered,
    Bufferred,
    Buffered,
    Read
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
#[sqlx(rename_all = "camelCase")]
pub enum MessageType {
    ChatMessage,
    IdMessage,
    // ChatTyping,
    // File,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
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


pub async fn save_message(pool: &SqlitePool, msg: messaging::WsMessage) -> anyhow::Result<()>{
    let msg_data = messaging::MessageData {
        id: Some(Uuid::new_v4().to_string()),
        sender_id: msg.meta.sender_id,
        conversation_id: msg.meta.conversation_id,
        status: Status::Delivered,
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

    tracing::debug!("saved message");
    Ok(())
}

pub async fn find_conversation_members(pool: &SqlitePool, conversation_id:String) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT user_id from conversation_members where conversation_id = ?"
        ,conversation_id)
        .fetch_all(pool)
        .await?;
    let members = rows.into_iter().map(|row| row.user_id).collect();
    tracing::debug!("Members for conversation {}: {:?}", conversation_id, members); 
    Ok(members)
}
