mod save;

use save::save as Save;
use actix_web::{App, HttpServer};
use local_ip_address::local_ip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Save)
    })
        .bind((get_ip(), 8080))?
        .run()
        .await
}

fn get_ip() -> String {
    match local_ip() {
        Ok(s) => s.to_string(),
        Err(_) => "0.0.0.0".to_string()
    }
}