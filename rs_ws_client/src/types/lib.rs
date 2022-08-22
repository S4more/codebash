use ts_rs::TS;
use serde::{Deserialize, Serialize};
//
//#[derive(TS)]
//#[derive(Serialize, Deserialize, Debug)]
//#[ts(export, export_to="../node-client/bindings/messageTypes/ClientMessage/")]
//pub struct ChatMessage{
//    pub user_id: String,
//    pub message: String,
//}
//
//#[derive(TS)]
//#[ts(export, export_to="../node-client/bindings/messageTypes/ClientMessage/")]
//#[derive(Serialize, Deserialize, Debug)]
//pub struct ConnectionPayloadMessage {
//    pub data1: String,
//    pub data2: u8,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//#[derive(TS)]
//#[ts(export, export_to="../node-client/bindings/enums/")]
//pub enum ClientMessage {
//    ChatMessage(ChatMessage),
//    ConnectionPayloadMessage(ConnectionPayloadMessage),
//}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientHandler {
    ChatController(ClientChatMessages),
    LobbyController(ClientLobbyControllerMessages),
    ClientRoomController(ClientRoomController),
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientChatMessages {
    TryAddMessage(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientLobbyControllerMessages {
    TryEnterRoom{id: String},
    TryCreateRoom{id: String},
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientRoomController {
    TextAreaController(TextAreaControllerMessages),
    ChatController(ClientRoomChatControllerMessages),
    CompileController(ClientRoomCompilerMessages),
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum TextAreaControllerMessages {
    TryUpdate{message: String},
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientRoomChatControllerMessages {
    TrySendChat{message: String},
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(TS)]
#[ts(export, export_to="../node-client/bindings/enums/")]
pub enum ClientRoomCompilerMessages {
    TryRunHiddenTests(),
    TryRunVisibleTests(),
}


