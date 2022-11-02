use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;

use super::Role;

#[derive(Clone)]
pub struct User{
    pub id: usize,
    pub ws: mpsc::UnboundedSender<Result<Message, warp::Error>>,
    pub role: Role
}

pub type Users = Arc<RwLock<HashMap<usize, User>>>;