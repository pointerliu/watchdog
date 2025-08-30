//! Fetcher API routes

use actix_web::web;

use crate::domains::fetcher::api::handlers::{add_fetcher, get_fetcher_types, get_user_fetchers, remove_fetcher};

/// Configure fetcher routes
pub fn fetcher_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/fetchers/types", web::get().to(get_fetcher_types))
        .route("/fetchers/{user_id}", web::get().to(get_user_fetchers))
        .route("/fetchers", web::post().to(add_fetcher))
        .route("/fetchers/{name}", web::delete().to(remove_fetcher));
}