use crate::constants::{
    AZURE_OAUTH_CLIENT_ID, AZURE_OAUTH_REDIRECT_URI, AZURE_OAUTH_SCOPE, AZURE_OAUTH_STATE,
    AZURE_OAUTH_TENANT,
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
