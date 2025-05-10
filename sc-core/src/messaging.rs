use crate::db;
use crate::db::{MessageType, Status};
use crate::UserConn;
use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite;

#[derive(Serialize, Deserialize)]
pub struct MessageData {
    pub id: Option<String>,
    pub sender_id: String,
    pub conversation_id: String,
    pub status: Status,
    pub content: String,
    pub sent_from_client: String,
    pub sent_from_server: String,
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
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub conversation_id: String,
    pub sender_id: String,
    pub timestamp: String, // sent_from_client
}

pub type Tx = tokio::sync::mpsc::UnboundedSender<tungstenite::Message>;
pub type PeerMap = Arc<Mutex<HashMap<UserConn, Tx>>>;
pub type WsStream =
    futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>;

pub async fn handle_message(
    pool: &sqlx::SqlitePool,
    mut ws_stream: WsStream,
    peer_map: PeerMap,
) -> anyhow::Result<()> {
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(msg) => {
                let ws_msg = deserialize_message(msg)?;
                tracing::debug!("got msg from: {:?}", ws_msg.meta.sender_id);
                let peer_ids =
                    db::find_conversation_members(pool, ws_msg.meta.conversation_id.clone())
                        .await
                        .context("findind conversation members failed")?;

                let conv_members: HashMap<UserConn, Tx> = peer_map
                    .lock()
                    .unwrap()
                    .iter()
                    .filter(|(user_conn, _)| peer_ids.contains(&user_conn.id))
                    .map(|(user_conn, tx)| (user_conn.clone(), tx.clone()))
                    .collect();

                println!("{}", conv_members.clone().into_iter().count());
                for (user_conn, _) in conv_members.clone() {
                    println!("{:?}", user_conn.id)
                }

                forward_to_peer(ws_msg, conv_members)?;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

#[tracing::instrument]
pub fn deserialize_id_message(msg: tungstenite::Message) -> anyhow::Result<IdMessage> {
    match msg {
        tungstenite::Message::Text(text) => serde_json::from_str(&text).context("not valid JSON"),
        _ => Err(anyhow!("not a Text Message")),
    }
}

#[tracing::instrument]
pub fn deserialize_message(msg: tungstenite::Message) -> anyhow::Result<WsMessage> {
    match msg {
        tungstenite::Message::Text(text) => {
            serde_json::from_str(&text).context("not valid JSON")
        }
        _ => Err(anyhow::anyhow!("not a Text Message")),
    }
}

#[tracing::instrument]
pub fn add_user_conn_to_peers(
    msg: IdMessage,
    addr: std::net::SocketAddr,
    tx: Tx,
    peer_map: PeerMap,
) -> UserConn {
    let user_con = UserConn {
        id: msg.sender_id.clone(),
        addr,
    };
    let user_con_clone = user_con.clone();
    println!("Added user: {:?}", user_con);
    // save the user id + address and the corresponding transmitter for the tokio task in the peer map
    peer_map.lock().unwrap().insert(user_con, tx);

    println!("connected Users:");
    for (user_conn, _) in peer_map.lock().unwrap().clone().into_iter() {
        println!("{:?}", user_conn)
    }
    println!("");
    user_con_clone
}

#[tracing::instrument]
pub fn forward_to_peer(msg: WsMessage, conv_members: HashMap<UserConn, Tx>) -> anyhow::Result<()> {
    for (user_conn, tx) in conv_members {
        let text = serde_json::to_string(&msg)?;
        println!("sent {:?} to {:?}", &msg.clone(), user_conn.id);
        tx.send(tungstenite::Message::text(text))?;
    }
    Ok(())
}
