//! Subscription API routes
//!
//! This module defines the routes for subscription management.

use actix_web::web;

use crate::domains::subscription::api::handlers::{create_subscription, get_user_subscriptions, remove_subscription};

/// Configure subscription routes
pub fn subscription_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/subscriptions/{user_id}", web::get().to(get_user_subscriptions))
        .route("/subscriptions", web::post().to(create_subscription))
        .route("/subscriptions/{user_id}/{subscription_id}", web::delete().to(remove_subscription));
}
