//! Subscription API routes
//!
//! This module defines the routes for subscription management.

use actix_web::web;

use crate::domains::subscription::api::handlers::{create_subscription, remove_subscription};

/// Configure subscription routes
pub fn subscription_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/subscriptions", web::post().to(create_subscription))
        .route("/subscriptions/{id}", web::delete().to(remove_subscription));
}