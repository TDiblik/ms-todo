use std::collections::HashMap;

use chrono::{DateTime, Duration, Local};
use serde::Deserialize;

use crate::{
    config::{get_config, save_config},
    constants::{AZURE_OAUTH_CLIENT_ID, AZURE_OAUTH_SCOPE, AZURE_OAUTH_TENANT},
};

// All properties: https://learn.microsoft.com/en-us/graph/auth-v2-user?tabs=http#response
// Currentlly, I don't care about other properties.
#[derive(Debug, Deserialize)]
pub struct AzureOAuthTokenResp {
    pub expires_in: u32,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct CurrentAccessToken {
    expires_at: DateTime<Local>,
    token: String,
}

static mut CURRENT_ACCESS_TOKEN: Option<CurrentAccessToken> = None;

fn token_is_valid(token: &CurrentAccessToken) -> bool {
    (token.expires_at.time() - chrono::Local::now().time()).num_seconds() > 0
}

pub fn gen_new_expiration_datetime(expires_in: u32) -> DateTime<Local> {
    chrono::Local::now() + Duration::seconds(expires_in.into())
}

// Since you cannot spawn blocking reqwest in async context by default,
// use this wrapper inside async method
pub async fn get_current_access_token_async() -> String {
    tokio::task::spawn_blocking(get_current_access_token)
        .await
        .unwrap() // safe to unwrap, because if this fails, whole tokio runtime is fucked
}

// Purpusefully not async, since we want to wait for access token request when invalid,
// if I were to implement this asynchronously, once the access token expires,
// there will be like 10 places asking for it at the same time
pub fn get_current_access_token() -> String {
    let current_token = unsafe { CURRENT_ACCESS_TOKEN.clone() };
    match current_token {
        Some(token) if token_is_valid(&token) => token.token,
        _ => {
            let config = get_config();
            let current_user = config.get_current_user();
            let new_token = CurrentAccessToken {
                expires_at: current_user.access_token_expires_at,
                token: current_user.access_token.clone(),
            };
            if token_is_valid(&new_token) {
                let new_token_raw = new_token.token.clone();
                unsafe {
                    CURRENT_ACCESS_TOKEN = Some(new_token);
                }
                return new_token_raw;
            };

            let reqwest_client = reqwest::blocking::Client::new();
            let form_data = HashMap::from([
                ("client_id", AZURE_OAUTH_CLIENT_ID),
                ("scope", AZURE_OAUTH_SCOPE),
                ("refresh_token", &current_user.refresh_token),
                ("grant_type", "refresh_token"),
            ]);
            let resp = reqwest_client
                .post(format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    AZURE_OAUTH_TENANT
                ))
                .header(
                    reqwest::header::CONTENT_TYPE,
                    "application/x-www-form-urlencoded",
                )
                .form(&form_data)
                .send()
                .expect("Unable to refresh access token.");
            let token_resp = serde_json::from_str::<AzureOAuthTokenResp>(
                &resp.text().expect("Unable to get response body."),
            )
            .expect("Unable to parse response.");

            let new_token = CurrentAccessToken {
                expires_at: gen_new_expiration_datetime(token_resp.expires_in),
                token: token_resp.access_token,
            };
            let new_token_raw = new_token.token.clone();
            unsafe {
                CURRENT_ACCESS_TOKEN = Some(new_token.clone());
            }

            let mut new_config = config.clone();
            let mut user_to_edit = new_config
                .user_accounts
                .iter_mut()
                .find(|s| s.id == new_config.active_user_account_id)
                .unwrap();
            user_to_edit.access_token = new_token.token;
            user_to_edit.access_token_expires_at = new_token.expires_at;
            println!("{:?}", new_config);
            save_config(&new_config);

            new_token_raw
        }
    }
}
