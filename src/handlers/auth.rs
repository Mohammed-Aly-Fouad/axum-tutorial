use axum::{Form, response::{Html, IntoResponse, Redirect, Response}};

use crate::models::{templates::{LogInTemplate, SignUpTemplate}, user_form_model::UserFormModel};

use askama::Template;
 pub async fn sign_up_handler() -> Response {
        let html_string = SignUpTemplate {}.render().unwrap();
        Html(html_string).into_response()
    }
pub async fn post_sign_up_handler(Form(user_form): Form<UserFormModel>) -> Response {
    tracing::info!("Email is {} and the password is {}", user_form.email, user_form.password);
    Redirect::to("/").into_response()
}  
    pub async fn log_in_handler() -> Response {
            let html_string = LogInTemplate {}.render().unwrap();
            Html(html_string).into_response()
        }