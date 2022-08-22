use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::Result;
use types::ClientHandler;

pub struct Client {
    pub ws: WebSocketStream<TcpStream>
}

impl Client {
    pub fn new(ws: WebSocketStream<TcpStream>) -> Self {
        Client {ws}
    }

    pub async fn send_message(self: &mut Self, msg: ClientHandler) -> Result<()> {
        let serialized_message = serde_json::to_string(&msg).unwrap();
        let message = Message::Text(serialized_message);
        self.ws.send(message).await?;
        
        Ok(())
    }
}
