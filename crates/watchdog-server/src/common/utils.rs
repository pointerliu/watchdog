//! Utility functions for checking duplicate resources

use actix_web::HttpResponse;
use crate::common::dto::ApiResponse;

/// Check if a name already exists in a list of existing names
/// Returns an HTTP response if duplicate is found, None otherwise
pub fn check_duplicate_name(
    existing_names: &[String],
    new_name: &str,
    resource_type: &str,
) -> Option<HttpResponse> {
    if existing_names.contains(&new_name.to_string()) {
        let response: ApiResponse<()> = ApiResponse::error(
            409,
            format!("{} with this name already exists", resource_type),
        );
        Some(HttpResponse::Conflict().json(response))
    } else {
        None
    }
}