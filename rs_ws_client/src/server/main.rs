mod connected_user;
mod handlers;
mod ws_wrapper;
mod handler_trait;
mod room;

use std::net::SocketAddr;

use tokio::net::{TcpListener, TcpStream};
use log::*;
use tokio_tungstenite::{accept_async};
use tokio_tungstenite::tungstenite::{Result, Error};

use crate::connected_user::ConnectedUser;
use crate::ws_wrapper::Client;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => {},
            err => error!("Error proccessing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept stream");
    let client = Client::new(ws_stream);
    info!("New WebSocket connection: {}", peer);
    ConnectedUser::new("client".to_string(), client).listen().await
}

#[tokio::main]
async fn main() {
    println!("Starting server!");

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen.");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("Connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }

}
