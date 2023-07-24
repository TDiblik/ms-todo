use std::collections::HashMap;

use reqwest::Method;

use crate::{
    config,
    constants::{
        graph_api_query, AZURE_OAUTH_CLIENT_ID, AZURE_OAUTH_REDIRECT_URI, AZURE_OAUTH_SCOPE,
        AZURE_OAUTH_STATE, AZURE_OAUTH_TENANT, TOP_N_TASKS_TO_FETCH
    },
    utils::authed_req_async, models::{CommandResult, TaskListsGraphResponse, TaskList, Task, TaskGraphResponse, TasksMap},
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

type GetTaskListsResponse = CommandResult<Vec<TaskList>>;
#[tauri::command]
pub async fn get_task_lists() -> GetTaskListsResponse {
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

    let mut new_config = get_config();
    let mut user_acc = new_config.get_current_user();
    user_acc.task_lists = task_lists.value.clone();
    config::save_config(&new_config);

    CommandResult::new_success(task_lists.value)
}
#[tauri::command]
pub async fn get_task_lists_cached() -> GetTaskListsResponse {
    CommandResult::new_success(
        get_config()
        .get_current_user()
        .task_lists
        .clone()
    )
}

type GetTasksResponse = CommandResult<HashMap<String, Vec<Task>>> ;
#[tauri::command]
pub async fn get_tasks_by_list_ids(ids: Vec<String>) -> GetTasksResponse {
    // TODO: Check after like three months, check whether TODO service limits changed (https://learn.microsoft.com/en-us/graph/throttling-limits#tasks-and-plans-service-limits) --- This is how to implement it in parrallel. For some reason, Microsoft returns status code 429 every time I make more than one request at a time. Once To Do service limits stabilize, switch to the commented-out implementation for X times faster cache refresh.
    // let mut task_requests = vec![];
    // for id in ids {
    //     task_requests.push(
    //         authed_req_async(Method::GET, graph_api_query(format!("me/todo/lists/{}/tasks?$top={TOP_N_TASKS_TO_FETCH}", id).as_str())).await
    //             .send()
    //             .and_then(|s| async move { Ok((id, s)) })
    //     );
    // }
    // let finished_task_requests: Vec<Result<(String, reqwest::Response), reqwest::Error>>  = futures::future::join_all(task_requests).await;
    // let mut tasks_map: TasksMap = TasksMap::new();
    // for request_result in finished_task_requests {
    //     let Ok(tasks) = request_result else {
    //         return CommandResult::new_err("Unable to send request for task lists retrieval.");
    //     };
    //     let list_id = tasks.0;
    //     if tasks.1.status() != 200 {
    //         return CommandResult::new_err(format!("Task retrieval for list {list_id} return status other than 200.").as_str());
    //     }
    //     let Ok(tasks) = tasks.1.text().await else {
    //         return CommandResult::new_err(format!("Task retrieval for list {list_id} unable to get body.").as_str());
    //     };
    //     let Ok(tasks) = serde_json::from_str::<TaskGraphResponse>(&tasks) else {
    //         return CommandResult::new_err(format!("Unable to parse tasks for list {list_id}").as_str());
    //     };
    //     tasks_map.insert(list_id, tasks.value);
    // }

    let mut tasks_map: TasksMap = TasksMap::new();
    for id in ids {
        let Ok(tasks) = authed_req_async(Method::GET, graph_api_query(format!("me/todo/lists/{id}/tasks?$top={TOP_N_TASKS_TO_FETCH}").as_str()))
            .await
            .send()
            .await 
        else {
            return CommandResult::new_err("Unable to send request for task lists retrieval.");
        };
            
        if tasks.status() != 200 {
            return CommandResult::new_err(format!("Task retrieval for list {id} return status other than 200.").as_str());
        }
        let Ok(tasks) = tasks.text().await else {
            return CommandResult::new_err(format!("Task retrieval for list {id} unable to get body.").as_str());
        };
        let Ok(tasks) = serde_json::from_str::<TaskGraphResponse>(&tasks) else {
            return CommandResult::new_err(format!("Unable to parse tasks for list {id}").as_str());
        };
        tasks_map.insert(id, tasks.value);
    }

    let mut new_config = get_config();
    let mut user_acc = new_config.get_current_user();
    user_acc.tasks = tasks_map.clone();
    config::save_config(&new_config);

    CommandResult::new_success(tasks_map)
}
#[tauri::command]
pub async fn get_tasks_by_list_ids_cached(ids: Vec<String>) -> GetTasksResponse {
    CommandResult::new_success(
        get_config()
        .get_current_user()
        .tasks.iter()
        .filter(|s| ids.contains(s.0)).map(|s| (s.0.clone(), s.1.clone()))
        .collect()
    )
}