use crate::error::{Result, AppError};
use actix_web::{delete, HttpRequest, HttpResponse, Responder};
use tracing::{info, error};
use std::path::Path;

/// Deletes a file given its path.
///
/// This endpoint expects a JSON payload containing the `path` field.
/// The `path` field should contain the relative or absolute path to the file to be removed.
///
/// Returns a successful response with a plain text body containing the message "Successfully removed file"
/// if the file was removed successfully.
///
/// Returns a 400 Bad Request response with a plain text body containing the error message
/// if the request is invalid.
///
/// Returns a 500 Internal Server Error response with a plain text body containing the error message
/// if an error occurred while removing the file.
#[delete("/")]
async fn delete(req: HttpRequest) -> impl Responder {
    let path_string = match req.headers().get("Path") {
        Some(p) => match p.to_str() {
            Ok(p) => p.to_string(),
            Err(_) => return HttpResponse::BadRequest().body("Invalid UTF-8 in Path header")
        },
        None => return HttpResponse::BadRequest().body("Missing Path header")
    };
    let path = Path::new(&path_string);
    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return HttpResponse::BadRequest().body("Path traversal not allowed");
    }
    match remove(path_string).await {
        Ok(message) => {
            HttpResponse::Ok().body(message)
        }
        Err(e) => {
            error!("Error removing file: {}", e);
            match e {
                AppError::InvalidRequest(_) => HttpResponse::BadRequest().body(e.to_string()),
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    }
}

/// Removes a file or directory given its path.
///
/// Returns a successful response with a plain text body containing the message "Successfully removed file" or
/// "Successfully removed directory and its contents" if the file or directory was removed successfully.
///
/// Returns a 500 Internal Server Error response with a plain text body containing the error message
/// if an error occurred while removing the file or directory.
///
/// # Errors
///
/// * Returns an error if the file or directory does not exist.
/// * Returns an error if the file or directory cannot be removed.
async fn remove(path: String) -> Result<String> {
    let metadata = std::fs::metadata(&path)
        .map_err(|e| AppError::FileOperation(format!("Failed to get file metadata: {}", e)))?;

    let result = if metadata.is_dir() {
        std::fs::remove_dir_all(&path)
            .map(|_| "Successfully removed directory and its contents")
    } else {
        std::fs::remove_file(&path)
            .map(|_| "Successfully removed file")
    };

    result
        .map(|msg| {
            info!("{}: {}", msg, path);
            msg.to_string()
        })
        .map_err(|e| AppError::FileOperation(format!("Failed to remove: {}", e)))
}