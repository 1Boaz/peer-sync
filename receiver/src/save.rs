use crate::error::{Result, AppError};
use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;
use std::path::Path;
use tracing::{info, error};

#[derive(Deserialize)]
struct File {
    path: String,
    content: String,
}

/// Saves a file given its path and content.
///
/// Receives a JSON payload with `path` and `content` fields and saves the file to the given path,
/// creating directories as needed.
///
/// Returns a successful response with a plain text body containing the message "Successfully saved file"
/// if the file was saved successfully.
///
/// Returns a 400 Bad Request response with a plain text body containing the error message
/// if the request is invalid.
///
/// Returns a 500 Internal Server Error response with a plain text body containing the error message
/// if an error occurred while saving the file.
#[post("/")]
async fn save(req_body: String) -> impl Responder {
    match handle_save(req_body).await {
        Ok(message) => {
            info!("Successfully saved file");
            HttpResponse::Ok().body(message)
        }
        Err(e) => {
            error!("Error saving file: {}", e);
            match e {
                AppError::InvalidRequest(_) => HttpResponse::BadRequest().body(e.to_string()),
                _ => HttpResponse::InternalServerError().body("Internal server error"),
            }
        }
    }
}

/// Handles a file save request by deserializing the JSON payload and saving the file to the given path.
///
/// Returns a successful response with a plain text body containing the message "Successfully saved file"
/// if the file was saved successfully.
///
/// Returns a 400 Bad Request response with a plain text body containing the error message
/// if the request is invalid.
///
/// Returns a 500 Internal Server Error response with a plain text body containing the error message
/// if an error occurred while saving the file.
async fn handle_save(req_body: String) -> Result<String> {
    let file: File = serde_json::from_str(&req_body)
        .map_err(|e| AppError::Serialization(e))?;

    save_file(file)
}

/// Saves a file to the given path.
///
/// If the file path is invalid, returns an `AppError::FileOperation` with an appropriate error message.
///
/// If the file path does not exist, creates the directory recursively.
///
/// If an error occurs while writing the file, returns an `AppError::FileOperation` with an appropriate error message.
///
/// Returns a successful response with a plain text body containing the message "File saved successfully"
/// if the file was saved successfully.
fn save_file(data: File) -> Result<String> {
    let path = Path::new(&data.path);

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            std::fs::create_dir_all(parent_dir)
                .map_err(|e| AppError::FileOperation(format!("Failed to create directory: {}", e)))?;
        }
    } else {
        return Err(AppError::FileOperation("Invalid file path".to_string()));
    }

    std::fs::write(path, &data.content)
        .map_err(|e| AppError::FileOperation(format!("Failed to write file: {}", e)))?;

    info!("Successfully saved file: {}", path.display());
    Ok("File saved successfully".to_string())
}