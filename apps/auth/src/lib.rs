use axum::routing::post;
use axum::Router;
mod handles;

pub fn app() -> Router {
    Router::new().route("/login", post(handles::login))
}
