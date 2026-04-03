use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post, delete},
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::signal;
use tower_http::cors::CorsLayer;

use stock_core::*;

// ── error type ────────────────────────────────────────────────────────────

struct AppError(StatusCode, String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, Json(serde_json::json!({"error": self.1}))).into_response()
    }
}

impl AppError {
    fn not_found(msg: impl Into<String>) -> Self { Self(StatusCode::NOT_FOUND, msg.into()) }
    fn internal(msg: impl Into<String>) -> Self { Self(StatusCode::INTERNAL_SERVER_ERROR, msg.into()) }
}

const WEBUI_HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/webui.html"));

// ── Python CLI helpers ─────────────────────────────────────────────────────

/// Build a Python command with correct PYTHONPATH for quant_analysis_cli
fn python_cmd(module: &str, args: &[&str], project_dir: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new("python3");
    let python_path = format!(
        "{0}/stock_api_cli/python:{0}/quant_analysis_cli/python",
        project_dir
    );
    cmd.env("PYTHONPATH", &python_path)
        .args([&["-m", module], args].concat())
        .current_dir(project_dir);
    cmd
}

// ── handlers ──────────────────────────────────────────────────────────────

async fn serve_webui() -> Html<&'static str> {
    Html(WEBUI_HTML)
}

async fn get_history(
    Path(symbol): Path<String>,
    Query(q): Query<HQ>,
    State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let days = q.days.unwrap_or(30);
    let bars = cached_fetch(&st, &symbol, days).await;
    Json(serde_json::json!({"symbol": symbol, "bars": bars}))
}

async fn get_quote(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let bars = match fetch_yahoo(&st.client, &symbol, 5).await {
        Ok(b) if !b.is_empty() => b,
        _ => fetch_av(&st.client, &symbol, &st.av_key).await.unwrap_or_default(),
    };
    if bars.len() < 2 {
        return Err(AppError::not_found(format!("no quote data for {symbol}")));
    }
    let prev = &bars[bars.len() - 2];
    let last = &bars[bars.len() - 1];
    let chg = last.close - prev.close;
    let pct = if prev.close > 0.0 { chg / prev.close * 100.0 } else { 0.0 };
    Ok(Json(serde_json::json!({
        "symbol": symbol, "price": last.close, "change": chg,
        "change_pct": pct, "volume": last.volume,
        "high": last.high, "low": last.low
    })))
}

async fn get_indicators(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let bars = cached_fetch(&st, &symbol, 180).await;
    if bars.is_empty() {
        return Err(AppError::not_found(format!("no data for {symbol}")));
    }
    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let rsi = calc_rsi(&closes, 14);
    let (macd, sig, hist) = calc_macd(&closes);
    let (bbu, bbm, bbl) = calc_bb(&closes, 20, 2.0);
    Ok(Json(serde_json::json!({
        "symbol": symbol, "rsi_14": rsi,
        "macd": macd, "macd_signal": sig, "macd_hist": hist,
        "bb_upper": bbu, "bb_mid": bbm, "bb_lower": bbl
    })))
}

// ── quant_analysis_cli integration ────────────────────────────────────────

async fn get_backtest(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = python_cmd(
        "quant_analysis_cli",
        &["analyze", &symbol, "--save"],
        &st.project_dir,
    ).output().await
        .map_err(|e| AppError::internal(format!("spawn: {e}")))?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(AppError::internal(format!("{err}")));
    }
    let json_path = format!("{}/output/analysis_result.json", st.project_dir);
    let s = tokio::fs::read_to_string(&json_path).await
        .map_err(|e| AppError::internal(format!("read: {e}")))?;
    let v: serde_json::Value = serde_json::from_str(&s)
        .map_err(|e| AppError::internal(format!("parse: {e}")))?;
    let result = if v.is_array() {
        v.as_array().and_then(|a| a.first().cloned()).unwrap_or(v)
    } else { v };
    Ok(Json(result))
}

async fn get_kline(
    Path(symbol): Path<String>,
    Query(q): Query<KQ>,
    State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let days = match q.period.as_deref() {
        Some("weekly") => 730,
        Some("monthly") => 1825,
        _ => 365,
    };
    let bars = cached_fetch(&st, &symbol, days).await;
    let from_ts: Option<i64> = q.from.as_ref().and_then(|d| {
        Some(chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()?.and_hms_opt(0, 0, 0)?.and_utc().timestamp())
    });
    let to_ts: Option<i64> = q.to.as_ref().and_then(|d| {
        Some(chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()?.and_hms_opt(23, 59, 59)?.and_utc().timestamp())
    });
    let rows: Vec<serde_json::Value> = bars.into_iter().filter(|b| {
        if let Some(f) = from_ts { if b.time < f { return false; } }
        if let Some(t) = to_ts { if b.time > t { return false; } }
        true
    }).map(|b| {
        let d = chrono::DateTime::from_timestamp(b.time, 0)
            .map(|t| t.format("%Y-%m-%d").to_string()).unwrap_or_default();
        serde_json::json!({"date": d, "open": b.open, "high": b.high, "low": b.low, "close": b.close, "volume": b.volume})
    }).collect();
    Json(serde_json::Value::Array(rows))
}

