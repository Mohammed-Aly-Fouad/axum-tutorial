use axum::{routing::get, Router};
use tower_http::services::ServeDir;

use crate::handlers::{public::home, todos::{create_todos_handler, todos_handler}, auth::{log_in_handler, sign_up_handler}};


pub fn router () -> Router {
    let server_dir = ServeDir::new("static");
    let app = Router::new()
        .route("/create", get(create_todos_handler))
        .route("/", get(home))
        .route("/log-in", get(log_in_handler))
        .route("/sign-up", get(sign_up_handler))
        .route("/todos", get(todos_handler))
        .nest_service("/static", server_dir);
    
    app

}