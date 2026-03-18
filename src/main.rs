use axum_tutorial::{init, routes};


#[tokio::main]
async fn main() {
    let address = "127.0.0.1:8000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Fail to bind address");
    
    init::logging();
    
    init::database_connection().await;
    
    tracing::info!("Server is starting...");
    tracing::info!("Listening at {}", address);
    // tracing::debug!("testing");
    
    let app = routes::router();
    
    axum::serve(listener, app).await.expect("Failed to start the server");

    
    
    // async fn not_found_handler() -> Response {
    //     let html_string = NotFoundTemplate {}.render().unwrap();
    //     Html(html_string).into_response()
    // }
    // async fn server_error_handler() -> Response {
    //     let html_string = ServerErrorTemplate {}.render().unwrap();
    //     Html(html_string).into_response()
    // }
  
    

}
