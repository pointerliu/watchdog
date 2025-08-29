//! Notifier API handlers

use actix_web::{web, HttpResponse, Responder};
use tracing::info;
use std::sync::Arc;
use watchdog_core::Notifier;

use crate::common::app_state::AppState;
use crate::common::dto::ApiResponse;
use crate::domains::notifier::dto::AddNotifierRequest;
use crate::domains::notifier::dto::UserNotifierResponse;
use watchdog_service::arxiv::model::ArxivPaper;
use watchdog_service::arxiv::criteria::ArxivCriteria;
use watchdog_service::arxiv::notifier::{ArxivConsoleNotifier, ArxivEmailNotifier};

/// Get available notifier types
pub async fn get_notifier_types(
    _data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
) -> impl Responder {
    info!("Getting available notifier types");
    
    // For now, we support ArxivConsoleNotifier and ArxivEmailNotifier
    let notifier_types = vec![
        "ArxivConsoleNotifier".to_string(),
        "ArxivEmailNotifier".to_string(),
    ];
    
    let response = ApiResponse::success(notifier_types);
    HttpResponse::Ok().json(response)
}

/// Get user's current notifier
pub async fn get_user_notifier(
    _data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = path.into_inner();
    info!("Getting notifier for user: {}", user_id);
    
    // For now, we'll return a mock response
    let response = UserNotifierResponse {
        user_id,
        notifier_name: "default_notifier".to_string(),
        notifier_type: "ArxivConsoleNotifier".to_string(),
    };
    
    let api_response = ApiResponse::success(response);
    HttpResponse::Ok().json(api_response)
}

/// Add a new notifier
pub async fn add_notifier(
    data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    req: web::Json<AddNotifierRequest>,
) -> impl Responder {
    info!(
        "Adding notifier for user: {}, name: {}, type: {}",
        req.user_id, req.notifier_name, req.notifier_type
    );
    
    match req.notifier_type.as_str() {
        "ArxivConsoleNotifier" => {
            let mut notifier = ArxivConsoleNotifier::default();
            notifier.set_name(req.notifier_name.clone());
            
            match data.watchdog.add_notifier(
                req.user_id.clone(),
                Arc::new(notifier)
            ).await {
                Ok(_) => {
                    let response: ApiResponse<()> = ApiResponse::success_with_message(
                        "Console notifier added successfully".to_string()
                    );
                    HttpResponse::Ok().json(response)
                }
                Err(e) => {
                    let response: ApiResponse<()> = ApiResponse::error(
                        500, 
                        format!("Failed to add console notifier: {}", e)
                    );
                    HttpResponse::InternalServerError().json(response)
                }
            }
        }
        "ArxivEmailNotifier" => {
            if let Some(email_address) = &req.email_address {
                match ArxivEmailNotifier::new(&req.notifier_name, email_address.clone()) {
                    Ok(mut notifier) => {
                        notifier.set_name(req.notifier_name.clone());
                        
                        match data.watchdog.add_notifier(
                            req.user_id.clone(),
                            Arc::new(notifier)
                        ).await {
                            Ok(_) => {
                                let response: ApiResponse<()> = ApiResponse::success_with_message(
                                    "Email notifier added successfully".to_string()
                                );
                                HttpResponse::Ok().json(response)
                            }
                            Err(e) => {
                                let response: ApiResponse<()> = ApiResponse::error(
                                    500, 
                                    format!("Failed to add email notifier: {}", e)
                                );
                                HttpResponse::InternalServerError().json(response)
                            }
                        }
                    }
                    Err(e) => {
                        let response: ApiResponse<()> = ApiResponse::error(
                            500, 
                            format!("Failed to create email notifier: {}", e)
                        );
                        HttpResponse::InternalServerError().json(response)
                    }
                }
            } else {
                let response: ApiResponse<()> = ApiResponse::error(
                    400, 
                    "Email address is required for email notifier".to_string()
                );
                HttpResponse::BadRequest().json(response)
            }
        }
        _ => {
            let response: ApiResponse<()> = ApiResponse::error(
                400, 
                "Unsupported notifier type".to_string()
            );
            HttpResponse::BadRequest().json(response)
        }
    }
}

/// Remove a notifier by name
pub async fn remove_notifier(
    _data: web::Data<AppState<ArxivPaper, ArxivCriteria>>,
    path: web::Path<String>,
) -> impl Responder {
    let notifier_name = path.into_inner();
    info!("Removing notifier: {}", notifier_name);
    
    // Currently the watchdog-core doesn't have a remove_notifier method
    // We'll return a success response for now
    let response: ApiResponse<()> = ApiResponse::success_with_message(
        format!("Notifier '{}' removal requested", notifier_name)
    );
    HttpResponse::Ok().json(response)
}