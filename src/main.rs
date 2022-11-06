use warp::{Filter, ws::{Ws, self}, Rejection, reject::Reject,};
use dotenv::dotenv;
mod libs;

use libs::{user_connected, user::Users, Role, token_to_role, no_anonymous_login};


#[tokio::main]
async fn main() {
    dotenv().ok();

    let users = warp::any().map(move || Users::default().clone());

    let auth_endpoint = std::env::var("AUTH_ENDPOINT").expect("add auth_endpoint env var");
    let auth_endpoint = warp::any().map(move || { auth_endpoint.clone() });

    let websocket_filter = warp::path("ws")
        .and(get_token())
        .and(auth_endpoint)
        .then(token_to_role)
        .and_then(no_anonymous_login)
        .and(warp::ws())
        .and(users)
        .map(|role: Role, ws: warp::ws::Ws, users| {
            ws.on_upgrade(move |socket| user_connected(socket, users, role ))
        });

    warp::serve(websocket_filter).run(([0, 0, 0, 0], 8000)).await;
}

fn get_token() -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::header::<String>("Sec-WebSocket-Protocol")
}