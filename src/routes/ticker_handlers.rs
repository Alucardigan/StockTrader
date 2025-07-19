use axum :: Json;
use crate::models::stock_ticker::{Ticker,demo_tickers};
pub async fn tickers_handler() -> Json<Vec<Ticker>> {
    Json(demo_tickers())
}