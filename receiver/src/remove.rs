use actix_web::{delete, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::from_str;
use std::fs;
use std::fs::metadata;

#[derive(Deserialize)]
struct File {
    path: String,
    content: String,
}

/// Handles the HTTP DELETE request to remove a file.
///
/// # Parameters
/// * `req_body` - A JSON string containing the file path to be removed
///
/// # Returns
/// * `HttpResponse` - 200 OK with success message on success
///                   - 400 Bad Request for invalid JSON
///                   - 500 Internal Server Error if file cannot be removed
#[delete("/")]
async fn delete(req_body: String) -> impl Responder {
    match from_str::<File>(&req_body) {
        Ok(file) => match remove(file.path) {
            Ok(message) => HttpResponse::Ok().body(message),
            Err(e) => HttpResponse::InternalServerError().body(e),
        },
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
    }
}


/// Removes a file or directory at the given path.
///
/// # Returns
/// * `Result<String, String>` - Ok with success message on success
///                   - Err with error message if file cannot be removed
fn remove(path: String) -> Result<String, String> {
    match metadata(&path) {
        Ok(md) => match md.is_dir() {
            true => {
                match fs::remove_dir_all(path) {
                    Ok(_) => Ok("Removed file".to_string()),
                    Err(e) => Err(format!("Error removing directory and its contents: {}", e))
                }
            }
            false => {
                match fs::remove_file(path) {
                    Ok(_) => Ok("Removed file".to_string()),
                    Err(e) => Err(format!("Error removing file: {}", e))
                }
            }
        }
        Err(e) => Err(format!("Error getting metadata: {}", e))
    }
}