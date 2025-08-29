//! Application configuration and routing
//!
//! This module sets up the routes for all domains in the application.

use actix_web::web;

use crate::common::handlers::health_check;
use crate::domains::fetcher::api::routes;
use crate::domains::notifier::api::routes as notifier_routes;
use crate::domains::subscription::api::routes::subscription_routes;

/// Configure all application routes
pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(routes::fetcher_routes)
            .configure(notifier_routes::notifier_routes)
            .configure(subscription_routes),
    );

    // Health check endpoint
    cfg.route("/", web::get().to(health_check));
}
