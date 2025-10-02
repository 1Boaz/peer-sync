mod error;
mod save;
mod args;
mod middleware;
mod remove;

use std::io;
use std::io::Error;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::from_fn;
use clap::Parser;
use local_ip_address::local_ip;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;


/// The main entry point of the application.
///
/// This function parses the command-line arguments, retrieves the local IP address,
/// initializes tracing, and starts the Actix Web server.
///
/// The server is configured to use the provided passkey for authorization,
/// and to listen on the provided IP and port.
#[actix_web::main]
async fn main() -> io::Result<()> {
    let args = args::ReceiverArgs::parse();
    let passkey = args.key;
    let ip = get_ip()?;

    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Starting server at http://{}:{}", ip, args.port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::PayloadConfig::new(80_000_000))
            .app_data(web::Data::new(passkey.clone()))
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(from_fn(middleware::validate))
            .service(save::save)
            .service(remove::delete)
    })
        .bind((ip, args.port))?
        .run()
        .await
}
/// Returns the local IP address as a string.
///
/// # Errors
///
/// * Returns an error if the local IP address cannot be determined.

fn get_ip() -> Result<String, Error> {
    local_ip()
        .map(|ip| ip.to_string())
        .map_err(|e| {
            tracing::error!("Failed to get local IP: {}", e);
            Error::new(io::ErrorKind::Other, format!("Failed to get local IP: {}", e))
        })
}