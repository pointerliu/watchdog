use actix_web::{test, App};

use watchdog_server::app::app_config;
use watchdog_server::common::bootstrap::bootstrap_app;

#[actix_web::test]
async fn test_fetcher_endpoints() {
    // Bootstrap the application state
    let app_state = bootstrap_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(app_config)
    ).await;

    // Test fetcher types endpoint
    let req = test::TestRequest::get().uri("/api/v1/fetchers/types").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_notifier_endpoints() {
    // Bootstrap the application state
    let app_state = bootstrap_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(app_config)
    ).await;

    // Test notifier types endpoint
    let req = test::TestRequest::get().uri("/api/v1/notifiers/types").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_subscription_endpoints() {
    // Bootstrap the application state
    let app_state = bootstrap_app().await;

    let app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .configure(app_config)
    ).await;

    // Test subscription endpoint
    let req = test::TestRequest::get().uri("/api/v1/subscriptions/user/test_user").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}