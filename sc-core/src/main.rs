use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod db;
mod messaging;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct UserConn {
    id: String,
    conv_id:String,
    addr: std::net::SocketAddr,
}
#[tracing::instrument]
async fn handle_connection(
    peer_map: messaging::PeerMap,
    pool: Arc<sqlx::SqlitePool>,
    raw_stream: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> anyhow::Result<()> {
    // perform websocket handshake on a accepted connection
    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    // tx=transmit, rx=receive (tx and rx as a channel between tokio tasks)
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // new tokio task which forwars all messages from rx end to the ws sink ( ws_stream -> tx -> |task bound| -> rx -> ws_sink)
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = ws_sink.send(msg).await {
                println!("WebSocket send error: {:?}", e);
                break;
            }
        }
    });

    let mut user_conn: Option<UserConn> = None;
    // get id from the first message and save the user conn in the peer map
    if let Some(Ok(first_msg)) = ws_stream.next().await {
        let user_msg = messaging::deserialize_id_message(first_msg)?;
        println!("Id Message: {:?}", user_msg);
        user_conn =
            messaging::add_user_conn_to_peers(user_msg.clone(), addr, tx, peer_map.clone(), &pool).await?;
    }

    // remove the connection after the client disconnects
    if let Some(user_conn) = user_conn {
        // forward each message after the first to all peers
        messaging::handle_message(&pool, ws_stream, peer_map.clone(), user_conn).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber)?;
    let pool = Arc::new(sqlx::SqlitePool::connect(&std::env::var("DATABASE_URL")?).await?);
    let addr = "0.0.0.0:8080".to_string();

    let state = messaging::PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = tokio::net::TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    tracing::info!("Listening on: {}", addr);

    // spawn new thread for each connection
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), pool.clone(), stream, addr));
    }

    Ok(())
}
