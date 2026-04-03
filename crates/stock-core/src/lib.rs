//! stock-core: shared types, technical indicators, SQLite layer, data fetchers

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// ── types ─────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone, Debug)]
pub struct Bar {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

pub struct AppState {
    pub av_key: String,
    pub client: reqwest::Client,
    pub db: Mutex<rusqlite::Connection>,
    pub project_dir: String,
}

#[derive(Deserialize)]
pub struct HQ {
    pub days: Option<u64>,
}

#[derive(Deserialize)]
pub struct KQ {
    pub period: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

// ── Watchlist types ───────────────────────────────────────────────────────

#[derive(Serialize, Clone, Debug)]
pub struct WatchlistItem {
    pub symbol: String,
    pub name: String,
    pub strategy_id: Option<i64>,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Strategy {
    pub id: Option<i64>,
    pub name: String,
    pub stype: String,        // "hmm_regime", "rsi_reversal", "macd_cross", "bb_bounce"
    pub params: String,       // JSON: {"n_states":4, "best_regime_only":true, ...}
    pub enabled: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct SignalLog {
    pub id: i64,
    pub symbol: String,
    pub date: String,
    pub signal_type: String,  // "LONG", "SHORT", "EXIT", "HOLD"
    pub regime_state: Option<i64>,
    pub confidence: f64,
    pub details: Option<String>,
    pub created_at: String,
}

pub fn is_tw(symbol: &str) -> bool {
    symbol.ends_with(".TW") || symbol.ends_with(".TWO")
}

// ── indicators ────────────────────────────────────────────────────────────

pub fn calc_rsi(closes: &[f64], period: usize) -> f64 {
    if closes.len() < period + 1 {
        return 50.0;
    }
    let mut g = 0.0;
    let mut l = 0.0;
    let n = closes.len();
    for i in (n - period)..n {
        let d = closes[i] - closes[i - 1];
        if d > 0.0 {
            g += d;
        } else {
            l += d.abs();
        }
    }
    let ag = g / period as f64;
    let al = l / period as f64;
    if al == 0.0 {
        return 100.0;
    }
    100.0 - (100.0 / (1.0 + ag / al))
}

pub fn calc_ema(data: &[f64], p: usize) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let k = 2.0 / (p as f64 + 1.0);
    let mut e = vec![0.0; data.len()];
    e[0] = data[0];
    for i in 1..data.len() {
        e[i] = data[i] * k + e[i - 1] * (1.0 - k);
    }
    e
}

pub fn calc_macd(closes: &[f64]) -> (f64, f64, f64) {
    let e12 = calc_ema(closes, 12);
    let e26 = calc_ema(closes, 26);
    if e12.len() < 26 {
        return (0.0, 0.0, 0.0);
    }
    let ml: Vec<f64> = e12.iter().zip(&e26).map(|(a, b)| a - b).collect();
    let sl = calc_ema(&ml, 9);
    let n = ml.len();
    (ml[n - 1], sl[n - 1], ml[n - 1] - sl[n - 1])
}

pub fn calc_bb(closes: &[f64], p: usize, m: f64) -> (f64, f64, f64) {
    if closes.len() < p {
        let l = closes.last().copied().unwrap_or(0.0);
        return (l, l, l);
    }
    let s = &closes[closes.len() - p..];
    let mean = s.iter().sum::<f64>() / p as f64;
    let var = s.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / p as f64;
    let sd = var.sqrt();
    (mean + m * sd, mean, mean - m * sd)
}

// ── DB ────────────────────────────────────────────────────────────────────

pub fn init_db(c: &rusqlite::Connection) {
    c.execute_batch(
        "CREATE TABLE IF NOT EXISTS kline_daily (
            time INTEGER NOT NULL, symbol TEXT NOT NULL,
            open REAL, high REAL, low REAL, close REAL, volume INTEGER,
            PRIMARY KEY (symbol, time));
         CREATE INDEX IF NOT EXISTS idx_kd ON kline_daily(symbol, time ASC);

         CREATE TABLE IF NOT EXISTS watchlist (
            symbol TEXT PRIMARY KEY,
            name TEXT NOT NULL DEFAULT '',
            strategy_id INTEGER,
            added_at TEXT NOT NULL DEFAULT (datetime('now')));

         CREATE TABLE IF NOT EXISTS strategies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            stype TEXT NOT NULL DEFAULT 'hmm_regime',
            params TEXT NOT NULL DEFAULT '{}',
            enabled INTEGER NOT NULL DEFAULT 1);