async fn get_report(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let out = python_cmd(
        "quant_analysis_cli",
        &["report", &symbol],
        &st.project_dir,
    ).output().await
        .map_err(|e| AppError::internal(format!("spawn: {e}")))?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(AppError::internal(format!("{err}")));
    }
    let filename = format!("{}/output/report_{}.html", st.project_dir, symbol.replace('.', "_"));
    let html = tokio::fs::read_to_string(&filename).await
        .map_err(|e| AppError::internal(format!("read report: {e}")))?;
    Ok(Html(html))
}

// ── Watchlist API ─────────────────────────────────────────────────────────

async fn api_watchlist_get(State(st): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let db = st.db.lock().unwrap();
    let items = watchlist_get_all(&db);
    Json(serde_json::json!({"watchlist": items}))
}

async fn api_watchlist_add(
    State(st): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let symbol = body["symbol"].as_str().unwrap_or("").to_string();
    let name = body["name"].as_str().unwrap_or("").to_string();
    let strategy_id = body["strategy_id"].as_i64();
    if symbol.is_empty() {
        return Json(serde_json::json!({"error": "symbol required"}));
    }
    let db = st.db.lock().unwrap();
    watchlist_add(&db, &symbol, &name, strategy_id);
    Json(serde_json::json!({"ok": true, "symbol": symbol}))
}

async fn api_watchlist_delete(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let db = st.db.lock().unwrap();
    watchlist_remove(&db, &symbol);
    Json(serde_json::json!({"ok": true}))
}

// ── Strategies API ────────────────────────────────────────────────────────

async fn api_strategies_get(State(st): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let db = st.db.lock().unwrap();
    let items = strategy_get_all(&db);
    Json(serde_json::json!({"strategies": items}))
}

async fn api_strategies_add(
    State(st): State<Arc<AppState>>,
    Json(body): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let s = Strategy {
        id: None,
        name: body["name"].as_str().unwrap_or("").to_string(),
        stype: body["stype"].as_str().unwrap_or("hmm_regime").to_string(),
        params: body["params"].as_str().unwrap_or("{}").to_string(),
        enabled: body["enabled"].as_bool().unwrap_or(true),
    };
    if s.name.is_empty() {
        return Json(serde_json::json!({"error": "name required"}));
    }
    let db = st.db.lock().unwrap();
    let id = strategy_add(&db, &s);
    Json(serde_json::json!({"ok": true, "id": id}))
}

async fn api_strategies_delete(
    Path(id): Path<i64>,
    State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let db = st.db.lock().unwrap();
    strategy_delete(&db, id);
    Json(serde_json::json!({"ok": true}))
}

// ── Signals API ───────────────────────────────────────────────────────────

async fn api_signals_get(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let db = st.db.lock().unwrap();
    let signals = signal_get_latest(&db, &symbol, 20);
    Json(serde_json::json!({"signals": signals}))
}

// ── 1-minute kline ───────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
struct HoursQ {
    hours: Option<u64>,
}

/// GET /api/kline1m/:symbol?hours=N  — query stored 1-min bars (default 168h = 7 days)
async fn api_kline1m_get(
    Path(symbol): Path<String>,
    Query(q): Query<HoursQ>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let hours = q.hours.unwrap_or(168);
    let now = chrono::Utc::now().timestamp();
    let from = now - (hours as i64) * 3600;
    let bars = {
        let db = st.db.lock().unwrap();
        db_query_1min(&db, &symbol, from, now)
    };
    if bars.is_empty() {
        return Err(AppError::not_found(format!(
            "No 1-min data for {symbol}. POST /api/fetch1m/{symbol} to fetch."
        )));
    }
    let rows: Vec<serde_json::Value> = bars.iter().map(|b| {
        let dt = chrono::DateTime::from_timestamp(b.time, 0)
            .map(|t| t.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .unwrap_or_default();
        serde_json::json!({
            "ts": b.time, "dt": dt,
            "open": b.open, "high": b.high, "low": b.low,
            "close": b.close, "volume": b.volume,
        })
    }).collect();
    let total = { let db = st.db.lock().unwrap(); count_1min(&db, &symbol) };
    Ok(Json(serde_json::json!({
        "symbol": symbol, "interval": "1m",
        "hours": hours, "bars": rows.len(), "total_stored": total,
        "data": rows,
    })))
}

/// GET /api/hmm1m/:symbol  — run 1-min HMM regime analysis (doc-08 taxonomy)
async fn api_hmm1m(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = python_cmd(
        "quant_analysis_cli",
        &["analyze1m", &symbol],
        &st.project_dir,
    ).output().await
        .map_err(|e| AppError::internal(format!("spawn: {e}")))?;

    if !out.status.success() {
        // Python may output JSON error to stdout even on non-zero exit
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
            if let Some(msg) = v.get("error").and_then(|e| e.as_str()).filter(|s| !s.is_empty()) {
                return Err(AppError::internal(msg.to_string()));
            }
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(AppError::internal(if stderr.is_empty() { format!("analyze1m {symbol} failed") } else { stderr.to_string() }));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout.trim())
        .map_err(|e| AppError::internal(format!("parse: {e}")))?;
    // Surface Python-level errors as 500
    if v.get("error").is_some() {
        let msg = v["error"].as_str().filter(|s| !s.is_empty()).unwrap_or("analyze1m failed");
        return Err(AppError::internal(msg.to_string()));
    }
    Ok(Json(v))
}

/// POST /api/fetch1m/:symbol  — spawn Python to fetch & store 1-min bars
async fn api_fetch1m(
    Path(symbol): Path<String>,
    State(st): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let out = python_cmd(
        "quant_analysis_cli",
        &["fetch-1m", &symbol, "--store"],
        &st.project_dir,
    ).output().await
        .map_err(|e| AppError::internal(format!("spawn: {e}")))?;

    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        return Err(AppError::internal(format!("{err}")));
    }
    // Parse the JSON summary printed to stdout
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|_| serde_json::json!({"ok": true}));
    Ok(Json(v))
}

