use rusqlite::{Connection, Result};

/// Initialize the geographic knowledge base tables.
pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS geo_broker (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            branch TEXT NOT NULL DEFAULT '',
            is_major INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS geo_company_broker (
            symbol TEXT NOT NULL,
            broker_id INTEGER NOT NULL,
            buy_amount REAL NOT NULL DEFAULT 0,
            sell_amount REAL NOT NULL DEFAULT 0,
            net_amount REAL NOT NULL DEFAULT 0,
            period TEXT NOT NULL,
            PRIMARY KEY (symbol, broker_id, period)
        );

        CREATE TABLE IF NOT EXISTS geo_sector (
            symbol TEXT PRIMARY KEY,
            sector TEXT NOT NULL DEFAULT ''
        );

        CREATE INDEX IF NOT EXISTS idx_company_broker_symbol ON geo_company_broker(symbol);
        CREATE INDEX IF NOT EXISTS idx_company_broker_period ON geo_company_broker(period);
        CREATE INDEX IF NOT EXISTS idx_sector ON geo_sector(sector);
        ",
    )?;
    Ok(())
}

/// Broker record.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Broker {
    pub id: i64,
    pub name: String,
    pub branch: String,
    pub is_major: bool,
}

/// Company-broker relationship record.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CompanyBroker {
    pub symbol: String,
    pub broker_id: i64,
    pub buy_amount: f64,
    pub sell_amount: f64,
    pub net_amount: f64,
    pub period: String,
}

/// Get top N brokers by net buy amount for a stock.
pub fn get_top_brokers(
    conn: &Connection,
    symbol: &str,
    period: &str,
    limit: usize,
) -> Result<Vec<CompanyBroker>> {
    let mut stmt = conn.prepare(
        "SELECT symbol, broker_id, buy_amount, sell_amount, net_amount, period
         FROM geo_company_broker
         WHERE symbol = ?1 AND period = ?2
         ORDER BY net_amount DESC
         LIMIT ?3",
    )?;
    let rows = stmt
        .query_map(rusqlite::params![symbol, period, limit], |row| {
            Ok(CompanyBroker {
                symbol: row.get(0)?,
                broker_id: row.get(1)?,
                buy_amount: row.get(2)?,
                sell_amount: row.get(3)?,
                net_amount: row.get(4)?,
                period: row.get(5)?,
            })
        })?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Get all stocks in the same sector as the given symbol.
pub fn get_sector_stocks(conn: &Connection, symbol: &str) -> Result<Vec<String>> {
    let sector: String = conn
        .query_row(
            "SELECT sector FROM geo_sector WHERE symbol = ?1",
            [symbol],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "Unknown".into());

    let mut stmt = conn.prepare(
        "SELECT symbol FROM geo_sector WHERE sector = ?1 AND symbol != ?2",
    )?;
    let rows = stmt
        .query_map(rusqlite::params![sector, symbol], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(rows)
}

/// Get broker name by id.
#[allow(dead_code)]
pub fn get_broker_name(conn: &Connection, broker_id: i64) -> Option<String> {
    conn.query_row(
        "SELECT name FROM geo_broker WHERE id = ?1",
        [broker_id],
        |row| row.get(0),
    )
    .ok()
}

/// Seed initial data for development.
pub fn seed_data(conn: &Connection) -> Result<()> {
    // Check if already seeded
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM geo_broker", [], |row| row.get(0))
        .unwrap_or(0);
    if count > 0 {
        return Ok(());
    }

    let brokers = [
        ("凱基-台北", "信義", 1),
        ("凱基-新竹", "新竹", 0),
        ("元大-信義", "信義", 1),
        ("元大-內湖", "內湖", 0),
        ("富邦-建國", "建國", 1),
        ("富邦-南京", "南京", 0),
        ("國泰-敦南", "敦南", 1),
        ("永豐金", "南京", 0),
        ("美林", "信義", 1),
        ("摩根大通", "信義", 1),
        ("凱基-竹科", "竹科", 0),
        ("元大-新竹", "新竹", 0),
        ("國泰-新竹", "新竹", 0),
        ("富邦-竹科", "竹科", 0),
    ];

    for (name, branch, is_major) in &brokers {
        conn.execute(
            "INSERT INTO geo_broker (name, branch, is_major) VALUES (?1, ?2, ?3)",
            rusqlite::params![name, branch, is_major],
        )?;
    }

    let sectors = [
        ("2330.TW", "半導體"),
        ("2454.TW", "半導體"),
        ("2308.TW", "半導體"),
        ("6488.TW", "半導體"),
        ("3711.TW", "封測"),
        ("5274.TW", "IC設計"),
        ("2317.TW", "電子代工"),
        ("2881.TW", "金融"),
        ("2882.TW", "金融"),
        ("2884.TW", "金融"),
        ("2603.TW", "航運"),
        ("2609.TW", "航運"),
        ("2412.TW", "電信"),
        ("3045.TW", "散熱"),
        ("0050.TW", "ETF"),
    ];

    for (symbol, sector) in &sectors {
        conn.execute(
            "INSERT OR REPLACE INTO geo_sector (symbol, sector) VALUES (?1, ?2)",
            rusqlite::params![symbol, sector],
        )?;
    }

    Ok(())
}
