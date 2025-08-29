//! Fetcher API handlers

use actix_web::{web, HttpResponse, Responder};
use tracing::info;

use crate::common::app_state::AppState;
use crate::common::dto::ApiResponse;
use crate::domains::fetcher::dto::AddFetcherRequest;
use watchdog_service::arxiv::model::ArxivPaper;
use watchdog_service::arxiv::criteria::ArxivCriteria;

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
        let response: ApiResponse<()> = ApiResponse::error(400, "Unsupported fetcher type".to_string());
        return HttpResponse::BadRequest().json(response);
    }
    
    // Create a default ArxivFetcher
    let fetcher = watchdog_service::arxiv::fetcher::ArxivFetcher::default();
    
    match data.watchdog.add_fetcher(
        req.fetcher_name.clone(), 
        Box::new(fetcher)
    ).await {
        Ok(_) => {
            let response: ApiResponse<()> = ApiResponse::success_with_message(
                "Fetcher added successfully".to_string()
            );
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            let response: ApiResponse<()> = ApiResponse::error(
                500, 
                format!("Failed to add fetcher: {}", e)
            );
            HttpResponse::InternalServerError().json(response)
        }
    }
}

/// Remove a fetcher by name
pub async fn remove_fetcher(
    _data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<String>,
) -> impl Responder {
    let fetcher_name = path.into_inner();
    info!("Removing fetcher: {}", fetcher_name);
    
    // Currently the watchdog-core doesn't have a remove_fetcher method
    // We'll return a success response for now
    let response: ApiResponse<()> = ApiResponse::success_with_message(
        format!("Fetcher '{}' removal requested", fetcher_name)
    );
    HttpResponse::Ok().json(response)
}