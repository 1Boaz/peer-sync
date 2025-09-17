use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use local_ip_address::local_ip;

#[post("/")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", req_body);
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
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