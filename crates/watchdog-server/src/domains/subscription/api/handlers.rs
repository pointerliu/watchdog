//! Subscription API handlers
//!
//! This module contains the Actix-web handlers for subscription management endpoints.

use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use tracing::info;

use crate::common::app_state::AppState;
use crate::common::dto::ApiResponse;
use crate::common::utils::check_duplicate_name;
use crate::domains::subscription::dto::{CreateSubscriptionRequest, SubscriptionResponse};
use watchdog_core::subscription::Subscription;
use watchdog_service::arxiv::criteria::ArxivCriteria;
use watchdog_service::arxiv::model::ArxivPaper;

/// Get user's current subscriptions
pub async fn get_user_subscriptions(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Getting subscriptions for user: {}", user_id);

    let data = data.watchdog.get_user_subscriptions(&user_id).await;
    let api_response = ApiResponse::success(data);
    HttpResponse::Ok().json(api_response)
}

/// Create a new subscription
pub async fn create_subscription(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    req: web::Json<CreateSubscriptionRequest>,
) -> impl Responder {
    info!(
        "Creating subscription for user: {}, name: {}",
        req.user_id, req.subscription_id
    );

    // Check if subscription with the same ID already exists
    let existing_subscription_ids = data.watchdog.get_user_subscriptions(&req.user_id).await;
    
    if let Some(response) = check_duplicate_name(&existing_subscription_ids, &req.subscription_id, "Subscription") {
        return response;
    }

    let criteria = ArxivCriteria::new(req.subscription_id.clone(), req.keywords.clone());
    let subscription = Subscription::new(req.user_id.clone(), criteria);

    match data.watchdog.add_subscription(subscription).await {
        Ok(_) => {
            let response = SubscriptionResponse {
                user_id: req.user_id.clone(),
                subscription_id: req.subscription_id.clone(),
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

#[derive(Deserialize)]
pub struct RemoveSubscriptionRequest {
    pub user_id: String,
    pub subscription_id: String,
}

/// Remove a subscription by name
pub async fn remove_subscription(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<RemoveSubscriptionRequest>,
) -> impl Responder {
    let req = path.into_inner();
    info!(
        "Removing subscription {} for user {}",
        req.subscription_id, req.user_id
    );

    match data
        .watchdog
        .remove_subscription(&req.user_id, &req.subscription_id)
        .await
    {
        Ok(_) => {
            let response: ApiResponse<()> =
                ApiResponse::success_with_message("Subscription removed successfully".to_string());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to remove subscription: {}", e));
            HttpResponse::InternalServerError().json(response)
        }
    }
}
