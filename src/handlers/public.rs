use axum::response::{Html, IntoResponse, Response};

use crate::models::templates::{HomeTemplate};

use askama::Template;


pub async fn home() -> Response {
    let html_string = HomeTemplate {}.render().unwrap();
    Html(html_string).into_response()
}