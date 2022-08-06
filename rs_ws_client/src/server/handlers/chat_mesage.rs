use types::{ChatMessage, ClientMessage};
use crate::connected_user::ConnectedUser;


pub async fn handle_chat_message(message: ChatMessage, connected_user: &mut ConnectedUser) {
    println!("Received chat message: {:?}", message); 
    let chat_mesage = ChatMessage { user_id: "server".to_string(), message: "chat received.".to_string()};
    connected_user.client.send_message(ClientMessage::ChatMessage(chat_mesage)).await.unwrap();
}
// use types::ChatMessage;