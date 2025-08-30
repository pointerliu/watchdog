//! Fetcher API handlers

use actix_web::{web, HttpResponse, Responder};
use tracing::info;

use crate::common::app_state::AppState;
use crate::common::dto::ApiResponse;
use crate::common::utils::check_duplicate_name;
use crate::domains::fetcher::dto::{AddFetcherRequest, RemoveFetcherRequest};
use watchdog_service::arxiv::criteria::ArxivCriteria;
use watchdog_service::arxiv::fetcher::ArxivFetcherBuilder;
use watchdog_service::arxiv::model::ArxivPaper;

/// Get available fetcher types
pub async fn get_fetcher_types(
    _data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
) -> impl Responder {
    info!("Getting available fetcher types");

    // For now, we only support ArxivFetcher
    let fetcher_types = vec!["ArxivFetcher".to_string()];

    let response = ApiResponse::success(fetcher_types);
    HttpResponse::Ok().json(response)
}

pub async fn get_user_fetchers(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Getting fetchers for user {}", user_id);

    match data.watchdog.get_user_fetchers(&user_id).await {
        Ok(fetcher_names) => {
            let response: ApiResponse<_> = ApiResponse::success(fetcher_names);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to add fetcher: {}", e));
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/// Add a new fetcher
pub async fn add_fetcher(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    req: web::Json<AddFetcherRequest>,
) -> impl Responder {
    info!(
        "Adding fetcher for user: {}, name: {}, type: {}",
        req.user_id, req.fetcher_name, req.fetcher_type
    );

    // Currently only support ArxivFetcher
    if req.fetcher_type != "ArxivFetcher" {
        let response: ApiResponse<()> =
            ApiResponse::error(400, "Unsupported fetcher type".to_string());
        return HttpResponse::BadRequest().json(response);
    }

    // Check if fetcher with the same name already exists
    match data.watchdog.get_user_fetchers(&req.user_id).await {
        Ok(existing_fetchers) => {
            if let Some(response) =
                check_duplicate_name(&existing_fetchers, &req.fetcher_name, "Fetcher")
            {
                return response;
            }
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to check existing fetchers: {}", e));
            return HttpResponse::InternalServerError().json(response);
        }
    }

    // Check if subscription has been registered.
    let registered_subscription = {
        if data
            .watchdog
            .get_user_subscriptions(&req.user_id)
            .await
            .contains(&req.subscription_id)
        {
            data.watchdog
                .get_subscription_by_id(&req.subscription_id)
                .await
        } else {
            None
        }
    };

    if registered_subscription.is_none() {
        let response: ApiResponse<()> = ApiResponse::error(
            500,
            format!("Subscription ID: {} not exists", &req.subscription_id),
        );
        return HttpResponse::InternalServerError().json(response);
    }

    let registered_subscription = registered_subscription.unwrap();

    let query = registered_subscription.criteria.keywords.join(" ");
    let fetcher = ArxivFetcherBuilder::default().query(query).build().unwrap();

    match data
        .watchdog
        .add_fetcher(&req.user_id, &req.fetcher_name, Box::new(fetcher))
        .await
    {
        Ok(_) => {
            let response: ApiResponse<()> =
                ApiResponse::success_with_message("Fetcher added successfully".to_string());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to add fetcher: {}", e));
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/// Remove a fetcher by name
pub async fn remove_fetcher(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<RemoveFetcherRequest>,
) -> impl Responder {
    let req = path.into_inner();
    info!(
        "Removing fetcher {} for user {}",
        req.fetcher_name, req.user_id
    );

    match data
        .watchdog
        .remove_fetcher(&req.user_id, &req.fetcher_name)
        .await
    {
        Ok(_) => {
            let response: ApiResponse<()> =
                ApiResponse::success_with_message("Fetcher removed successfully".to_string());
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> =
                ApiResponse::error(500, format!("Failed to remove fetcher: {}", e));
            HttpResponse::InternalServerError().json(response)
        }
    }
}
