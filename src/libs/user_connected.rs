use std::sync::atomic::{Ordering, AtomicUsize};

use futures::{StreamExt, FutureExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{WebSocket};

use super::user::{Users, User};
use super::{user_message, user_disconnected, Role};
static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub async fn user_connected(ws: WebSocket, users: Users, role: Role) {
    match role {
        Role::Admin(_) => eprintln!("Admin logged in. info:"),
        Role::User(_) => eprintln!("User logged in."),
        Role::Anonymous => eprintln!("Anonymous tried to log in")
    }

    let user_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let (user_ws_tx, mut user_ws_rx) = ws.split();
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    let user: User = User { id: user_id, ws: tx, role };
    {
        users.write().await.insert(user_id, user);
    }
    let users2 = users.clone();

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => {
                break;
            }
        };

       match users.try_read() {
            Ok(map)=> {
                match map.get(&user_id) {
                    Some(usr) => {user_message(&usr, msg, &users).await;}
                    _ => {}
                }
            },
            Err(_)=> {}
        }
    }

    user_disconnected(user_id, &users2).await;
}