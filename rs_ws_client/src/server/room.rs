use crate::handler_trait::HandleMessage;

pub struct Room {
}

impl HandleMessage<String> for Room {
    fn handle(_: String) {
        println!("Handling messsage at {}", file!());
    }
}
