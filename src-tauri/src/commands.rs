use reqwest::Method;

use crate::{
    config,
    constants::{
        graph_api_query, AZURE_OAUTH_CLIENT_ID, AZURE_OAUTH_REDIRECT_URI, AZURE_OAUTH_SCOPE,
        AZURE_OAUTH_STATE, AZURE_OAUTH_TENANT,
    },
    token::get_current_access_token,
    utils::{authed_req, authed_req_async}, models::{CommandResult, TaskListsGraphResponse, TaskLists},
};

#[tauri::command]
pub fn get_login_url() -> String {
    let mut login_url = String::from("https://login.microsoftonline.com/");
    login_url.push_str(AZURE_OAUTH_TENANT);
    login_url.push_str("/oauth2/v2.0/authorize?client_id=");
    login_url.push_str(AZURE_OAUTH_CLIENT_ID);
    login_url.push_str("&response_type=code&redirect_uri=");
    login_url.push_str(AZURE_OAUTH_REDIRECT_URI);
    login_url.push_str("&response_mode=query&scope=");
    login_url.push_str(AZURE_OAUTH_SCOPE);
    login_url.push_str("&state=");
    login_url.push_str(AZURE_OAUTH_STATE);

    login_url
}

#[tauri::command]
pub fn initial_check() -> String {
    let config = get_config();

    if config.active_user_account_id.is_empty() {
        return "send-to-login".to_owned();
    }

    // TODO: Check if refresh token is valid and if not, return to login once again.

    "send-to-app".to_owned()
}

#[tauri::command]
pub fn get_config() -> config::Config {
    config::get_config()
}

#[tauri::command]
pub fn login_manual(user_id: String) {
    let mut new_config = get_config();
    new_config.active_user_account_id = user_id;
    config::save_config(&new_config);
}

#[tauri::command]
pub fn logout() {
    let mut new_config = get_config();
    new_config.active_user_account_id = "".to_owned();
    config::save_config(&new_config);
}

#[tauri::command]
pub async fn get_task_lists() -> CommandResult<Vec<TaskLists>> {
    let Ok(task_lists) = authed_req_async(Method::GET, graph_api_query("me/todo/lists")).await
        .send()
        .await 
    else {
        return CommandResult::new_err("Unable to send request for task lists retrieval.");
    };
    if task_lists.status() != 200 {
        return CommandResult::new_err("Task lists retrieval return status other than 200.");
    }
    let Ok(task_lists) = task_lists.text().await else {
        return CommandResult::new_err("Task lists retrieval unable to get body.");
    };
    let Ok(task_lists) = serde_json::from_str::<TaskListsGraphResponse>(&task_lists) else {
        return CommandResult::new_err("Unable to parse task_lists");
    };

    CommandResult::new_success(task_lists.value)
}
