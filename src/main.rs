use std::fmt::Error;

use futures::TryFuture;
use warp::{Filter, Rejection, reject::Reject};

mod libs;

use libs::{user_connected, user::Users, Role, transform_param_to_role, no_anonymous_role};


#[tokio::main]
async fn main() {
    let users = Users::default();
    let users = warp::any().map(move || users.clone());

    let websocket_filter = warp::path!("ws" / String)
        .then(transform_param_to_role)
        .and_then(no_anonymous_role)
        .and(warp::ws())
        .and(users)
        .map(|role: Role, ws: warp::ws::Ws, users| {
            ws.on_upgrade(move |socket| user_connected(socket, users, role ))
        });

    warp::serve(websocket_filter).run(([0, 0, 0, 0], 8000)).await;
}

