use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: Option<i64>,
    pub name: String,
    pub login_id: String,
    pub password: String,
    pub url: Option<String>,
}
