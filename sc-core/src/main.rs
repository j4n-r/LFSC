mod db;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use db::{IdMessage, WsMessage};
use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use serde_json::{to_string, Error};
use sqlx::SqlitePool;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<UserConn, Tx>>>;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct UserConn {
    id: String,
    addr: SocketAddr,
}

async fn handle_connection(
    peer_map: PeerMap,
    pool: Arc<SqlitePool>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    // perform websocket handshae on a accepted connection
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (ws_sink, mut ws_stream) = ws_stream.split();
    // tx=transmit, rx=receive (tx and rx as a channel between tokio tasks)
    let (tx, rx) = unbounded();

    // new tokio task which forwars all messages from rx end to the ws sink ( ws_stream -> tx -> |task bound| -> rx -> ws_sink)
    tokio::spawn(rx.map(Ok).forward(ws_sink));

    let mut user_con: Option<UserConn> = None;
    // get id from the first message and save the user conn in the peer map
    if let Some(Ok(first_msg)) = ws_stream.next().await {
        let user_msg = handle_id_message(first_msg);
        user_con = Some(add_user_conn_to_peers(user_msg.clone(), addr, tx, peer_map.clone()));
    }
    // forward each message after the first to all peers
    ws_stream
        .try_for_each(|msg| {
            let ws_msg = handle_message(msg);
            let _ = forward_to_peer(ws_msg, peer_map.clone()).unwrap();
            future::ok(())
        })
        .await
        .expect("Error processing incoming messages");
    
    println!("{} disconnected", &addr);
    // remove the connection after the client disconnects
    if let Some(user_con) = &user_con {
        peer_map.lock().unwrap().remove(user_con);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Arc::new(SqlitePool::connect(&env::var("DATABASE_URL")?).await?);
    let addr = "192.168.0.240:8080".to_string();

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // spawn new thread for each connection
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), pool.clone(), stream, addr));
    }

    Ok(())
}

fn handle_id_message(msg: Message) -> IdMessage {
    match msg {
        Message::Text(text) => serde_json::from_str(&text).expect("not valid JSON"),
        _ => panic!("not a text message: {:?}",msg),
    }
}

fn handle_message(msg: Message) -> WsMessage {
    match msg {
        Message::Text(text) => serde_json::from_str(&text).expect("not valid JSON"),
        _ => panic!("not a text message"),
    }
}

fn add_user_conn_to_peers(msg: IdMessage, addr: SocketAddr, tx: Tx, peer_map: PeerMap) -> UserConn {
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

fn forward_to_peer(msg: WsMessage, peer_map: PeerMap) -> Result<(), String> {
    let maybe_tx = {
        let peers = peer_map.lock().unwrap();
        peers
            .iter()
            .find(|(conn, _)| conn.id == msg.payload.target_id)
            .map(|(_, tx)| tx.clone())
    };

    if let Some(tx) = maybe_tx {
        let text = to_string(&msg).map_err(|e| format!("JSON serialize error: {}", e))?;
        println!("sent {:?} to {:?}", &msg.clone(), msg.payload.target_id);
        tx.unbounded_send(Message::text(text))
            .map_err(|e| format!("Send error: {}", e))?;
        Ok(())
    } else {
        Err("Target not connected".into())
    }
}
