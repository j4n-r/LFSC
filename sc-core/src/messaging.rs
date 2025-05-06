use crate::db;
use crate::UserConn;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio_stream::StreamExt;
use tokio_tungstenite::tungstenite;
use anyhow::{Context, anyhow};

pub type Tx = tokio::sync::mpsc::UnboundedSender<tungstenite::Message>;
pub type PeerMap = Arc<Mutex<HashMap<UserConn, Tx>>>;

pub async fn handle_messaging(
    mut ws_stream: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    >,
    peer_map: PeerMap,
) -> anyhow::Result<()> {
    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(msg) => {
                let ws_msg = handle_message(msg)?;
                forward_to_peer(ws_msg, peer_map.clone())?;
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn handle_id_message(msg: tungstenite::Message) -> anyhow::Result<db::IdMessage> {
    match msg {
        tungstenite::Message::Text(text) => { serde_json::from_str(&text).context("not valid JSON")},
        _ => Err(anyhow!("not a Text Message")),
    }
}

pub fn handle_message(msg: tungstenite::Message) -> anyhow::Result<db::WsMessage> {
    match msg {
        tungstenite::Message::Text(text) => serde_json::from_str(&text).context("not valid JSON"),
        _ => Err(anyhow!("not a Text Message")),
    }
}

pub fn add_user_conn_to_peers(
    msg: db::IdMessage,
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

    user_con_clone
}

pub fn forward_to_peer(msg: db::WsMessage, peer_map: PeerMap) -> anyhow::Result<()> {
    let maybe_tx = {
        let peers = peer_map.lock().unwrap();
        peers
            .iter()
            .find(|(conn, _)| conn.id == msg.payload.conversation_id)
            .map(|(_, tx)| tx.clone())
    };

    if let Some(tx) = maybe_tx {
        let text = serde_json::to_string(&msg)?;
        println!(
            "sent {:?} to {:?}",
            &msg.clone(),
            msg.payload.conversation_id
        );
        tx.send(tungstenite::Message::text(text))?;
    }
    Ok(())
}
