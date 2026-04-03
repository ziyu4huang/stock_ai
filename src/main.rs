use axum::{
    extract::{Path, Query, State},
    response::Html,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tower_http::cors::CorsLayer;

const WEBUI_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/webui.html"));

// ── types ─────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone, Debug)]
struct Bar {
    time: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

struct AppState {
    av_key: String,
    client: reqwest::Client,
    db: Mutex<rusqlite::Connection>,
}

#[derive(Deserialize)]
struct HQ { days: Option<u64> }

#[derive(Deserialize)]
struct KQ { period: Option<String>, from: Option<String>, to: Option<String> }

fn is_tw(symbol: &str) -> bool {
    symbol.ends_with(".TW") || symbol.ends_with(".TWO")
}

// ── indicators ────────────────────────────────────────────────────────────

fn calc_rsi(closes: &[f64], period: usize) -> f64 {
    if closes.len() < period + 1 { return 50.0; }
    let mut g = 0.0; let mut l = 0.0; let n = closes.len();
    for i in (n - period)..n { let d = closes[i] - closes[i-1]; if d > 0.0 { g += d; } else { l += d.abs(); } }
    let ag = g / period as f64; let al = l / period as f64;
    if al == 0.0 { return 100.0; } 100.0 - (100.0 / (1.0 + ag / al))
}

fn calc_ema(data: &[f64], p: usize) -> Vec<f64> {
    if data.is_empty() { return vec![]; }
    let k = 2.0 / (p as f64 + 1.0);
    let mut e = vec![0.0; data.len()]; e[0] = data[0];
    for i in 1..data.len() { e[i] = data[i] * k + e[i-1] * (1.0 - k); } e
}

fn calc_macd(closes: &[f64]) -> (f64, f64, f64) {
    let e12 = calc_ema(closes, 12); let e26 = calc_ema(closes, 26);
    if e12.len() < 26 { return (0.0, 0.0, 0.0); }
    let ml: Vec<f64> = e12.iter().zip(&e26).map(|(a,b)| a-b).collect();
    let sl = calc_ema(&ml, 9); let n = ml.len();
    (ml[n-1], sl[n-1], ml[n-1] - sl[n-1])
}

fn calc_bb(closes: &[f64], p: usize, m: f64) -> (f64, f64, f64) {
    if closes.len() < p { let l = closes.last().copied().unwrap_or(0.0); return (l,l,l); }
    let s = &closes[closes.len()-p..]; let mean = s.iter().sum::<f64>() / p as f64;
    let var = s.iter().map(|x| (x-mean).powi(2)).sum::<f64>() / p as f64;
    let sd = var.sqrt(); (mean + m*sd, mean, mean - m*sd)
}

// ── DB ────────────────────────────────────────────────────────────────────

fn init_db(c: &rusqlite::Connection) {
    c.execute_batch("
        CREATE TABLE IF NOT EXISTS kline_daily (
            time INTEGER NOT NULL, symbol TEXT NOT NULL,
            open REAL, high REAL, low REAL, close REAL, volume INTEGER,
            PRIMARY KEY (symbol, time));
        CREATE INDEX IF NOT EXISTS idx_kd ON kline_daily(symbol, time ASC);
    ").expect("db init");
}

fn db_query(c: &rusqlite::Connection, sym: &str, from: i64, to: i64) -> Vec<Bar> {
    let mut s = match c.prepare("SELECT time,open,high,low,close,volume FROM kline_daily WHERE symbol=?1 AND time BETWEEN ?2 AND ?3 ORDER BY time") {
        Ok(s) => s, Err(_) => return vec![],
    };
    s.query_map(rusqlite::params![sym, from, to], |r| Ok(Bar {
        time: r.get(0)?, open: r.get(1)?, high: r.get(2)?, low: r.get(3)?, close: r.get(4)?, volume: r.get(5)?,
    })).map(|rows| rows.filter_map(|b| b.ok()).collect()).unwrap_or_default()
}

fn db_upsert(c: &rusqlite::Connection, sym: &str, bars: &[Bar]) {
    let tx = c.unchecked_transaction().unwrap();
    for b in bars { let _ = tx.execute("INSERT OR REPLACE INTO kline_daily VALUES (?1,?2,?3,?4,?5,?6,?7)", rusqlite::params![b.time, sym, b.open, b.high, b.low, b.close, b.volume]); }
    let _ = tx.commit();
}

// ── data sources ──────────────────────────────────────────────────────────

async fn fetch_twse(client: &reqwest::Client, symbol: &str) -> Result<Vec<Bar>, String> {
    let no = symbol.trim_end_matches(".TW").trim_end_matches(".TWO");
    let url = format!("https://openapi.twse.com.tw/v1/exchangeReport/STOCK_DAY?stockNo={no}&response=json");
    let raw: serde_json::Value = client.get(&url).send().await.map_err(|e| format!("TWSE:{e}"))?.json().await.map_err(|e| format!("TWSE parse:{e}"))?;
    let rows = raw.get("data").and_then(|d| d.as_array()).ok_or("no TWSE data")?;
    let mut bars: Vec<Bar> = rows.iter().filter_map(|r| {
        let a = r.as_array()?; let ds = a.get(0)?.as_str()?;
        let p: Vec<&str> = ds.split('/').collect(); if p.len() != 3 { return None; }
        let y: i32 = p[0].parse::<i32>().ok()? + 1911; let m: u32 = p[1].parse().ok()?; let d: u32 = p[2].parse().ok()?;
        let t = chrono::NaiveDate::from_ymd_opt(y,m,d)?.and_hms_opt(0,0,0)?.and_utc().timestamp();
        let pf = |i: usize| -> Option<f64> { a.get(i)?.as_str()?.replace(",","").parse().ok() };
        let pi = |i: usize| -> Option<i64> { a.get(i)?.as_str()?.replace(",","").parse().ok() };
        Some(Bar { time: t, open: pf(3)?, high: pf(4)?, low: pf(5)?, close: pf(6)?, volume: pi(1)? / 1000 })
    }).collect();
    bars.sort_by_key(|b| b.time); Ok(bars)
}

async fn fetch_yahoo(client: &reqwest::Client, symbol: &str, days: u64) -> Result<Vec<Bar>, String> {
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{symbol}?interval=1d&range={days}d");
    let raw: serde_json::Value = client.get(&url).header("User-Agent","Mozilla/5.0").send().await.map_err(|e| format!("Y:{e}"))?.json().await.map_err(|e| format!("Y parse:{e}"))?;
    let r = &raw["chart"]["result"][0]; let ts = r["timestamp"].as_array().ok_or("no ts")?;
    let q = &r["indicators"]["quote"][0];
    let (o,h,l,c,v) = (q["open"].as_array().ok_or("o")?, q["high"].as_array().ok_or("h")?, q["low"].as_array().ok_or("l")?, q["close"].as_array().ok_or("c")?, q["volume"].as_array().ok_or("v")?);
    let bars: Vec<Bar> = (0..ts.len()).filter_map(|i| {
        let t = ts[i].as_i64()?; let op = o[i].as_f64()?; let hi = h[i].as_f64()?;
        let lo = l[i].as_f64()?; let cl = c[i].as_f64()?; let vo = v[i].as_i64()?;
        if t > 0 && cl > 0.0 { Some(Bar { time: t, open: op, high: hi, low: lo, close: cl, volume: vo }) } else { None }
    }).collect(); Ok(bars)
}

async fn fetch_av(client: &reqwest::Client, symbol: &str, key: &str) -> Result<Vec<Bar>, String> {
    let url = format!("https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={symbol}&outputsize=compact&apikey={key}");
    let raw: serde_json::Value = client.get(&url).send().await.map_err(|e| format!("AV:{e}"))?.json().await.map_err(|e| format!("AV parse:{e}"))?;
    let series = raw.get("Time Series (Daily)").ok_or("no AV series")?.as_object().ok_or("not obj")?;
    let mut bars: Vec<Bar> = series.iter().filter_map(|(d,v)| {
        let t = chrono::NaiveDate::parse_from_str(d,"%Y-%m-%d").ok()?.and_hms_opt(0,0,0)?.and_utc().timestamp();
        let pf = |k: &str| -> Option<f64> { v.get(k)?.as_str()?.parse().ok() };
        let pi = |k: &str| -> Option<i64> { v.get(k)?.as_str()?.parse().ok() };
        Some(Bar { time: t, open: pf("1. open")?, high: pf("2. high")?, low: pf("3. low")?, close: pf("4. close")?, volume: pi("5. volume")? })
    }).collect();
    bars.sort_by_key(|b| b.time); Ok(bars)
}

async fn fetch_history(client: &reqwest::Client, symbol: &str, av_key: &str, days: u64) -> Vec<Bar> {
    let primary = if is_tw(symbol) { fetch_twse(client, symbol).await } else { fetch_av(client, symbol, av_key).await };
    match primary { Ok(ref b) if !b.is_empty() => { let mut b = b.clone(); b.sort_by_key(|x| x.time); b } _ => fetch_yahoo(client, symbol, days).await.unwrap_or_else(|e| { eprintln!("Yahoo err: {e}"); vec![] }) }
}

/// Get bars with SQLite cache
async fn cached_fetch(st: &AppState, symbol: &str, days: u64) -> Vec<Bar> {
    let from_ts = chrono::Utc::now().checked_sub_signed(chrono::Duration::days(days as i64 + 60)).map(|t| t.timestamp()).unwrap_or(0);
    let to_ts = chrono::Utc::now().timestamp();

    // Try cache
    if let Ok(conn) = st.db.lock() {
        let cached = db_query(&conn, symbol, from_ts, to_ts);
        if cached.len() > 5 {
            eprintln!("[CACHE HIT] {} ({} bars)", symbol, cached.len());
            return slice(cached, days);
        }
    }

    // Miss → fetch + store
    eprintln!("[CACHE MISS] {}", symbol);
    let bars = fetch_history(&st.client, symbol, &st.av_key, days + 60).await;
    if !bars.is_empty() {
        if let Ok(conn) = st.db.lock() {
            db_upsert(&conn, symbol, &bars);
            eprintln!("[STORED] {} ({} bars)", symbol, bars.len());
        }
    }
    slice(bars, days)
}

fn slice(bars: Vec<Bar>, days: u64) -> Vec<Bar> {
    if (days as usize) < bars.len() { bars[bars.len()-days as usize..].to_vec() } else { bars }
}

// ── handlers ──────────────────────────────────────────────────────────────

async fn serve_webui() -> Html<&'static str> { Html(WEBUI_HTML) }

async fn get_history(
    Path(symbol): Path<String>, Query(q): Query<HQ>, State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let days = q.days.unwrap_or(30);
    let bars = cached_fetch(&st, &symbol, days).await;
    Json(serde_json::json!({"symbol": symbol, "bars": bars}))
}

async fn get_quote(
    Path(symbol): Path<String>, State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let bars = if is_tw(&symbol) { fetch_yahoo(&st.client, &symbol, 5).await.unwrap_or_default() }
        else { fetch_av(&st.client, &symbol, &st.av_key).await.unwrap_or_default() };
    if bars.len() < 2 { return Json(serde_json::json!({"error": "no data"})); }
    let prev = &bars[bars.len()-2]; let last = &bars[bars.len()-1];
    let chg = last.close - prev.close; let pct = if prev.close > 0.0 { chg / prev.close * 100.0 } else { 0.0 };
    Json(serde_json::json!({"symbol": symbol, "price": last.close, "change": chg, "change_pct": pct, "volume": last.volume, "high": last.high, "low": last.low}))
}

async fn get_indicators(
    Path(symbol): Path<String>, State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let bars = cached_fetch(&st, &symbol, 180).await;
    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let rsi = calc_rsi(&closes, 14);
    let (macd, sig, hist) = calc_macd(&closes);
    let (bbu, bbm, bbl) = calc_bb(&closes, 20, 2.0);
    Json(serde_json::json!({"symbol": symbol, "rsi_14": rsi, "macd": macd, "macd_signal": sig, "macd_hist": hist, "bb_upper": bbu, "bb_mid": bbm, "bb_lower": bbl}))
}

async fn get_backtest(
    Path(symbol): Path<String>, State(_st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let out = tokio::process::Command::new("python3")
        .arg("analyze.py").arg("--json").arg(&symbol)
        .current_dir("/Users/huangziyu/proj/stock_ai")
        .output().await;
    match out {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout);
            serde_json::from_str(&s).unwrap_or_else(|e| serde_json::json!({"error": format!("parse:{e}"), "raw": &s[..s.len().min(500)]}))
        }
        Ok(o) => serde_json::json!({"error": String::from_utf8_lossy(&o.stderr).chars().take(500).collect::<String>()}),
        Err(e) => serde_json::json!({"error": format!("spawn:{e}")}),
    }.into()
}

async fn get_kline(
    Path(symbol): Path<String>, Query(q): Query<KQ>, State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let days = match q.period.as_deref() { Some("weekly") => 730, Some("monthly") => 1825, _ => 365 };
    let bars = cached_fetch(&st, &symbol, days).await;
    let from_ts: Option<i64> = q.from.as_ref().and_then(|d| Some(chrono::NaiveDate::parse_from_str(d,"%Y-%m-%d").ok()?.and_hms_opt(0,0,0)?.and_utc().timestamp()));
    let to_ts: Option<i64> = q.to.as_ref().and_then(|d| Some(chrono::NaiveDate::parse_from_str(d,"%Y-%m-%d").ok()?.and_hms_opt(23,59,59)?.and_utc().timestamp()));
    let rows: Vec<serde_json::Value> = bars.into_iter().filter(|b| {
        if let Some(f) = from_ts { if b.time < f { return false; } }
        if let Some(t) = to_ts { if b.time > t { return false; } }
        true
    }).map(|b| {
        let d = chrono::DateTime::from_timestamp(b.time,0).map(|t| t.format("%Y-%m-%d").to_string()).unwrap_or_default();
        serde_json::json!({"date": d, "open": b.open, "high": b.high, "low": b.low, "close": b.close, "volume": b.volume})
    }).collect();
    Json(serde_json::Value::Array(rows))
}

async fn get_report(
    Path(symbol): Path<String>,
) -> Html<String> {
    let out = tokio::process::Command::new("python3")
        .args(["-m", "quant_cli", "report", &symbol, "--period", "1y"])
        .current_dir("/Users/huangziyu/proj/stock_ai")
        .output().await;

    match out {
        Ok(o) if o.status.success() => {
            // Read the generated HTML file
            let filename = format!("/Users/huangziyu/proj/stock_ai/report_{}.html", symbol.replace('.', "_"));
            match tokio::fs::read_to_string(&filename).await {
                Ok(html) => Html(html),
                Err(e) => Html(format!("<html><body><h2>Error reading report</h2><pre>{e}</pre></body></html>")),
            }
        }
        Ok(o) => {
            let err = String::from_utf8_lossy(&o.stderr);
            Html(format!("<html><body><h2>Report generation failed</h2><pre>{err}</pre></body></html>"))
        }
        Err(e) => Html(format!("<html><body><h2>Spawn failed</h2><pre>{e}</pre></body></html>")),
    }
}

// ── main ──────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let av_key = std::env::var("AV_API_KEY").expect("AV_API_KEY not set");
    let db_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| format!("{}/.stock_ai", std::env::var("HOME").unwrap_or_else(|_| ".".into())));
    std::fs::create_dir_all(&db_dir).unwrap();
    let db_path = format!("{db_dir}/data.db");
    let conn = rusqlite::Connection::open(&db_path).expect("open db");
    init_db(&conn);
    eprintln!("SQLite: {db_path}");

    let state = Arc::new(AppState { av_key, client: reqwest::Client::new(), db: Mutex::new(conn) });
    let app = Router::new()
        .route("/", get(serve_webui))
        .route("/api/history/:symbol", get(get_history))
        .route("/api/quote/:symbol", get(get_quote))
        .route("/api/indicators/:symbol", get(get_indicators))
        .route("/api/backtest/:symbol", get(get_backtest))
        .route("/api/kline/:symbol", get(get_kline))
        .route("/api/report/:symbol", get(get_report))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0,0,0,0], 3003));
    println!("Stock AI running at http://localhost:{}", addr.port());
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .with_graceful_shutdown(async { let _ = signal::ctrl_c().await; })
        .await.unwrap();
}
