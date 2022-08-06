use crate::{handlers::chat_mesage::handle_chat_message, ws_wrapper::Client};

use tokio_tungstenite::tungstenite::Result;
use futures_util::{SinkExt, StreamExt};
use types::ClientMessage;
pub struct ConnectedUser {
    id: String,
    pub client: Client
}

impl ConnectedUser {
    pub fn new(id: String, client: Client) -> Self {
        ConnectedUser { id, client }
    }

    // Main receiving loop.
    // Receives the message and just send it back.
    pub async fn listen(&mut self) -> Result<()> {
        while let Some(msg) = self.client.ws.next().await {
            let msg = msg?;
            if msg.is_text() {
                let client_message: ClientMessage = serde_json::from_str(msg.to_string().as_str()).unwrap();
                println!("[{}] - {:?}", self.id, client_message);
                match client_message {
                    ClientMessage::ChatMessage(c) => handle_chat_message(c, self).await,
                    ClientMessage::ConnectionPayloadMessage(_) => todo!(),
                }}
        }
        Ok(())
    }
}