// ── Scan: run quant_analysis_cli signals on all watchlist stocks ──────────

async fn api_scan(State(st): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let symbols: Vec<String> = {
        let db = st.db.lock().unwrap();
        watchlist_get_all(&db).into_iter().map(|w| w.symbol).collect()
    };
    if symbols.is_empty() {
        return Json(serde_json::json!({"error": "watchlist is empty"}));
    }

    // Run signals in parallel for all watchlist stocks
    let mut handles = Vec::new();
    for sym in &symbols {
        let sym = sym.clone();
        let dir = st.project_dir.clone();
        handles.push(tokio::spawn(async move {
            let out = python_cmd("quant_analysis_cli", &["signals", &sym, "--save"], &dir).output().await;
            match out {
                Ok(o) if o.status.success() => {
                    let stdout = String::from_utf8_lossy(&o.stdout).to_string();
                    (sym, true, stdout)
                }
                Ok(o) => {
                    let err = String::from_utf8_lossy(&o.stderr).to_string();
                    (sym, false, err)
                }
                Err(e) => (sym, false, e.to_string()),
            }
        }));
    }

    let mut results = Vec::new();
    for h in handles {
        match h.await {
            Ok((sym, ok, msg)) => results.push(serde_json::json!({
                "symbol": sym, "success": ok, "output": msg
            })),
            Err(e) => results.push(serde_json::json!({"error": e.to_string()})),
        }
    }
    Json(serde_json::json!({"scanned": results}))
}

// ── main ──────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let av_key = std::env::var("AV_API_KEY").expect("AV_API_KEY not set");
    let project_dir = std::env::var("PROJECT_DIR").unwrap_or_else(|_| {
        format!("{}", std::env::current_dir().unwrap_or_else(|_| ".".into()).display())
    });
    let db_dir = std::env::var("DATA_DIR").unwrap_or_else(|_| {
        format!("{}/.stock_ai", std::env::var("HOME").unwrap_or_else(|_| ".".into()))
    });
    std::fs::create_dir_all(&db_dir).unwrap();
    let db_path = format!("{db_dir}/data.db");
    let conn = rusqlite::Connection::open(&db_path).expect("open db");
    init_db(&conn);
    eprintln!("SQLite: {db_path}");

    let state = Arc::new(AppState {
        av_key,
        client: reqwest::Client::new(),
        db: Mutex::new(conn),
        project_dir,
    });
    let app = Router::new()
        // ── webui ──
        .route("/", get(serve_webui))
        // ── market data ──
        .route("/api/history/:symbol", get(get_history))
        .route("/api/quote/:symbol", get(get_quote))
        .route("/api/indicators/:symbol", get(get_indicators))
        .route("/api/kline/:symbol", get(get_kline))
        // ── quant analysis ──
        .route("/api/backtest/:symbol", get(get_backtest))
        .route("/api/report/:symbol", get(get_report))
        // ── watchlist ──
        .route("/api/watchlist", get(api_watchlist_get))
        .route("/api/watchlist", post(api_watchlist_add))
        .route("/api/watchlist/:symbol", delete(api_watchlist_delete))
        // ── strategies ──
        .route("/api/strategies", get(api_strategies_get))
        .route("/api/strategies", post(api_strategies_add))
        .route("/api/strategies/:id", delete(api_strategies_delete))
        // ── signals ──
        .route("/api/signals/:symbol", get(api_signals_get))
        // ── scan all ──
        .route("/api/scan", post(api_scan))
        // ── 1-minute kline ──
        .route("/api/kline1m/:symbol", get(api_kline1m_get))
        .route("/api/fetch1m/:symbol", post(api_fetch1m))
        .route("/api/hmm1m/:symbol", get(api_hmm1m))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));
    println!("Stock AI running at http://localhost:{}", addr.port());
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .with_graceful_shutdown(async {
        let _ = signal::ctrl_c().await;
    })
    .await
    .unwrap();
}
