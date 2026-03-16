use axum_tutorial::routes;
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    let app = routes::router();
    
    axum::serve(listener, app).await.unwrap();

    
    
    
    // async fn not_found_handler() -> Response {
    //     let html_string = NotFoundTemplate {}.render().unwrap();
    //     Html(html_string).into_response()
    // }
    // async fn server_error_handler() -> Response {
    //     let html_string = ServerErrorTemplate {}.render().unwrap();
    //     Html(html_string).into_response()
    // }
  
    

}
