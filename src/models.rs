use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: u16,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u16,
    pub service: u16,
    pub username: String,
    pub last_change: String,
    pub account_creation_date: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub access_token: Option<String>,
    pub pin: Option<String>,
    pub passcode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityQuestion {
    pub id: u16,
    pub account: u16,
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shortcut {
    pub id: u16,
    pub account: u16,
    pub field: String,
    pub sequence: String,
}
