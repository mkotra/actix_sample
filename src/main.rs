mod config;
mod hello_response;
mod hello_handler;

use actix_web::{web, App, HttpServer};
use config::Settings;
use hello_handler::hello; // Import the hello handler

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Load configuration
    let settings = Settings::new().expect("Failed to load configuration");

    // Start server with configuration host and port
    println!("Starting server on {}:{}", settings.host, settings.port);

    HttpServer::new(|| {
        App::new().route(
            "/", web::get().to(hello)
        ) })
        .bind(format!("{}:{}", settings.host, settings.port))?
        .run()
        .await
}

// Integration tests
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;

    #[actix_web::test]
    async fn integration_test() {
        // Create a test app
        let app = test::init_service(App::new().route("/", web::get().to(hello))).await;

        // Send a GET request to the root URL
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        // Check that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);

        // Optionally, check the response body
        let body = test::read_body(resp).await;
        let expected_body = json!({"message": "Hello, World!"});
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_body);
    }
}
