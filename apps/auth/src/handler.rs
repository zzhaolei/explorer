use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login {
    username: String,
    password: String,
}

pub async fn login(Json(login): Json<Login>) -> impl IntoResponse {
    println!("{}, {}", login.username, login.password);
    format!("{} 登录成功", login.username)
}
