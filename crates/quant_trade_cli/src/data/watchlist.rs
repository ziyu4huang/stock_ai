use crate::data::types::StockEntry;

pub fn default_watchlist() -> Vec<StockEntry> {
    vec![
        StockEntry { symbol: "2330.TW".into(), name: "台積電".into() },
        StockEntry { symbol: "2317.TW".into(), name: "鴻海".into() },
        StockEntry { symbol: "2454.TW".into(), name: "聯發科".into() },
        StockEntry { symbol: "2881.TW".into(), name: "富邦金".into() },
        StockEntry { symbol: "2308.TW".into(), name: "台達電".into() },
    ]
}
