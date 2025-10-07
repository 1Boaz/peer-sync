use std::path::Path;
use crate::error::{Result, AppError};
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use tracing::{info, error};

struct File {
    path: String,
    content: Vec<u8>,
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
async fn save(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let path = match req.headers().get("Path") {
        Some(path) => match path.to_str() {
            Ok(p) => p.to_string(),
            Err(_) => return HttpResponse::BadRequest().body("Invalid Path header"),
        },
        None => return HttpResponse::BadRequest().body("Missing Path header"),
    };

    let path = Path::new(&path);
    if path.is_absolute() {
        return HttpResponse::BadRequest().body("Absolute paths are not allowed");
    }
    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return HttpResponse::BadRequest().body("Path traversal is not allowed");
    }
    let path = path.to_string_lossy().to_string();

    let file = File {
        path,
        content: body.to_vec()
    };
    match save_file(file).await {
        Ok(_) => {
            info!("Successfully saved file");
            HttpResponse::Ok().body("Successfully saved file")
        }
        Err(e) => {
            error!("Error saving file: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
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
async fn save_file(data: File) -> Result<String> {
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