mod save;
mod args;
mod middleware;
mod remove;

use save::save as Save;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::from_fn;
use local_ip_address::local_ip;
use clap::Parser;

/// Main entry point for the file receiver server.
///
/// # Returns
/// * `std::io::Result<()>` - Ok if the server runs successfully, or an IO error if it fails.
///
/// Errors are propagated via the returned `Result` rather than causing a panic.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = args::ReceiverArgs::parse();
    let passkey = args.key;
    let ip = get_ip();

    println!("Starting server at http://{}:{}", ip, args.port);

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(80_000_000)) // 80MB max payload size
            .app_data(web::Data::new(passkey.clone()))
            .wrap(from_fn(middleware::validate))
            .wrap(actix_web::middleware::Logger::default())
            .service(Save)
            .service(remove::delete)
    })
        .bind((ip, args.port))?
        .run()
        .await
}

/// Gets the local IP address of the machine.
///
/// # Returns
/// * `String` - The local IP address as a string, or "0.0.0.0" if the IP cannot be determined.
fn get_ip() -> String {
    match local_ip() {
        Ok(s) => s.to_string(),
        Err(_) => "0.0.0.0".to_string()
    }
}