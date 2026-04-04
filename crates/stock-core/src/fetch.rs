use crate::db::{db_query, db_upsert};
use crate::types::{AppState, Bar};

// ── Yahoo Finance fetcher ─────────────────────────────────────────────────

pub async fn fetch_yahoo(client: &reqwest::Client, symbol: &str, days: u64, interval: &str) -> Result<Vec<Bar>, String> {
    let now = chrono::Utc::now();
    let from = now - chrono::Duration::days(days as i64);
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval={}",
        symbol, from.timestamp(), now.timestamp(), interval
    );
    let resp = client.get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send().await.map_err(|e| e.to_string())?;
    let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let timestamps = data["chart"]["result"][0]["timestamp"].as_array()
        .ok_or("no timestamps")?;
    let quotes = data["chart"]["result"][0]["indicators"]["quote"][0].as_object()
        .ok_or("no quotes")?;
    let opens = quotes["open"].as_array().ok_or("no open")?;
    let highs = quotes["high"].as_array().ok_or("no high")?;
    let lows = quotes["low"].as_array().ok_or("no low")?;
    let closes = quotes["close"].as_array().ok_or("no close")?;
    let volumes = quotes["volume"].as_array().ok_or("no volume")?;

    let mut bars = Vec::new();
    for i in 0..timestamps.len() {
        let o = opens[i].as_f64().unwrap_or(0.0);
        let h = highs[i].as_f64().unwrap_or(0.0);
        let l = lows[i].as_f64().unwrap_or(0.0);
        let cl = closes[i].as_f64().unwrap_or(0.0);
        let v = volumes[i].as_i64().unwrap_or(0);
        if o == 0.0 && cl == 0.0 { continue; }
        bars.push(Bar { time: timestamps[i].as_i64().unwrap_or(0), open: o, high: h, low: l, close: cl, volume: v });
    }
    Ok(bars)
}

// ── Alpha Vantage fetcher ─────────────────────────────────────────────────

pub async fn fetch_av(client: &reqwest::Client, symbol: &str, api_key: &str) -> Result<Vec<Bar>, String> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&apikey={}&outputsize=full",
        symbol, api_key
    );
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let ts = data["Time Series (Daily)"].as_object().ok_or("no AV data")?;
    let mut bars: Vec<Bar> = ts.iter().map(|(date, v)| {
        let t = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map(|d| d.and_hms_opt(0,0,0).unwrap().and_utc().timestamp())
            .unwrap_or(0);
        Bar {
            time: t,
            open: v["1. open"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
            high: v["2. high"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
            low: v["3. low"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
            close: v["4. close"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0.0),
            volume: v["5. volume"].as_str().and_then(|s| s.parse().ok()).unwrap_or(0),
        }
    }).collect();
    bars.sort_by_key(|b| b.time);
    Ok(bars)
}

// ── cached fetch (Yahoo/AV with SQLite cache) ─────────────────────────────

pub async fn cached_fetch(st: &AppState, symbol: &str, days: u64, interval: &str) -> Vec<Bar> {
    // Only use SQLite cache for daily interval (intraday bars have different granularity)
    if interval == "1d" {
        let now = chrono::Utc::now().timestamp();
        let from = now - (days as i64) * 86400;
        let stale_after = now - 4 * 3600; // refetch if latest bar is older than 4h
        {
            let db = st.db.lock().unwrap();
            let cached = db_query(&db, symbol, from, now);
            if !cached.is_empty() {
                let latest_ts = cached.iter().map(|b| b.time).max().unwrap_or(0);
                if latest_ts >= stale_after {
                    return cached; // fresh enough
                }
                // fall through to refetch stale data
            }
        }
    }
    // Respect fetch_backend config
    let backend = st.fetch_backend.lock().unwrap().clone();
    let result = match backend.as_str() {
        "av" => fetch_av(&st.client, symbol, &st.av_key).await,
        "yahoo-only" => fetch_yahoo(&st.client, symbol, days, interval).await,
        _ => { // "yahoo" (default) — Yahoo first, AV fallback
            let result = fetch_yahoo(&st.client, symbol, days, interval).await;
            if result.is_err() || result.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
                fetch_av(&st.client, symbol, &st.av_key).await
            } else {
                result
            }
        }
    };
    match result {
        Ok(bars) if !bars.is_empty() => {
            if interval == "1d" {
                let db = st.db.lock().unwrap();
                db_upsert(&db, symbol, &bars);
            }
            bars
        }
        _ => vec![],
    }
}
