use super::Role;

pub async fn transform_param_to_role(login_id: String) -> Role{
    match login_id.as_str() {
        "a" => Role::Admin,
        "b" => Role::User,
        _ => Role::Anonymous,
    }
}