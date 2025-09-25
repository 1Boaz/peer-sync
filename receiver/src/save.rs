use actix_web::{post, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::from_str;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct File {
    path: String,
    content: String,
}

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