use serde_json::json;
use warp::ws::Message;

use super::{user::{Users, User}, Role};

pub async fn user_message(sender: &User, msg: Message, users: &Users) {
    match sender.role {
        Role::Admin(_)=>{}
        _=>{
            return ;
        }
    }

    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    //users get the admin message broadcast
    for (&uid, user) in users.read().await.iter() {
        if sender.id == uid { continue; }
        
        match &user.role {
            Role::User(info)=>{
                let body = json!({
                    "user_info": info,
                    "message": msg
                }).to_string();

                user.ws
                    .send(Ok(Message::text(body)))
                    .unwrap_or(());
            }
            _=>{}
        }    
    }
}