use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub err_message: Option<String>,
    pub result: Option<T>,
}

impl<T> CommandResult<T>
where
    T: Default + DeserializeOwned + Serialize,
{
    pub fn new_err(msg: &str) -> CommandResult<T> {
        CommandResult {
            success: false,
            err_message: Some(String::from(msg)),
            result: None,
        }
    }

    pub fn new_success(result: T) -> CommandResult<T> {
        CommandResult {
            success: true,
            err_message: None,
            result: Some(result),
        }
    }
}

#[derive(Deserialize)]
pub struct UserInfoGraphResponse {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub mail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskListsGraphResponse {
    pub value: Vec<TaskList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskList {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "wellknownListName")]
    pub well_known_list_name: String,
}
