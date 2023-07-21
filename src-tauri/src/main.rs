// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod constants;
mod models;
mod utils;
mod token;
mod config;

use std::collections::HashMap;

use base64::Engine;
use config::save_config;
use constants::AZURE_OAUTH_DEEP_LINK_NAME;
use tauri::Manager;

use crate::{
    constants::{
        AZURE_OAUTH_CLIENT_ID, AZURE_OAUTH_REDIRECT_URI, AZURE_OAUTH_SCHEMA_NAME,
        AZURE_OAUTH_SCOPE, AZURE_OAUTH_STATE, AZURE_OAUTH_TENANT,
    }, token::{AzureOAuthTokenResp, gen_new_expiration_datetime}, models::UserInfoGraphResponse, config::{get_config, UserAccount},
};

fn main() {
    tauri_plugin_deep_link::prepare("ms-todo.tomasdiblik.cz"); // Should be equal to `tauri.conf.json > tauri > bundle > identifier` field

    tauri::Builder::default()
        // Handle auth
        // This part of code does not use CommandResult<T> because it was written before CommandResult<T> was implemented.
        // It works, and changes are there will never be need to touch this ever again, 
        // plus it's kinda isolated from rest of the app, I didn't bothered with rewriting it for these reason.
        .setup(|app| {
            let handle = app.handle();
            tauri_plugin_deep_link::register(
                AZURE_OAUTH_DEEP_LINK_NAME,
                move |auth_callback_response| {
                    let parts_of_response: Vec<&str> = auth_callback_response.split('&').collect();

                    let state_code = parts_of_response[1].replace("state=", "");
                    if state_code != AZURE_OAUTH_STATE {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.unable-to-parse-callback",
                                "callback_response": auth_callback_response
                            }),
                        );
                        return;
                    }

                    let temp_auth_code = parts_of_response[0]
                        .replace(AZURE_OAUTH_DEEP_LINK_NAME, "")
                        .replace(AZURE_OAUTH_SCHEMA_NAME, "")
                        .replace("?code=", "");

                    let reqwest_client = reqwest::blocking::Client::new();

                    let form_data = HashMap::from([
                        ("client_id", AZURE_OAUTH_CLIENT_ID),
                        ("scope", AZURE_OAUTH_SCOPE),
                        ("code", &temp_auth_code),
                        ("redirect_uri", AZURE_OAUTH_REDIRECT_URI),
                        ("grant_type", "authorization_code"),
                    ]);
                    let Ok(resp) = reqwest_client
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
                    else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.token-req-response-did-err",
                            }),
                        );
                        return;
                    };
                    if resp.status() != 200 {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.token-req-response-not-200",
                                "resp_status": resp.status().as_u16()
                            }),
                        );
                        return;
                    }
                    let Ok(body) = resp.text() else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.token-req-response-unable-to-get-body",
                            }),
                        );
                        return;
                    };
                    let Ok(token) = serde_json::from_str::<AzureOAuthTokenResp>(&body) else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.response-unable-to-parse-body",
                            }),
                        );
                        return;
                    };
                    let Ok(resp) = reqwest_client
                        .get("https://graph.microsoft.com/v1.0/me")
                        .header("Authorization", format!("Bearer {}", token.access_token)).send() 
                    else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.user-info-resp-did-err",
                            }),
                        );
                        return;
                    };
                    if resp.status() != 200 {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.user-info-response-not-200",
                                "resp_status": resp.status().as_u16()
                            }),
                        );
                        return;
                    }
                    let Ok(body) = resp.text() else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.user-info-response-unable-to-get-body",
                            }),
                        );
                        return;
                    };
                    let Ok(user_info) = serde_json::from_str::<UserInfoGraphResponse>(&body) else {
                        _ = handle.emit_all(
                            "app://login-request-error",
                            serde_json::json!({
                                "msg": "login.user-info-unable-to-parse-body",
                            }),
                        );
                        return;
                    };
                    
                    
                    let mut user_profile_picture: Option<String> = None;
                    if let Ok(resp) = reqwest_client
                        .get("https://graph.microsoft.com/v1.0/me/photo/$value")
                        .header("Authorization", format!("Bearer {}", token.access_token)).send() 
                        {
                            if let Some(content_type) = resp.headers().clone().get("content-type").and_then(|s| s.to_str().ok()) {
                                if let Ok(bytes) = resp.bytes() {
                                    user_profile_picture = Some(
                                        format!("data:{};base64,{}",
                                            content_type,
                                            base64::engine::general_purpose::STANDARD.encode(bytes)
                                        )
                                    );
                                }
                            }
                        }

                    let mut config = get_config();
                    if let Some(position_to_delete) = config.user_accounts.iter().position(|s| s.id == user_info.id) {
                        config.user_accounts.remove(position_to_delete);
                    }
                    let new_user_account = UserAccount {
                        id: user_info.id,
                        display_name: user_info.display_name,
                        mail: user_info.mail,
                        access_token: token.access_token,
                        access_token_expires_at: gen_new_expiration_datetime(token.expires_in),
                        refresh_token: token.refresh_token,
                        profile_photo: user_profile_picture,
                    };
                    config.active_user_account_id = new_user_account.id.clone();
                    config.user_accounts.push(new_user_account);
                    save_config(&config);

                    _ = handle.emit_all("app://login-request-success", auth_callback_response);
                },
            )
            .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_login_url, 
            commands::initial_check, 
            commands::get_config, 
            commands::login_manual, 
            commands::logout,
            commands::get_task_lists
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
