//! Watchdog API Server
//!
//! This crate provides a REST API server for the Watchdog subscription system,
//! following a clean architecture pattern with Actix-web.

pub mod app;
pub mod common;
pub mod domains;

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};
    use serde_json::json;

    use crate::app::app_config;
    use crate::common::bootstrap::bootstrap_app;

    #[actix_web::test]
    async fn test_health_check() {
        let app_state = bootstrap_app().await;
        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(app_config)).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_subscription() {
        let app_state = bootstrap_app().await;
        let app =
            test::init_service(App::new().app_data(app_state.clone()).configure(app_config)).await;

        let payload = json!({
            "user_id": "test_user",
            "criteria_id": "test_criteria",
            "keywords": ["test", "keyword"]
        });

        let req = test::TestRequest::post()
            .uri("/api/v1/subscriptions")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
