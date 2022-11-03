use super::user::{Users, User};


pub async fn user_disconnected(user_id: usize, users: &Users) {
    eprintln!("user {} disconnected", user_id);
    users.write().await.remove(&user_id);
}