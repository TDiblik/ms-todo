use crate::token::{get_current_access_token, get_current_access_token_async};

pub async fn authed_req_async(method: reqwest::Method, url: String) -> reqwest::RequestBuilder {
    let access_token = get_current_access_token_async().await;
    let req = reqwest::Client::new();
    req.request(method, url)
        .header("Authorization", format!("Bearer {}", access_token))
}

#[allow(dead_code)] // TODO: Could be usefull in the future
pub fn authed_req(method: reqwest::Method, url: String) -> reqwest::blocking::RequestBuilder {
    let access_token = get_current_access_token();
    let req = reqwest::blocking::Client::new();
    req.request(method, url)
        .header("Authorization", format!("Bearer {}", access_token))
}
