use warp::ws::Message;

use super::user::{Users, User};

pub async fn user_message(sender: &User, msg: Message, users: &Users) {
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", sender.id, msg);

    for (&uid, user) in users.read().await.iter() {
        if sender.id != uid {
            if let Err(_disconnected) = user.ws.send(Ok(Message::text(new_msg.clone()))) {
                //ops
            }
        }
    }
}