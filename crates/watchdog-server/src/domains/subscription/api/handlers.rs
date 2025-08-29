//! Subscription API handlers
//!
//! This module contains the Actix-web handlers for subscription management endpoints.

use actix_web::{web, HttpResponse, Responder};
use tracing::info;

use crate::common::app_state::AppState;
use crate::common::bootstrap::{FetchedDataStorage, SimpleSubscriptionCriteria};
use crate::common::dto::ApiResponse;
use crate::domains::subscription::dto::subscription_dto::{
    CreateSubscriptionRequest, SubscriptionResponse,
};
use watchdog_core::subscription::Subscription;

/// Create a new subscription
pub async fn create_subscription(
    data: web::Data<AppState<String, SimpleSubscriptionCriteria, FetchedDataStorage<String>>>,
    req: web::Json<CreateSubscriptionRequest>,
) -> impl Responder {
    info!(
        "Creating subscription for user: {}, criteria: {}",
        req.user_id, req.criteria_id
    );

    let criteria = SimpleSubscriptionCriteria::new(req.criteria_id.clone(), req.keywords.clone());
    let subscription = Subscription::new(req.user_id.clone(), criteria);

    match data.watchdog.add_subscription(subscription).await {
        Ok(_) => {
            let response = SubscriptionResponse {
                user_id: req.user_id.clone(),
                criteria_id: req.criteria_id.clone(),
            };
            let api_response = ApiResponse::success(response);
            HttpResponse::Ok().json(api_response)
        }
        Err(e) => {
            let api_response: ApiResponse<SubscriptionResponse> =
                ApiResponse::error(500, format!("Failed to create subscription: {}", e));
            HttpResponse::InternalServerError().json(api_response)
        }
    }
}

/// Remove a subscription by criteria ID
pub async fn remove_subscription(
    data: web::Data<AppState<String, SimpleSubscriptionCriteria, FetchedDataStorage<String>>>,
    path: web::Path<String>,
) -> impl Responder {
    let criteria_id = path.into_inner();
    info!("Removing subscription with criteria ID: {}", criteria_id);

    match data.watchdog.remove_subscription(&criteria_id).await {
        Ok(Some(_)) => {
            let response: ApiResponse<()> =
                ApiResponse::success_with_message("Subscription removed successfully".to_string());
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            let response: ApiResponse<()> =
                ApiResponse::success_with_message("Subscription not found".to_string());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to remove subscription: {}", e));
            HttpResponse::InternalServerError().json(response)
        }
    }
}
