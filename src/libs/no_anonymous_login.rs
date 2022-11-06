use warp::{reject::Reject, Rejection};

use super::Role;

pub async fn no_anonymous_login(role: Role) -> Result<Role, Rejection> {
    match role {
        Role::Anonymous => Err(warp::reject::custom(InvalidLogin)),
        _ => Ok(role)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InvalidLogin;

impl Reject for InvalidLogin {}