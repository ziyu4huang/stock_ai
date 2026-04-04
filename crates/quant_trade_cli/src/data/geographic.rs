use rusqlite::Connection;

use super::db;

/// Geographic analysis helper. Computes geo bonus and sector cluster count.
pub struct GeoHelper;

impl GeoHelper {
    /// Compute geo bonus score based on top broker concentration.
    /// Returns 0-20 based on how concentrated buying is among top brokers.
    pub fn geo_bonus(conn: &Connection, symbol: &str, period: &str) -> i32 {
        let brokers = db::get_top_brokers(conn, symbol, period, 3).unwrap_or_default();
        if brokers.is_empty() {
            return 0;
        }

        let total_net: f64 = brokers.iter().map(|b| b.net_amount).sum();
        if total_net <= 0.0 {
            return 0;
        }

        // Scale: 50M+ NTD net buy = full 20 points
        let bonus = (total_net / 50_000_000.0 * 20.0).min(20.0) as i32;
        bonus
    }

    /// Count how many stocks in the same sector have positive net broker activity.
    pub fn sector_cluster_count(conn: &Connection, symbol: &str, period: &str) -> usize {
        let peers = db::get_sector_stocks(conn, symbol).unwrap_or_default();
        let mut count = 0;
        for peer in &peers {
            let brokers = db::get_top_brokers(conn, peer, period, 1).unwrap_or_default();
            if let Some(b) = brokers.first() {
                if b.net_amount > 0.0 {
                    count += 1;
                }
            }
        }
        count
    }

    /// Get broker names and net amounts for display.
    #[allow(dead_code)]
    pub fn get_broker_details(
        conn: &Connection,
        symbol: &str,
        period: &str,
        limit: usize,
    ) -> Vec<(String, f64)> {
        let brokers = db::get_top_brokers(conn, symbol, period, limit).unwrap_or_default();
        brokers
            .into_iter()
            .map(|b| {
                let name = db::get_broker_name(conn, b.broker_id)
                    .unwrap_or_else(|| format!("Broker#{}", b.broker_id));
                (name, b.net_amount)
            })
            .collect()
    }
}
