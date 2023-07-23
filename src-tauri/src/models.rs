use std::collections::HashMap;

use chrono::Utc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/* ----------- Related to communication between rust and node ----------- */
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

/* ----------- Generic Microsoft graph API objects ----------- */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateTimeTimeZoneGraphObj {
    #[serde(rename = "dateTime")]
    date_time: String,
    #[serde(rename = "timeZone")]
    time_zone: String,
}

/* ----------- Related to Microsoft graph API communication ----------- */
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskList {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "wellknownListName")]
    pub well_known_list_name: String,
}

pub type TasksMap = HashMap<String, Vec<Task>>;
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGraphResponse {
    pub value: Vec<Task>,
}
// TODO: add `recurrence` https://learn.microsoft.com/en-us/graph/api/resources/todotask?view=graph-rest-1.0#properties
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub status: String,
    pub importance: String,
    pub categories: Vec<String>,
    pub title: String,
    pub body: TaskBody,
    #[serde(rename = "bodyLastModifiedDateTime")]
    pub body_last_modified_date_time: Option<chrono::DateTime<Utc>>,

    #[serde(rename = "checklistItems")]
    pub checklist_items: Option<Vec<ChecklistItem>>,

    #[serde(rename = "createdDateTime")]
    pub created_date_time: chrono::DateTime<Utc>,
    #[serde(rename = "startDateTime")]
    pub start_date_time: Option<DateTimeTimeZoneGraphObj>,
    #[serde(rename = "completedDateTime")]
    pub completed_date_time: Option<DateTimeTimeZoneGraphObj>,
    #[serde(rename = "dueDateTime")]
    pub due_date_time: Option<DateTimeTimeZoneGraphObj>,

    #[serde(rename = "hasAttachments")]
    pub has_attachments: bool,
    #[serde(rename = "isReminderOn")]
    pub is_reminder_on: bool,
    #[serde(rename = "lastModifiedDateTime")]
    pub last_modified_date_time: chrono::DateTime<Utc>,
    #[serde(rename = "reminderDateTime")]
    pub reminder_date_time: Option<DateTimeTimeZoneGraphObj>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskBody {
    pub content: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChecklistItem {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "isChecked")]
    pub is_checked: bool,
    #[serde(rename = "checkedDateTime")]
    pub checked_date_time: Option<chrono::DateTime<Utc>>,
}
