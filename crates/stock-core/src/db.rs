use crate::types::{Bar, SignalLog, Strategy, WatchlistItem};

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
         CREATE INDEX IF NOT EXISTS idx_sig_sym ON signal_log(symbol, date DESC);

         CREATE TABLE IF NOT EXISTS kline_1min (
            symbol TEXT NOT NULL,
            ts     INTEGER NOT NULL,
            open   REAL NOT NULL,
            high   REAL NOT NULL,
            low    REAL NOT NULL,
            close  REAL NOT NULL,
            volume INTEGER NOT NULL,
            PRIMARY KEY (symbol, ts));
         CREATE INDEX IF NOT EXISTS idx_k1m ON kline_1min(symbol, ts DESC);",
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

/// Query kline_1min bars between two UTC Unix timestamps.
pub fn db_query_1min(c: &rusqlite::Connection, sym: &str, from: i64, to: i64) -> Vec<Bar> {
    let mut s = match c.prepare(
        "SELECT ts,open,high,low,close,volume FROM kline_1min WHERE symbol=?1 AND ts BETWEEN ?2 AND ?3 ORDER BY ts",
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

/// Count stored 1-min bars for a symbol.
pub fn count_1min(c: &rusqlite::Connection, sym: &str) -> i64 {
    c.query_row(
        "SELECT COUNT(*) FROM kline_1min WHERE symbol=?1",
        rusqlite::params![sym],
        |r| r.get(0),
    )
    .unwrap_or(0)
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
