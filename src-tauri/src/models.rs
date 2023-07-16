use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserInfoGraphResponse {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub mail: String,
}
