use super::user::{Users, User};


pub async fn user_disconnected(user: &User, users: &Users) {
    eprintln!("user {} disconnected", user.id);
    users.write().await.remove(&user.id);
}