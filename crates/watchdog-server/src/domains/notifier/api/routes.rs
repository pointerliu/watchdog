//! Notifier API routes

use actix_web::web;

use crate::domains::notifier::api::handlers::{
    add_notifier, get_notifier_types, get_user_notifiers, remove_notifier,
};

/// Configure notifier routes
pub fn notifier_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/notifiers/types", web::get().to(get_notifier_types))
        .route("/notifiers/{user_id}", web::get().to(get_user_notifiers))
        .route("/notifiers", web::post().to(add_notifier))
        .route("/notifiers/{user_id}/{notifier_name}", web::delete().to(remove_notifier));
}