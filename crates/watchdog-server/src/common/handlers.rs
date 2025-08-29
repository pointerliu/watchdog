//! Common HTTP handlers
//!
//! This module contains handlers for common endpoints like health checks.

use crate::common::dto::ApiResponse;
use actix_web::{HttpResponse, Responder};

/// Health check endpoint
pub async fn health_check() -> impl Responder {
    let response: ApiResponse<()> =
        ApiResponse::success_with_message("Watchdog API server is running".to_string());
    HttpResponse::Ok().json(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().route("/", web::get().to(health_check))).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
