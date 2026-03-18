use axum::{
    Form,
    response::{Html, IntoResponse, Redirect, Response},
};

use crate::models::{
    templates::{LogInTemplate, SignUpTemplate},
    user_form_model::AuthFormModel,
};

use askama::Template;
pub async fn sign_up_handler() -> Response {
    let html_string = SignUpTemplate {}.render().unwrap();
    Html(html_string).into_response()
}

use validator::{Validate};
pub async fn post_sign_up_handler(Form(user_form): Form<AuthFormModel>) -> Response {
    match user_form.validate() {
        Ok(_) => {
           Redirect::to("/").into_response() 
        } ,

        Err(errs) => {
            let errs = errs.to_string();
            tracing::info!("{}", errs);
            Redirect::to("/").into_response()
        } 
    }
}
// tracing::info!(
//     "Email is {} and the password is {}",
//     user_form.email,
//     user_form.password
// );

pub async fn log_in_handler() -> Response {
    let html_string = LogInTemplate {}.render().unwrap();
    Html(html_string).into_response()
}
