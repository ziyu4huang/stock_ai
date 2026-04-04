use std::process::Command;

use crate::stock_view::AlertKind;
use crate::types::{WhaleRank, PositionDir};

/// Play a zh_TW voice alert using macOS `say` command with Meijia voice.
fn speak_zh_tw(text: &str) {
    Command::new("say")
        .args(["-v", "Meijia", "-r", "200", text])
        .spawn()
        .ok();
}

/// Play a bull alert sound (Ping — bright, ascending tone).
fn play_bull_sound() {
    Command::new("afplay")
        .arg("/System/Library/Sounds/Ping.aiff")
        .spawn()
        .ok();
}

/// Play a bear alert sound (Basso — low bass tone).
fn play_bear_sound() {
    Command::new("afplay")
        .arg("/System/Library/Sounds/Basso.aiff")
        .spawn()
        .ok();
}

/// Play an ignition sound (Hero — urgent alert).
fn play_ignition_sound() {
    Command::new("afplay")
        .arg("/System/Library/Sounds/Hero.aiff")
        .spawn()
        .ok();
}

/// Play a mega whale sound (Sosumi — distinctive attention grabber).
fn play_mega_sound() {
    Command::new("afplay")
        .arg("/System/Library/Sounds/Sosumi.aiff")
        .spawn()
        .ok();
}

/// Fire a sound + voice alert based on the alert kind.
/// sound_on: play afplay sound effect
/// voice_on: speak zh_TW message via `say`
pub fn fire_alert(
    sound_on: bool,
    voice_on: bool,
    stock_name: &str,
    kind: &AlertKind,
    amount_m: f64,
    price: f64,
    lots: u64,
    _direction: Option<&PositionDir>,
) -> String {
    // Build the Chinese TTS message
    let msg = match kind {
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
        match kind {
            AlertKind::WhaleBuy(WhaleRank::Mega) | AlertKind::WhaleSell(WhaleRank::Mega) => {
                play_mega_sound();
            }
            AlertKind::WhaleBuy(_) => play_bull_sound(),
            AlertKind::WhaleSell(_) => play_bear_sound(),
            AlertKind::LongIgnition | AlertKind::ShortIgnition => {
                play_ignition_sound();
            }
        }
    }

    if voice_on {
        speak_zh_tw(&msg);
    }

    // Return short label for UI display
    match kind {
        AlertKind::WhaleBuy(_) => format!("BULL WHALE {:.1}M ▲", amount_m),
        AlertKind::WhaleSell(_) => format!("BEAR WHALE {:.1}M ▼", amount_m),
        AlertKind::LongIgnition => "LONG IGNITION 🔥▲".to_string(),
        AlertKind::ShortIgnition => "SHORT IGNITION 🔥▼".to_string(),
    }
}
