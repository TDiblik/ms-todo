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
