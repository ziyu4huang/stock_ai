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
            SoundReq::BullWhale  => play_sound(SoundKind::BullWhale),
            SoundReq::BearWhale  => play_sound(SoundKind::BearWhale),
            SoundReq::MegaWhale  => play_sound(SoundKind::MegaWhale),
            SoundReq::Ignition   => play_sound(SoundKind::Ignition),
            SoundReq::Voice(txt) => speak(txt),
        }
    }
}

// ── Platform sound helpers ───────────────────────────────────────────────────

enum SoundKind { BullWhale, BearWhale, MegaWhale, Ignition }

// ── macOS ────────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn play_sound(kind: SoundKind) {
    let path = match kind {
        SoundKind::BullWhale => "/System/Library/Sounds/Ping.aiff",
        SoundKind::BearWhale => "/System/Library/Sounds/Basso.aiff",
        SoundKind::MegaWhale => "/System/Library/Sounds/Sosumi.aiff",
        SoundKind::Ignition  => "/System/Library/Sounds/Hero.aiff",
    };
    Command::new("afplay").arg(path).spawn().ok();
}

#[cfg(target_os = "macos")]
fn speak(text: &str) {
    Command::new("say")
        .args(["-v", "Meijia", "-r", "200", text])
        .spawn().ok();
}

// ── Windows ──────────────────────────────────────────────────────────────────

#[cfg(windows)]
fn play_sound(kind: SoundKind) {
    // Use built-in Windows system sounds via PowerShell — no .wav assets needed.
    // [System.Media.SystemSounds]::X.Play() is async (non-blocking).
    let ps = match kind {
        SoundKind::BullWhale => "[System.Media.SystemSounds]::Asterisk.Play()",
        SoundKind::BearWhale => "[System.Media.SystemSounds]::Exclamation.Play()",
        SoundKind::MegaWhale => "[System.Media.SystemSounds]::Hand.Play()",
        // Two beeps for ignition
        SoundKind::Ignition  =>
            "[System.Media.SystemSounds]::Asterisk.Play(); \
             Start-Sleep -Milliseconds 350; \
             [System.Media.SystemSounds]::Asterisk.Play()",
    };
    Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps])
        .spawn().ok();
}

#[cfg(windows)]
pub fn speak(text: &str) {
    // Use WinRT (Windows.Media.SpeechSynthesis) — accesses OneCore voices like
    // Xiaoxiao (zh-CN), Zhiwei (zh-TW), Yating (zh-TW).
    // Voice priority:
    //   1. Microsoft Xiaoxiao  (zh-CN neural, if installed)
    //   2. Microsoft Zhiwei    (zh-TW, Win11 built-in OneCore)
    //   3. Microsoft Yating    (zh-TW, Win11 built-in OneCore)
    //   4. Microsoft Hanhan Desktop (zh-TW SAPI5 fallback)
    //   5. system default (no crash)
    let safe = text.replace('\'', "''").replace('"', "\\\"");
    let script = format!(
        r#"
Add-Type -AssemblyName System.Runtime.WindowsRuntime
$ag = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {{
    $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and
    $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1'
}})[0]
function Await-WinRT($task, $type) {{
    $t = $ag.MakeGenericMethod($type).Invoke($null, @($task)); $t.Wait(-1) | Out-Null; $t.Result
}}
[Windows.Media.SpeechSynthesis.SpeechSynthesizer, Windows.Media.SpeechSynthesis, ContentType=WindowsRuntime] | Out-Null
$s = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::new()
$build = [System.Environment]::OSVersion.Version.Build
$pref = if ($build -ge 22000) {{
    @('Hanhan','Xiaoxiao','Zhiwei','Yating')
}} else {{
    @('Xiaoxiao','Zhiwei','Yating','Hanhan')
}}
foreach ($p in $pref) {{
    $v = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices |
         Where-Object {{ $_.DisplayName -like "*$p*" }} | Select-Object -First 1
    if ($v) {{ $s.Voice = $v; break }}
}}
$stream = Await-WinRT ($s.SynthesizeTextToStreamAsync('{safe}')) ([Windows.Media.SpeechSynthesis.SpeechSynthesisStream])
$reader = [Windows.Storage.Streams.DataReader]::new($stream)
Await-WinRT ($reader.LoadAsync([uint32]$stream.Size)) ([uint32]) | Out-Null
$bytes = New-Object byte[] $stream.Size; $reader.ReadBytes($bytes)
$tmp = [IO.Path]::GetTempFileName() + '.wav'
[IO.File]::WriteAllBytes($tmp, $bytes)
(New-Object Media.SoundPlayer $tmp).PlaySync()
Remove-Item $tmp -ErrorAction SilentlyContinue
"#
    );
    Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .spawn().ok();
}

