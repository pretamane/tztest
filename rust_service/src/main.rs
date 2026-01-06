use axum::{
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route("/process-urls", post(process_urls));

    // run our app with hyper
    // axum 0.6+ uses SocketAddr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct UrlPayload {
    source: String,
    urls: Vec<String>,
}

async fn process_urls(Json(payload): Json<UrlPayload>) -> &'static str {
    println!("Received payload from source: {}", payload.source);
    println!("URLs to process:");
    for url in payload.urls {
        println!(" - {}", url);
    }
    "Received and processed"
}
