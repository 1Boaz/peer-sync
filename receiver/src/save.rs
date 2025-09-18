use actix_web::{post, HttpResponse, Responder};
use std::fs;
use serde_json::from_str;
use serde::Deserialize;

#[derive(Deserialize)]
struct File {
    path: String,
    content: String,
}

#[post("/")]
async fn save(req_body: String) -> impl Responder {
    match from_str::<File>(&req_body) {
        Ok(file) => HttpResponse::Ok().body(save_file(file)),
        Err(e) => HttpResponse::BadRequest().body(format!("Invalid request: {}", e)),
    }
}

fn save_file(data: File) -> String {
    match fs::write(data.path, data.content) {
        Ok(_) => "File saved successfully".to_string(),
        Err(e) => format!("Error saving file: {}", e),
    }
}