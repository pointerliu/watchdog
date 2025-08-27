//! REST API layer for the watchdog subscription framework
//! 
//! This module provides a flexible and scalable API for managing subscriptions,
//! allowing developers to easily build subscription-based services.

use actix_web::{
    web::{self, Data, Json, Path},
    Result as ActixResult, Scope, HttpRequest,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use watchdog_core::{
    subscription::{Subscription, SubscriptionCriteria},
    FrameworkError,
};

/// API error response
#[derive(Serialize)]
pub struct ApiError {
    pub error: String,
}

impl From<FrameworkError> for ApiError {
    fn from(err: FrameworkError) -> Self {
        ApiError {
            error: err.to_string(),
        }
    }
}

/// Generic API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// Request to create a new subscription
#[derive(Deserialize, Serialize, Clone)]
pub struct CreateSubscriptionRequest<C> {
    pub user_id: String,
    pub criteria: C,
}

/// Request to update an existing subscription
#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateSubscriptionRequest<C> {
    pub criteria: C,
}

/// Response for subscription operations
#[derive(Serialize)]
pub struct SubscriptionResponse {
    pub message: String,
}

/// Response containing subscription details
#[derive(Serialize)]
pub struct SubscriptionDetailsResponse<C> {
    pub user_id: String,
    pub criteria: C,
}

/// Service trait for subscription management
/// 
/// This trait abstracts the subscription management operations, allowing
/// different implementations for different use cases.
#[async_trait::async_trait]
pub trait SubscriptionService<C: SubscriptionCriteria + Clone + Send + Sync> {
    type Error: std::error::Error + Send + Sync;

    async fn add_subscription(
        &self,
        subscription: Subscription<C>,
    ) -> Result<(), Self::Error>;

    async fn remove_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error>;

    async fn get_subscription(
        &self,
        id: &C::Id,
    ) -> Result<Option<Subscription<C>>, Self::Error>;

    async fn list_subscriptions(
        &self,
    ) -> Result<Vec<Subscription<C>>, Self::Error>;
}

/// API handler for creating a new subscription
pub async fn create_subscription<C, S>(
    _req: HttpRequest,
    service: Data<Arc<RwLock<S>>>,
    json_req: Json<CreateSubscriptionRequest<C>>,
) -> ActixResult<Json<ApiResponse<SubscriptionResponse>>>
where
    C: SubscriptionCriteria + Clone + Send + Sync + for<'de> Deserialize<'de> + Serialize + 'static,
    S: SubscriptionService<C> + Send + Sync + 'static,
{
    let subscription = Subscription::new(json_req.user_id.clone(), json_req.criteria.clone());
    
    match service.read().await.add_subscription(subscription).await {
        Ok(()) => Ok(Json(ApiResponse::success(SubscriptionResponse {
            message: "Subscription created successfully".to_string(),
        }))),
        Err(e) => Ok(Json(ApiResponse::error(format!(
            "Failed to create subscription: {}",
            e
        )))),
    }
}

/// API handler for removing a subscription
pub async fn remove_subscription<C, S>(
    _req: HttpRequest,
    service: Data<Arc<RwLock<S>>>,
    path: Path<String>,
) -> ActixResult<Json<ApiResponse<SubscriptionResponse>>>
where
    C: SubscriptionCriteria + Clone + Send + Sync + 'static,
    C::Id: From<String>,
    S: SubscriptionService<C> + Send + Sync + 'static,
{
    let id: C::Id = path.into_inner().into();
    
    match service.read().await.remove_subscription(&id).await {
        Ok(Some(_)) => Ok(Json(ApiResponse::success(SubscriptionResponse {
            message: "Subscription removed successfully".to_string(),
        }))),
        Ok(None) => Ok(Json(ApiResponse::error("Subscription not found".to_string()))),
        Err(e) => Ok(Json(ApiResponse::error(format!(
            "Failed to remove subscription: {}",
            e
        )))),
    }
}

/// API handler for getting a subscription
pub async fn get_subscription<C, S>(
    _req: HttpRequest,
    service: Data<Arc<RwLock<S>>>,
    path: Path<String>,
) -> ActixResult<Json<ApiResponse<SubscriptionDetailsResponse<C>>>>
where
    C: SubscriptionCriteria + Clone + Send + Sync + Serialize + 'static,
    C::Id: From<String>,
    S: SubscriptionService<C> + Send + Sync + 'static,
{
    let id: C::Id = path.into_inner().into();
    
    match service.read().await.get_subscription(&id).await {
        Ok(Some(subscription)) => Ok(Json(ApiResponse::success(
            SubscriptionDetailsResponse {
                user_id: subscription.user_id,
                criteria: subscription.criteria,
            },
        ))),
        Ok(None) => Ok(Json(ApiResponse::error("Subscription not found".to_string()))),
        Err(e) => Ok(Json(ApiResponse::error(format!(
            "Failed to get subscription: {}",
            e
        )))),
    }
}

/// API handler for listing all subscriptions
pub async fn list_subscriptions<C, S>(
    _req: HttpRequest,
    service: Data<Arc<RwLock<S>>>,
) -> ActixResult<Json<ApiResponse<Vec<SubscriptionDetailsResponse<C>>>>>
where
    C: SubscriptionCriteria + Clone + Send + Sync + Serialize + 'static,
    S: SubscriptionService<C> + Send + Sync + 'static,
{
    match service.read().await.list_subscriptions().await {
        Ok(subscriptions) => {
            let response: Vec<SubscriptionDetailsResponse<C>> = subscriptions
                .into_iter()
                .map(|s| SubscriptionDetailsResponse {
                    user_id: s.user_id,
                    criteria: s.criteria,
                })
                .collect();
            
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => Ok(Json(ApiResponse::error(format!(
            "Failed to list subscriptions: {}",
            e
        )))),
    }
}

/// Create a new API scope for subscription management
/// 
/// This function creates an Actix Web scope with all the subscription management endpoints.
/// It should be mounted on an Actix Web application to expose the API.
pub fn subscription_scope<C, S>() -> Scope
where
    C: SubscriptionCriteria + Clone + Send + Sync + for<'de> Deserialize<'de> + Serialize + 'static,
    C::Id: From<String>,
    S: SubscriptionService<C> + Send + Sync + 'static,
{
    web::scope("/subscriptions")
        .route("", web::post().to(create_subscription::<C, S>))
        .route("/{id}", web::delete().to(remove_subscription::<C, S>))
        .route("/{id}", web::get().to(get_subscription::<C, S>))
        .route("", web::get().to(list_subscriptions::<C, S>))
}