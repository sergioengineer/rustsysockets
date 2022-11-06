use futures::Future;
use reqwest::Error;
use serde_json::{Value, Map, json};
use warp::{header, Filter, reply::with::headers, hyper::client::conn::Connection};

use super::{Role};

pub async fn token_to_role(token: String, auth_endpoint: String) -> Role{
    let client = reqwest::Client::new();
    let body = json!({
        "token": token
    }).to_string();

    let authresponse = client.post(auth_endpoint)
    .header("content-type", "application/json")
        .body(body)
        .send()
        .await
        .map(|op| {
             return op.text()
        })
        .unwrap()
        .await
        .map_or_else(|_| Role::Anonymous, |text| response_to_role(text));


    authresponse
}

fn response_to_role(text: String)->Role{
    serde_json::from_str(text.as_str())
        .map(|op: Value| match op{
            Value::Object(obj)=> jsonobj_to_role(obj),
            _ => Role::Anonymous
        })
        .unwrap_or(Role::Anonymous)
}

fn jsonobj_to_role(obj: Map<String, Value>)->Role{
    obj["role"]
        .as_str()
        .map(|role| (role, obj["information"].as_str().unwrap_or("")))
        .map(|(role, info)| match role{
            "admin"=> Role::Admin(String::from(info)),
            "user"=> Role::User(String::from(info)),
            _=> Role::Anonymous
        })
        .unwrap_or(Role::Anonymous)
}