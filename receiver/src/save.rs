use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::from_str;
use std::fs;
use std::path::Path;

/// Represents a file to be saved, containing its path and content.
///
/// # Fields
/// * `path` - The filesystem path where the file should be saved
/// * `content` - The content to be written to the file
#[derive(Deserialize)]
struct File {
    path: String,
    content: String,
}

/// Handles the HTTP POST request to save a file.
///
/// # Arguments
/// * `req_body` - A JSON string containing the file path and content
///
/// # Returns
/// * `HttpResponse` - 200 OK with success message on success
///                   - 400 Bad Request for invalid JSON
///                   - 500 Internal Server Error if file cannot be saved
#[post("/")]
async fn save(req_body: String) -> impl Responder {
    match from_str::<File>(&req_body) {
        Ok(file) => match save_file(file) {
            Ok(message) => HttpResponse::Ok().body(message),
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
    }
}

/// Saves the file to the specified path, creating directories if needed.
///
/// # Arguments
/// * `data` - A `File` struct containing the path and content
///
/// # Returns
/// * `Result<String, String>` - Success message or error description
///
/// # Errors
/// * Returns an error if directories cannot be created or file cannot be written
fn save_file(data: File) -> Result<String, String> {
    let path = Path::new(&data.path);

    if let Some(parent_dir) = path.parent() {
        if !parent_dir.exists() {
            if let Err(e) = fs::create_dir_all(parent_dir) {
                return Err(format!("Error creating directory: {}", e));
            }
        }
    } else {
        return Err("Could not determine parent directory from path".to_string());
    }

    match fs::write(path, &data.content) {
        Ok(_) => Ok("File saved successfully".to_string()),
        Err(e) => Err(format!("Error saving file: {}", e)),
    }
}