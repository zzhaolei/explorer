use axum::routing::post;
use axum::Router;
mod handler;

pub fn app() -> Router {
    Router::new().route("/login", post(handler::login))
}
