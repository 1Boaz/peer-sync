use axum::{Router, routing::get};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(axum_is_solid));
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn axum_is_solid() -> &'static str {
    "dsf"
}