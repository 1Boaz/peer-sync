use crate::error::{Result, AppError};
use actix_web::{delete, HttpResponse, Responder};
use serde::Deserialize;
use tracing::{info, error};

#[derive(Deserialize)]
struct File {
    path: String,
}

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
async fn delete(req_body: String) -> impl Responder {
    match handle_delete(req_body).await {
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

/// Handles a file deletion request by deserializing the JSON payload and removing the file at the given path.
///
/// Returns a successful response with a plain text body containing the message "Successfully removed file"
/// if the file was removed successfully.
///
/// Returns a 400 Bad Request response with a plain text body containing the error message
/// if the request is invalid.
///
/// Returns a 500 Internal Server Error response with a plain text body containing the error message
/// if an error occurred while removing the file.
async fn handle_delete(req_body: String) -> Result<String> {
    let file: File = serde_json::from_str(&req_body)
        .map_err(|e| AppError::Serialization(e))?;

    remove(file.path)
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
fn remove(path: String) -> Result<String> {
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