         CREATE TABLE IF NOT EXISTS signal_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            symbol TEXT NOT NULL,
            date TEXT NOT NULL,
            signal_type TEXT NOT NULL,
            regime_state INTEGER,
            confidence REAL NOT NULL DEFAULT 0,
            details TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            UNIQUE(symbol, date, signal_type));
         CREATE INDEX IF NOT EXISTS idx_sig_sym ON signal_log(symbol, date DESC);",
    )
    .expect("db init");
}

// ── kline_daily CRUD ──────────────────────────────────────────────────────

pub fn db_query(c: &rusqlite::Connection, sym: &str, from: i64, to: i64) -> Vec<Bar> {
    let mut s = match c.prepare(
        "SELECT time,open,high,low,close,volume FROM kline_daily WHERE symbol=?1 AND time BETWEEN ?2 AND ?3 ORDER BY time",
    ) {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    s.query_map(rusqlite::params![sym, from, to], |r| {
        Ok(Bar {
            time: r.get(0)?,
            open: r.get(1)?,
            high: r.get(2)?,
            low: r.get(3)?,
            close: r.get(4)?,
            volume: r.get(5)?,
        })
    })
    .map(|rows| rows.filter_map(|b| b.ok()).collect())
    .unwrap_or_default()
}

pub fn db_upsert(c: &rusqlite::Connection, sym: &str, bars: &[Bar]) {
    let tx = c.unchecked_transaction().unwrap();
    for b in bars {
        let _ = tx.execute(
            "INSERT OR REPLACE INTO kline_daily VALUES (?1,?2,?3,?4,?5,?6,?7)",
            rusqlite::params![b.time, sym, b.open, b.high, b.low, b.close, b.volume],
        );
    }
    let _ = tx.commit();
}

pub fn slice_bars(bars: Vec<Bar>, days: u64) -> Vec<Bar> {
    if (days as usize) < bars.len() {
        bars[bars.len() - days as usize..].to_vec()
    } else {
        bars
    }
}

// ── watchlist CRUD ────────────────────────────────────────────────────────

pub fn watchlist_get_all(c: &rusqlite::Connection) -> Vec<WatchlistItem> {
    c.prepare(
        "SELECT symbol, name, strategy_id, added_at FROM watchlist ORDER BY added_at DESC",
    ).map(|mut s| {
        let rows = s.query_map([], |r| Ok(WatchlistItem {
            symbol: r.get(0)?,
            name: r.get(1)?,
            strategy_id: r.get(2)?,
            added_at: r.get(3)?,
        }));
        rows.map(|r| r.filter_map(|v| v.ok()).collect()).unwrap_or_default()
    }).unwrap_or_default()
}

pub fn watchlist_add(c: &rusqlite::Connection, symbol: &str, name: &str, strategy_id: Option<i64>) {
    c.execute(
        "INSERT OR REPLACE INTO watchlist (symbol, name, strategy_id) VALUES (?1, ?2, ?3)",
        rusqlite::params![symbol, name, strategy_id],
    ).unwrap();
}

pub fn watchlist_remove(c: &rusqlite::Connection, symbol: &str) {
    c.execute("DELETE FROM watchlist WHERE symbol = ?1", rusqlite::params![symbol]).unwrap();
}

// ── strategies CRUD ───────────────────────────────────────────────────────

pub fn strategy_get_all(c: &rusqlite::Connection) -> Vec<Strategy> {
    c.prepare(
        "SELECT id, name, stype, params, enabled FROM strategies ORDER BY id",
    ).map(|mut s| {
        let rows = s.query_map([], |r| Ok(Strategy {
            id: Some(r.get(0)?),
            name: r.get(1)?,
            stype: r.get(2)?,
            params: r.get(3)?,
            enabled: r.get::<_, i32>(4)? != 0,
        }));
        rows.map(|r| r.filter_map(|v| v.ok()).collect()).unwrap_or_default()
    }).unwrap_or_default()
}

pub fn strategy_add(c: &rusqlite::Connection, s: &Strategy) -> i64 {
    c.execute(
        "INSERT INTO strategies (name, stype, params, enabled) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![s.name, s.stype, s.params, s.enabled as i32],
    ).unwrap();
    c.last_insert_rowid()
}

pub fn strategy_delete(c: &rusqlite::Connection, id: i64) {
    c.execute("DELETE FROM strategies WHERE id = ?1", rusqlite::params![id]).unwrap();
}

// ── signal_log CRUD ───────────────────────────────────────────────────────

pub fn signal_get_latest(c: &rusqlite::Connection, symbol: &str, limit: usize) -> Vec<SignalLog> {
    c.prepare(
        "SELECT id, symbol, date, signal_type, regime_state, confidence, details, created_at \
         FROM signal_log WHERE symbol = ?1 ORDER BY date DESC LIMIT ?2",
    ).map(|mut s| {
        let rows = s.query_map(rusqlite::params![symbol, limit as i64], |r| Ok(SignalLog {
            id: r.get(0)?,
            symbol: r.get(1)?,
            date: r.get(2)?,
            signal_type: r.get(3)?,
            regime_state: r.get(4)?,
            confidence: r.get(5)?,
            details: r.get(6)?,
            created_at: r.get(7)?,
        }));
        rows.map(|r| r.filter_map(|v| v.ok()).collect()).unwrap_or_default()
    }).unwrap_or_default()
}

pub fn signal_insert(c: &rusqlite::Connection, symbol: &str, date: &str, signal_type: &str,
                     regime_state: Option<i64>, confidence: f64, details: Option<&str>) {
    c.execute(
        "INSERT OR REPLACE INTO signal_log (symbol, date, signal_type, regime_state, confidence, details) \
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![symbol, date, signal_type, regime_state, confidence, details],
    ).unwrap();
}

// ── Yahoo Finance fetcher (inline) ────────────────────────────────────────

pub async fn fetch_yahoo(client: &reqwest::Client, symbol: &str, days: u64) -> Result<Vec<Bar>, String> {
    let now = chrono::Utc::now();
    let from = now - chrono::Duration::days(days as i64);
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?period1={}&period2={}&interval=1d",
        symbol, from.timestamp(), now.timestamp()
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

pub async fn cached_fetch(st: &AppState, symbol: &str, days: u64) -> Vec<Bar> {
    let now = chrono::Utc::now().timestamp();
    let from = now - (days as i64) * 86400;
    {
        let db = st.db.lock().unwrap();
        let cached = db_query(&db, symbol, from, now);
        if !cached.is_empty() {
            return cached;
        }
    }
    // Try Yahoo first (works for all symbols), fall back to Alpha Vantage
    let result = fetch_yahoo(&st.client, symbol, days).await;
    let result = if result.is_err() || result.as_ref().map(|b| b.is_empty()).unwrap_or(true) {
        fetch_av(&st.client, symbol, &st.av_key).await
    } else {
        result
    };
    match result {
        Ok(bars) if !bars.is_empty() => {
            let db = st.db.lock().unwrap();
            db_upsert(&db, symbol, &bars);
            bars
        }
        _ => vec![],
    }
}