// ── Other / headless (Linux CI, etc.) ────────────────────────────────────────

#[cfg(not(any(target_os = "macos", windows)))]
fn play_sound(_kind: SoundKind) {}

#[cfg(not(any(target_os = "macos", windows)))]
fn speak(_text: &str) {}

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

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Windows TTS / sound tests ─────────────────────────────────────────────
    // Run with:  cargo test -p quant_trade_signal_webui tts -- --nocapture
    // These tests are gated to Windows and actually invoke PowerShell so you
    // can hear the output. They verify the command exits successfully (exit 0).

    #[test]
    #[cfg(windows)]
    fn win_sound_asterisk_exits_ok() {
        // BullWhale → SystemSounds::Asterisk
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[System.Media.SystemSounds]::Asterisk.Play(); Start-Sleep -Milliseconds 600"])
            .status()
            .expect("powershell not found");
        assert!(status.success(), "PowerShell Asterisk.Play() failed: {status}");
    }

    #[test]
    #[cfg(windows)]
    fn win_sound_exclamation_exits_ok() {
        // BearWhale → SystemSounds::Exclamation
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[System.Media.SystemSounds]::Exclamation.Play(); Start-Sleep -Milliseconds 600"])
            .status()
            .expect("powershell not found");
        assert!(status.success(), "PowerShell Exclamation.Play() failed: {status}");
    }

    #[test]
    #[cfg(windows)]
    fn win_sound_hand_exits_ok() {
        // MegaWhale → SystemSounds::Hand
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[System.Media.SystemSounds]::Hand.Play(); Start-Sleep -Milliseconds 600"])
            .status()
            .expect("powershell not found");
        assert!(status.success(), "PowerShell Hand.Play() failed: {status}");
    }

    #[test]
    #[cfg(windows)]
    fn win_sound_ignition_double_beep() {
        // Ignition → two Asterisk beeps separated by 350 ms
        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[System.Media.SystemSounds]::Asterisk.Play(); \
                    Start-Sleep -Milliseconds 350; \
                    [System.Media.SystemSounds]::Asterisk.Play(); \
                    Start-Sleep -Milliseconds 600"])
            .status()
            .expect("powershell not found");
        assert!(status.success(), "Ignition double-beep failed: {status}");
    }

    // ── Helper: query OneCore voices via WinRT ────────────────────────────────
    // Returns a newline-separated list of DisplayName values.
    #[cfg(windows)]
    fn list_onecore_voices() -> String {
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[Windows.Media.SpeechSynthesis.SpeechSynthesizer, \
                     Windows.Media.SpeechSynthesis, ContentType=WindowsRuntime] | Out-Null; \
                    [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices | \
                    ForEach-Object { $_.DisplayName }"])
            .output()
            .expect("powershell not found");
        String::from_utf8_lossy(&out.stdout).to_string()
    }

    /// List all available voices (OneCore + SAPI5) and print them.
    /// Run with --nocapture to see the output.
    #[test]
    #[cfg(windows)]
    fn win_tts_list_all_voices() {
        // OneCore (WinRT)
        let onecore = list_onecore_voices();
        println!("=== OneCore voices ===\n{onecore}");

        // SAPI5
        let out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "Add-Type -AssemblyName System.Speech; \
                    (New-Object System.Speech.Synthesis.SpeechSynthesizer)\
                    .GetInstalledVoices() | ForEach-Object { $_.VoiceInfo.Name }"])
            .output().expect("powershell not found");
        println!("=== SAPI5 voices ===\n{}", String::from_utf8_lossy(&out.stdout));
    }

    /// Smart TTS test: auto-detects the best available zh voice and speaks aloud.
    /// Priority: Xiaoxiao → Zhiwei → Yating → Hanhan Desktop → system default.
    /// You should hear the alert spoken in the best available zh voice.
    #[test]
    #[cfg(windows)]
    fn win_tts_smart_speak_best_zh_voice() {
        let onecore = list_onecore_voices();
        println!("Available OneCore voices:\n{onecore}");

        // Pick the first preferred voice that is installed
        // Detect Windows build (≥22000 = Win11)
        let build_out = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command",
                   "[System.Environment]::OSVersion.Version.Build"])
            .output().expect("powershell not found");
        let build: u32 = String::from_utf8_lossy(&build_out.stdout)
            .trim().parse().unwrap_or(0);
        let is_win11 = build >= 22000;
        println!("OS build: {build}  ({})", if is_win11 { "Windows 11" } else { "Windows 10" });

        // Win11 prefers Hanhan; Win10 prefers neural OneCore voices
        let preferred: &[&str] = if is_win11 {
            &["Hanhan", "Xiaoxiao", "Zhiwei", "Yating"]
        } else {
            &["Xiaoxiao", "Zhiwei", "Yating", "Hanhan"]
        };
        let chosen = preferred.iter()
            .find(|&&name| onecore.contains(name))
            .copied()
            .unwrap_or("(system default)");
        println!("Selected voice: Microsoft {chosen}");

        // Build WinRT speak script — selects by keyword, falls back silently
        let text = "注意！台積電偵測到大戶買進，建議關注做多機會";
        let script = format!(
            r#"
Add-Type -AssemblyName System.Runtime.WindowsRuntime
$ag = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {{
    $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and
    $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1'
}})[0]
function Await-WinRT($task, $type) {{
    $t = $ag.MakeGenericMethod($type).Invoke($null, @($task)); $t.Wait(-1) | Out-Null; $t.Result
}}
[Windows.Media.SpeechSynthesis.SpeechSynthesizer, Windows.Media.SpeechSynthesis, ContentType=WindowsRuntime] | Out-Null
$s = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::new()
$pref = @('Xiaoxiao','Zhiwei','Yating','Hanhan')
foreach ($p in $pref) {{
    $v = [Windows.Media.SpeechSynthesis.SpeechSynthesizer]::AllVoices |
         Where-Object {{ $_.DisplayName -like "*$p*" }} | Select-Object -First 1
    if ($v) {{ $s.Voice = $v; Write-Host "Using: $($v.DisplayName)"; break }}
}}
$stream = Await-WinRT ($s.SynthesizeTextToStreamAsync('{text}')) ([Windows.Media.SpeechSynthesis.SpeechSynthesisStream])
$reader = [Windows.Storage.Streams.DataReader]::new($stream)
Await-WinRT ($reader.LoadAsync([uint32]$stream.Size)) ([uint32]) | Out-Null
$bytes = New-Object byte[] $stream.Size; $reader.ReadBytes($bytes)
$tmp = [IO.Path]::GetTempFileName() + '.wav'
[IO.File]::WriteAllBytes($tmp, $bytes)
(New-Object Media.SoundPlayer $tmp).PlaySync()
Remove-Item $tmp -ErrorAction SilentlyContinue
"#
        );
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &script])
            .output()
            .expect("powershell not found");
        println!("{}", String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        assert!(output.status.success(), "Smart TTS failed: exit {:?}", output.status.code());
    }

    /// Smoke-test: SoundQueue enqueue + fire_alert label string (cross-platform).
    #[test]
    fn fire_alert_label_bull_whale() {
        let sq = SoundQueue::new();
        let label = fire_alert(
            &sq, false, false,
            "台積電",
            &crate::stock_view::AlertKind::WhaleBuy(crate::types::WhaleRank::Large),
            8.4, 840.0, 10, None,
        );
        assert_eq!(label, "BULL WHALE 8.4M ▲");
    }

    #[test]
    fn fire_alert_label_ignition() {
        let sq = SoundQueue::new();
        let label = fire_alert(
            &sq, false, false,
            "台積電",
            &crate::stock_view::AlertKind::LongIgnition,
            0.0, 840.0, 0, None,
        );
        assert_eq!(label, "LONG IGNITION 🔥▲");
    }
}
