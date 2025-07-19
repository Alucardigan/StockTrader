use serde::Serialize;


#[derive(Serialize)]
pub struct Ticker {
    symbol: String,
    price: f64,
    trend: Vec<f64>,
}
pub fn demo_tickers() -> Vec<Ticker> {
    vec![
        Ticker { symbol: "AAPL".into(), price: 172.25, trend: vec![170.1, 171.3, 171.8, 172.0, 172.5, 172.4, 172.25] },
        Ticker { symbol: "TSLA".into(), price: 180.01, trend: vec![178.5, 179.2, 179.9, 180.3, 180.2, 180.0, 180.01] },
        Ticker { symbol: "AMZN".into(), price: 134.30, trend: vec![132.0, 132.5, 133.1, 133.8, 134.0, 134.2, 134.30] },
    ]
}