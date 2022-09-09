use axum::{response::IntoResponse, Json};
// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// pub struct Login {
//     username: String,
//     password: String,
// }
use types::auth::login::Resquest;

pub async fn handle(Json(login): Json<Resquest>) -> impl IntoResponse {
    println!("{}, {}", login.username, login.password);
    format!("{} 登录成功", login.username)
}
