use std::collections::HashMap;
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

use crate::stock_view::AlertKind;
use crate::types::{PositionDir, WhaleRank};

// ── Sound request ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SoundReq {
    BullWhale,
    BearWhale,
    MegaWhale,
    Ignition,
    Voice(String),
}

impl SoundReq {
    fn priority(&self) -> u8 {
        match self {
            SoundReq::Ignition => 100,
            SoundReq::MegaWhale => 80,
            SoundReq::Voice(_) => 60,
            SoundReq::BullWhale | SoundReq::BearWhale => 40,
        }
    }

    fn cooldown_group(&self) -> String {
        match self {
            SoundReq::BullWhale | SoundReq::BearWhale | SoundReq::MegaWhale => "whale",
            SoundReq::Ignition => "ignition",
            SoundReq::Voice(_) => "voice",
        }
        .to_string()
    }

    fn default_cooldown(&self) -> Duration {
        match self {
            SoundReq::BullWhale | SoundReq::BearWhale => Duration::from_secs(3),
            SoundReq::MegaWhale => Duration::from_secs(5),
            SoundReq::Ignition => Duration::from_secs(8),
            SoundReq::Voice(_) => Duration::from_secs(4),
        }
    }

    fn execute(&self) {
        match self {
            SoundReq::BullWhale => {
                Command::new("afplay")
                    .arg("/System/Library/Sounds/Ping.aiff")
                    .spawn()
                    .ok();
            }
            SoundReq::BearWhale => {
                Command::new("afplay")
                    .arg("/System/Library/Sounds/Basso.aiff")
                    .spawn()
                    .ok();
            }
            SoundReq::MegaWhale => {
                Command::new("afplay")
                    .arg("/System/Library/Sounds/Sosumi.aiff")
                    .spawn()
                    .ok();
            }
            SoundReq::Ignition => {
                Command::new("afplay")
                    .arg("/System/Library/Sounds/Hero.aiff")
                    .spawn()
                    .ok();
            }
            SoundReq::Voice(text) => {
                Command::new("say")
                    .args(["-v", "Meijia", "-r", "200", text])
                    .spawn()
                    .ok();
            }
        }
    }
}

// ── Sound queue ──────────────────────────────────────────────────────────────

pub struct SoundQueue {
    tx: mpsc::Sender<SoundReq>,
    cooldowns: Arc<Mutex<HashMap<String, f64>>>,
}

impl SoundQueue {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let cooldowns: Arc<Mutex<HashMap<String, f64>>> = Arc::new(Mutex::new(HashMap::new()));
        let cd_map = Arc::clone(&cooldowns);

        std::thread::spawn(move || player_loop(rx, cd_map));

        Self { tx, cooldowns }
    }

    pub fn enqueue(&self, req: SoundReq) {
        let _ = self.tx.send(req);
    }

    pub fn set_cooldown(&self, group: &str, secs: Option<f64>) {
        let mut map = self.cooldowns.lock().unwrap();
        match secs {
            Some(s) => { map.insert(group.to_string(), s); }
            None => { map.remove(group); }
        }
    }

    pub fn get_cooldown(&self, group: &str) -> Option<f64> {
        self.cooldowns.lock().unwrap().get(group).copied()
    }
}

fn player_loop(rx: mpsc::Receiver<SoundReq>, cd_map: Arc<Mutex<HashMap<String, f64>>>) {
    let mut last_played: HashMap<String, Instant> = HashMap::new();
    let mut pending: HashMap<String, (SoundReq, Duration)> = HashMap::new();

    loop {
        // ── Phase 1: flush any pending requests whose cooldown has elapsed
        let now = Instant::now();
        let ready: Vec<(String, SoundReq)> = pending
            .drain()
            .filter_map(|(group, (req, cd))| {
                let elapsed = last_played.get(&group)
                    .map(|t| now.duration_since(*t))
                    .unwrap_or(cd);
                if elapsed >= cd {
                    Some((group, req))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for (group, req) in ready {
            req.execute();
            last_played.insert(group, now);
        }

        // ── Phase 2: wait for next request (timeout = re-check interval)
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(req) => {
                let group = req.cooldown_group();
                let cd = cd_map
                    .lock()
                    .unwrap()
                    .get(&group)
                    .copied()
                    .map(Duration::from_secs_f64)
                    .unwrap_or_else(|| req.default_cooldown());

                let elapsed = last_played.get(&group)
                    .map(|t| now.duration_since(*t))
                    .unwrap_or(cd);

                if elapsed >= cd {
                    // Cooldown elapsed — play immediately
                    req.execute();
                    last_played.insert(group, now);
                } else {
                    // Cooldown active — keep only the highest-priority pending
                    let should_replace = match pending.get(&group) {
                        Some((existing, _)) => req.priority() > existing.priority(),
                        None => true,
                    };
                    if should_replace {
                        pending.insert(group, (req, cd));
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

// ── High-level fire_alert ────────────────────────────────────────────────────

/// Build alert requests and enqueue them on the sound queue.
/// Returns a short label string for UI display.
pub fn fire_alert(
    sq: &SoundQueue,
    sound_on: bool,
    voice_on: bool,
    stock_name: &str,
    kind: &AlertKind,
    amount_m: f64,
    price: f64,
    lots: u64,
    _direction: Option<&PositionDir>,
) -> String {
    let voice_msg = match kind {
        AlertKind::WhaleBuy(rank) => {
            let rank_str = match rank {
                WhaleRank::Mega => "超級大戶",
                WhaleRank::Large => "大戶",
                WhaleRank::Medium => "中型大戶",
            };
            format!(
                "注意！{} 偵測到{}買進，{:.1}百萬，價格{:.1}，{}張，建議關注做多機會",
                stock_name, rank_str, amount_m, price, lots
            )
        }
        AlertKind::WhaleSell(rank) => {
            let rank_str = match rank {
                WhaleRank::Mega => "超級大戶",
                WhaleRank::Large => "大戶",
                WhaleRank::Medium => "中型大戶",
            };
            format!(
                "注意！{} 偵測到{}賣出，{:.1}百萬，價格{:.1}，{}張，建議關注做空機會",
                stock_name, rank_str, amount_m, price, lots
            )
        }
        AlertKind::LongIgnition => {
            format!(
                "警告！{} 偵測到多頭點火，大戶連續買進推升價格，建議準備做多進場",
                stock_name
            )
        }
        AlertKind::ShortIgnition => {
            format!(
                "警告！{} 偵測到空頭點火，大戶連續賣出打壓價格，建議準備做空進場",
                stock_name
            )
        }
    };

    if sound_on {
        let req = match kind {
            AlertKind::WhaleBuy(WhaleRank::Mega) | AlertKind::WhaleSell(WhaleRank::Mega) => SoundReq::MegaWhale,
            AlertKind::WhaleBuy(_) => SoundReq::BullWhale,
            AlertKind::WhaleSell(_) => SoundReq::BearWhale,
            AlertKind::LongIgnition | AlertKind::ShortIgnition => SoundReq::Ignition,
        };
        sq.enqueue(req);
    }

    if voice_on {
        sq.enqueue(SoundReq::Voice(voice_msg));
    }

    match kind {
        AlertKind::WhaleBuy(_) => format!("BULL WHALE {:.1}M ▲", amount_m),
        AlertKind::WhaleSell(_) => format!("BEAR WHALE {:.1}M ▼", amount_m),
        AlertKind::LongIgnition => "LONG IGNITION 🔥▲".to_string(),
        AlertKind::ShortIgnition => "SHORT IGNITION 🔥▼".to_string(),
    }
}
