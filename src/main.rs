mod models;
mod routes;

use axum::{
    routing::get,Router
};
use routes::ticker_handlers::{tickers_handler};


#[tokio::main]
async fn main() {
    let app = Router::new().route("/tickers", get(tickers_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🦀 Server running on http://{}", listener.local_addr().unwrap());

    // Axum 0.7 helper: no hyper boilerplate needed
    axum::serve(listener, app).await.unwrap();
}