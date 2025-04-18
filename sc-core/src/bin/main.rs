use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex}, time::SystemTime,
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, SinkExt, StreamExt};

use sqlx::SqlitePool;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::{uuid, Uuid};
use serde::{Deserialize};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;


async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (write, read) = ws_stream.split();
    let msg = Message::text("message");
    read.forward(write).await.expect("failed to forward message");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let addr = "127.0.0.1:8080".to_string();

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}

#[derive(Deserialize)]
struct Msg {
    id: Uuid,
    send_id: Uuid,
    recv_id:Uuid,
    status: Uuid,
    sent_at: SystemTime, 
}

// async fn message_handler(State(pool): State<SqlitePool>) ->   Result<Json<Vec<Message>>, (StatusCode, String)>{
//  let recs = sqlx::query!(
//         r#"
// SELECT *
// FROM message_data
// ORDER BY status
//         "#
//     )
//     .fetch_all(&pool)
//      .await
//      .unwrap();

    // Ok(Json(recs))
// }


