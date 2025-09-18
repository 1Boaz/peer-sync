use actix_web::{post, HttpResponse, Responder};
use std::fs;


#[post("/")]
async fn save(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(save_file(req_body))
}

fn save_file(content: String) -> String {
    match fs::write("file.txt", content) {
        Ok(_) => "File saved successfully".to_string(),
        Err(e) => format!("Error saving file: {}", e),
    }
}