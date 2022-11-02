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
        Role::Admin => eprintln!("Admin logged in"),
        Role::User => eprintln!("User logged in"),
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
    let user2 = user.clone();
    users.write().await.insert(user_id, user);

    let users2 = users.clone();

    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(_) => {
                break;
            }
        };
        user_message(&user2, msg, &users).await;
    }

    user_disconnected(&user2, &users2).await;
}