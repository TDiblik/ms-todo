#[cfg(debug_assertions)]
pub const AZURE_OAUTH_CLIENT_ID: &str = "97759a74-5940-47c4-96a7-488b0e66aee1";
#[cfg(not(debug_assertions))]
pub const AZURE_OAUTH_CLIENT_ID: &str = "TODO";

pub const AZURE_OAUTH_TENANT: &str = "common";
pub const AZURE_OAUTH_SCOPE: &str = "offline_access Tasks.ReadWrite User.Read";
pub const AZURE_OAUTH_STATE: &str = "12345";
pub const AZURE_OAUTH_DEEP_LINK_NAME: &str = "ms-todo-unofficial-tomasdiblik-cz";
pub const AZURE_OAUTH_SCHEMA_NAME: &str = "://auth/";
pub const AZURE_OAUTH_REDIRECT_URI: &str = "ms-todo-unofficial-tomasdiblik-cz://auth/";

pub const MICROSOFT_GRAPH_API_URL: &str = "https://graph.microsoft.com/v1.0/";
// pub const MICROSOFT_GRAPH_BETA_API_URL: &str = "https://graph.microsoft.com/beta/";

pub fn graph_api_query(endpoint: &str) -> String {
    format!("{MICROSOFT_GRAPH_API_URL}{endpoint}")
}

// If you have more than 1_000_000 tasks, you have a problem xdd (1_000_000 tasks / 20 tasks per day = +-137 years)
pub const TOP_N_TASKS_TO_FETCH: u32 = 1_000_000;
