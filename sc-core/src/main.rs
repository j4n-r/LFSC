mod db;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use chrono::{ NaiveDateTime, Utc};
use futures_channel::mpsc::UnboundedSender;
use futures_util::{SinkExt, StreamExt};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Serialize, Deserialize)]
struct Msg {
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

async fn handle_connection(
    peer_map: PeerMap,
    pool: Arc<SqlitePool>,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut write, read) = ws_stream.split();

    let user = db::get_user(pool.clone()).await.unwrap();
    let json = serde_json::to_string(&user).unwrap();

    let msg_to_send = Message::text(json);
    write
        .send(msg_to_send)
        .await
        .expect("failed to forward message");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = Arc::new(SqlitePool::connect(&env::var("DATABASE_URL")?).await?);
    let addr = "127.0.0.1:8080".to_string();

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), pool.clone(), stream, addr));
    }

    Ok(())
}
