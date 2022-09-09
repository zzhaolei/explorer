use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Resquest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Response {
    pub name: String,
    pub access_token: String,
}
