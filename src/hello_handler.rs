use actix_web::{Responder, HttpResponse};
use crate::hello_response::HelloResponse;

pub async fn hello() -> impl Responder {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
    };
    HttpResponse::Ok().json(response) // Serialize struct to JSON and return the response
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, http::StatusCode};

    #[actix_web::test]
    async fn unit_test() {
        // Create a test app with the hello handler
        let app = test::init_service(App::new().route("/", actix_web::web::get().to(hello))).await;

        // Send a GET request to the root URL
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        // Check that the response status is OK (200)
        assert_eq!(resp.status(), StatusCode::OK);

        // Optionally, check the response body
        let body = test::read_body(resp).await;
        let expected_body = serde_json::json!({"message": "Hello, World!"});
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_body);
    }
}