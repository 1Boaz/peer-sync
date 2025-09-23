mod save;
mod args;
mod middleware;

use save::save as Save;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::from_fn;
use local_ip_address::local_ip;
use clap::Parser;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = args::ReceiverArgs::parse();
    let passkey = args.key;
    let ip = get_ip();

    println!("Starting server at http://{}:{}", ip, args.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(passkey.clone()))
            .wrap(from_fn(middleware::validate))
            .service(Save)
    })
        .bind((ip, args.port))?
        .run()
        .await
}

fn get_ip() -> String {
    match local_ip() {
        Ok(s) => s.to_string(),
        Err(_) => "0.0.0.0".to_string()
    }
}