use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use tauri::api::path::home_dir;

use crate::models::{TaskList, TasksMap};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub active_user_account_id: String, // Must always (after initial check) include valid user account id from user_accounts vector. Empty means user wants to change account.
    pub user_accounts: Vec<UserAccount>,
}
impl Config {
    pub fn get_current_user(&mut self) -> &mut UserAccount {
        self.user_accounts
            .iter_mut()
            .find(|s| s.id == self.active_user_account_id)
            .unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAccount {
    pub id: String,
    pub display_name: String,
    pub mail: String,
    pub access_token: String,
    pub access_token_expires_at: DateTime<Local>,
    pub refresh_token: String,
    pub profile_photo: Option<String>,
    pub task_lists: Vec<TaskList>,
    pub tasks: TasksMap,
}

fn get_config_dir_path() -> PathBuf {
    let mut config_dir = home_dir().expect("Unable to get user's home directory");
    config_dir.push(".config/ms-todo-unofficial-tomasdiblik-cz/");
    config_dir
}

fn get_config_path() -> PathBuf {
    let mut config_path = get_config_dir_path();
    config_path.push("config.json");
    config_path
}

pub fn get_config() -> Config {
    let config_dir = get_config_dir_path();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Unable to create user's config directory");
    }

    let config_path = get_config_path();
    if !config_path.exists() {
        save_config(&Config::default());
    };

    let config_file = File::open(config_path).expect("Unable to open user's config file");
    let config_buf = BufReader::new(config_file);

    serde_json::from_reader(config_buf).expect("Unable to parse config file")
}

pub fn save_config(new_config: &Config) {
    fs::write(
        get_config_path(),
        serde_json::to_string(new_config).expect("Unable to serialize config"),
    )
    .expect("Unable to write config file")
}
