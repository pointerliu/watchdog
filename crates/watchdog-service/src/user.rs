//! User management module for the watchdog server
//!
//! This module provides functionality for managing user information,
//! including email addresses for notifications.

use actix_web::{
    web::{self, Data, Json, Path},
    Result as ActixResult, Scope,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Request to set a user's email address
#[derive(Deserialize, Serialize, Clone)]
pub struct SetUserEmailRequest {
    pub email: String,
}

/// Response for user email operations
#[derive(Serialize)]
pub struct UserEmailResponse {
    pub message: String,
}

/// Response containing user email details
#[derive(Serialize)]
pub struct UserEmailDetailsResponse {
    pub user_id: String,
    pub email: String,
}

/// Service for managing user emails
pub struct UserEmailService {
    user_emails: RwLock<HashMap<String, String>>,
}

impl UserEmailService {
    pub fn new() -> Self {
        Self {
            user_emails: RwLock::new(HashMap::new()),
        }
    }

    pub async fn set_user_email(&self, user_id: String, email: String) -> Result<(), String> {
        let mut emails = self.user_emails.write().await;
        emails.insert(user_id, email);
        Ok(())
    }

    pub async fn get_user_email(&self, user_id: &str) -> Result<Option<String>, String> {
        let emails = self.user_emails.read().await;
        Ok(emails.get(user_id).cloned())
    }

    pub async fn remove_user_email(&self, user_id: &str) -> Result<Option<String>, String> {
        let mut emails = self.user_emails.write().await;
        Ok(emails.remove(user_id))
    }

    pub async fn list_user_emails(&self) -> Result<Vec<(String, String)>, String> {
        let emails = self.user_emails.read().await;
        Ok(emails.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
}

impl Default for UserEmailService {
    fn default() -> Self {
        Self::new()
    }
}

/// API handler for setting a user's email address
pub async fn set_user_email(
    service: Data<Arc<RwLock<UserEmailService>>>,
    path: Path<String>,
    req: Json<SetUserEmailRequest>,
) -> ActixResult<Json<crate::api::ApiResponse<UserEmailResponse>>> {
    let user_id = path.into_inner();
    match service
        .read()
        .await
        .set_user_email(user_id.clone(), req.email.clone())
        .await
    {
        Ok(()) => Ok(Json(crate::api::ApiResponse::success(UserEmailResponse {
            message: "User email set successfully".to_string(),
        }))),
        Err(e) => Ok(Json(crate::api::ApiResponse::error(format!(
            "Failed to set user email: {e}"
        )))),
    }
}

/// API handler for getting a user's email address
pub async fn get_user_email(
    service: Data<Arc<RwLock<UserEmailService>>>,
    path: Path<String>,
) -> ActixResult<Json<crate::api::ApiResponse<UserEmailDetailsResponse>>> {
    let user_id = path.into_inner();
    match service.read().await.get_user_email(&user_id).await {
        Ok(Some(email)) => Ok(Json(crate::api::ApiResponse::success(
            UserEmailDetailsResponse { user_id, email },
        ))),
        Ok(None) => Ok(Json(crate::api::ApiResponse::error(
            "User email not found".to_string(),
        ))),
        Err(e) => Ok(Json(crate::api::ApiResponse::error(format!(
            "Failed to get user email: {e}"
        )))),
    }
}

/// API handler for removing a user's email address
pub async fn remove_user_email(
    service: Data<Arc<RwLock<UserEmailService>>>,
    path: Path<String>,
) -> ActixResult<Json<crate::api::ApiResponse<UserEmailResponse>>> {
    let user_id = path.into_inner();
    match service.read().await.remove_user_email(&user_id).await {
        Ok(Some(_)) => Ok(Json(crate::api::ApiResponse::success(UserEmailResponse {
            message: "User email removed successfully".to_string(),
        }))),
        Ok(None) => Ok(Json(crate::api::ApiResponse::error(
            "User email not found".to_string(),
        ))),
        Err(e) => Ok(Json(crate::api::ApiResponse::error(format!(
            "Failed to remove user email: {e}"
        )))),
    }
}

/// API handler for listing all user emails
pub async fn list_user_emails(
    service: Data<Arc<RwLock<UserEmailService>>>,
) -> ActixResult<Json<crate::api::ApiResponse<Vec<UserEmailDetailsResponse>>>> {
    match service.read().await.list_user_emails().await {
        Ok(emails) => {
            let response: Vec<UserEmailDetailsResponse> = emails
                .into_iter()
                .map(|(user_id, email)| UserEmailDetailsResponse { user_id, email })
                .collect();

            Ok(Json(crate::api::ApiResponse::success(response)))
        }
        Err(e) => Ok(Json(crate::api::ApiResponse::error(format!(
            "Failed to list user emails: {e}"
        )))),
    }
}

/// Create a new API scope for user email management
pub fn user_email_scope() -> Scope {
    web::scope("/users")
        .route("/{user_id}/email", web::post().to(set_user_email))
        .route("/{user_id}/email", web::get().to(get_user_email))
        .route("/{user_id}/email", web::delete().to(remove_user_email))
        .route("/emails", web::get().to(list_user_emails))
}
