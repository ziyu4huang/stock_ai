use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let webui_dir = Path::new(&manifest_dir).join("webui");
    let webui_src = webui_dir.join("src/app.ts");
    let out_dir = env::var("OUT_DIR").unwrap();
    let bundle_path = Path::new(&out_dir).join("bundle.js");
    let html_path = Path::new(&out_dir).join("webui.html");

    let bun = if cfg!(windows) {
        "C:/Users/ziyu4/.bun/bin/bun.exe".to_string()
    } else {
        dirs_home_bun()
    };

    // bun install
    let status = Command::new(&bun)
        .args(["install"])
        .current_dir(&webui_dir)
        .status()
        .expect("failed to run bun install");
    assert!(status.success(), "bun install failed");

    // bun build app.ts -> bundle.js (IIFE)
    let status = Command::new(&bun)
        .args([
            "build",
            webui_src.to_str().unwrap(),
            "--outfile",
            bundle_path.to_str().unwrap(),
            "--target",
            "browser",
            "--format",
            "iife",
            "--minify",
        ])
        .current_dir(&manifest_dir)
        .status()
        .expect("failed to run bun build");
    assert!(status.success(), "bun build failed");

    let bundle_js = fs::read_to_string(&bundle_path)
        .expect("failed to read bundle.js")
        .replace("</script>", r"<\/script>");

    let html = format!(r##"<!DOCTYPE html>
<html lang="zh-TW">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>WHALE RADAR</title>
<style>
  :root {{
    --bg: #0a0a0a; --bg2: #111; --bg3: #1a1a1a;
    --border: #333; --text: #ccc; --dim: #666;
    --green: #4caf50; --lgreen: #69f0ae; --red: #ef5350; --lred: #ff5252;
    --cyan: #00bcd4; --yellow: #ffd600; --magenta: #e040fb;
    --btn-hover: #252525; --btn-active: #333;
    --tab-h: 36px; --status-h: 24px;
  }}
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{ background: var(--bg); color: var(--text); font-family: 'Menlo','SF Mono','Consolas',monospace; font-size: 12px; overflow: hidden; height: 100vh; display: flex; flex-direction: column; }}

  /* ── Tab bar ─────────────────────────────────────── */
  .tab-bar {{ display: flex; align-items: center; background: var(--bg2); border-bottom: 1px solid var(--border); padding: 4px 8px; gap: 4px; flex-shrink: 0; min-height: var(--tab-h); }}
  .tab-bar .logo {{ color: var(--text); font-size: 13px; font-weight: bold; margin-right: 8px; white-space: nowrap; letter-spacing: 0.5px; }}
  #tabs {{ display: flex; gap: 4px; }}
  .tab-btn {{ background: transparent; border: 1px solid transparent; color: var(--dim); padding: 4px 10px; cursor: pointer; font-family: inherit; font-size: 11px; border-radius: 4px; transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.08s ease; user-select: none; }}
  .tab-btn:hover {{ background: var(--btn-hover); color: var(--text); }}
  .tab-btn:active {{ transform: scale(0.96); }}
  .tab-btn.active {{ background: #fff; color: #000; font-weight: bold; border-color: #fff; }}
  .tab-btn.has-signal {{ color: var(--lred); font-weight: bold; animation: pulse-tab 1.5s ease-in-out infinite; }}
  @keyframes pulse-tab {{ 0%,100% {{ opacity: 1; }} 50% {{ opacity: 0.7; }} }}
  .tab-sep {{ flex: 1; }}

  /* ── Toolbar buttons ─────────────────────────────── */
  .toolbar-btn {{ background: transparent; border: 1px solid var(--border); color: var(--dim); padding: 3px 8px; cursor: pointer; font-family: inherit; font-size: 11px; border-radius: 4px; transition: background 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.08s ease; user-select: none; display: inline-flex; align-items: center; gap: 3px; }}
  .toolbar-btn:hover {{ background: var(--btn-hover); color: var(--text); border-color: #555; }}
  .toolbar-btn:active {{ background: var(--btn-active); transform: scale(0.96); }}
  .toolbar-btn.on {{ border-color: #555; color: var(--text); }}
  .toolbar-btn.pause-active {{ background: rgba(255,214,0,0.15); border-color: var(--yellow); color: var(--yellow); }}

  /* ── Main 3-column (CSS Grid) ────────────────────── */
  .main {{ display: grid; grid-template-columns: 38% 22% 40%; flex: 1; overflow: hidden; }}
  .col-left, .col-mid, .col-right {{ display: flex; flex-direction: column; overflow: hidden; }}
  .col-left, .col-mid {{ border-right: 1px solid var(--border); }}

  .panel {{ border: 1px solid var(--border); display: flex; flex-direction: column; overflow: hidden; }}
  .panel-header {{ background: var(--bg2); padding: 3px 8px; font-size: 11px; color: var(--dim); border-bottom: 1px solid var(--border); flex-shrink: 0; display: flex; align-items: center; justify-content: space-between; }}
  .panel-body {{ padding: 4px 6px; overflow-y: auto; flex: 1; scroll-behavior: smooth; }}

  /* ── Order book ──────────────────────────────────── */
  .ob-row {{ display: flex; align-items: center; padding: 2px 4px; border-radius: 3px; transition: background 0.1s; cursor: default; }}
  .ob-row:hover {{ background: rgba(255,255,255,0.04); }}
  .ob-label {{ width: 50px; color: var(--dim); }}
  .ob-price {{ width: 80px; text-align: right; }}
  .ob-lots {{ width: 60px; text-align: right; }}
  .ob-bar {{ flex: 1; height: 12px; margin-left: 6px; border-radius: 3px; transition: width 0.3s ease; }}
  .ob-delta {{ width: 44px; text-align: right; font-size: 10px; font-weight: bold; flex-shrink: 0; }}
  .ob-row.suspicious {{ background: rgba(255,214,0,0.06); border-left: 2px solid var(--yellow); }}
  .cum-delta-label {{ display: flex; justify-content: space-between; padding: 2px 4px; font-size: 10px; }}
  .cum-bar {{ margin: 2px 0; }}
  .ask .ob-price, .ask .ob-label {{ color: var(--red); }}
  .ask .ob-bar {{ background: rgba(239,83,80,0.3); }}
  .bid .ob-price, .bid .ob-label {{ color: var(--green); }}
  .bid .ob-bar {{ background: rgba(76,175,80,0.3); }}
  .spread-row {{ text-align: center; padding: 4px 0; color: #fff; font-weight: bold; font-size: 13px; border-top: 1px solid var(--border); border-bottom: 1px solid var(--border); }}
  .pressure-row {{ text-align: center; padding: 3px 0; font-size: 11px; }}
  .pressure-bar {{ display: flex; height: 6px; border-radius: 3px; overflow: hidden; margin-top: 2px; }}
  .pressure-bar .bid-bar {{ background: var(--green); transition: width 0.3s; }}
  .pressure-bar .ask-bar {{ background: var(--red); transition: width 0.3s; }}

  /* ── Order book delta & spoof indicators ────────── */
  .ob-delta {{ width: 40px; text-align: right; font-size: 10px; font-weight: bold; }}
  .ob-row.suspicious {{ background: rgba(255,214,0,0.08); border-left: 2px solid var(--yellow); }}
  .ob-row.suspicious .ob-label {{ color: var(--yellow); }}
  .cum-delta-label {{ display: flex; justify-content: space-between; padding: 4px 4px 2px; font-size: 10px; }}
  .cum-bar {{ margin-top: 2px; }}
  .spread-row {{ text-align: center; padding: 4px 0; color: #fff; font-weight: bold; font-size: 13px; border-top: 1px solid var(--border); border-bottom: 1px solid var(--border); }}
  .spoof-alert {{ background: rgba(255,214,0,0.1); border: 1px solid rgba(255,214,0,0.3); border-radius: 4px; padding: 4px 8px; margin: 4px 0; font-size: 11px; color: var(--yellow); }}
  .conf-bar-wrap {{ margin: 4px 0; }}
  .conf-label {{ font-size: 11px; display: flex; justify-content: space-between; }}
  .conf-bar {{ height: 10px; background: var(--bg3); border-radius: 3px; overflow: hidden; margin: 2px 0; }}
  .conf-fill {{ height: 100%; border-radius: 3px; transition: width 0.3s ease; }}
  .velocity-ind {{ font-size: 10px; color: var(--dim); padding: 2px 0; }}
  .velocity-ind.elevated {{ color: var(--yellow); }}
  .velocity-ind.spike {{ color: var(--red); font-weight: bold; }}

  /* ── Signal radar ────────────────────────────────── */
  .score-bar-wrap {{ margin: 6px 0; }}
  .score-label {{ font-size: 18px; font-weight: bold; }}
  .score-bar {{ height: 16px; background: var(--bg3); border-radius: 4px; position: relative; margin: 3px 0; overflow: hidden; }}
  .score-bar .mid {{ position: absolute; left: 50%; top: 0; bottom: 0; width: 2px; background: #444; z-index: 1; }}
  .score-bar .fill {{ position: absolute; top: 0; bottom: 0; border-radius: 4px; transition: all 0.3s ease; }}
  .market-state {{ font-size: 15px; font-weight: bold; padding: 4px 0; }}
  .hmm-state {{ font-size: 12px; padding: 2px 0; }}
  .hmm-agree {{ color: var(--green); font-size: 10px; margin-left: 6px; }}
  .hmm-div {{ color: var(--yellow); font-size: 10px; margin-left: 6px; }}
  .action-box {{ border: 1px solid var(--border); border-radius: 6px; padding: 8px; margin: 6px 0; transition: border-color 0.3s; }}
  .action-label {{ font-size: 15px; font-weight: bold; text-align: center; padding: 2px; }}
  .sl-label {{ font-size: 11px; color: var(--yellow); text-align: center; margin-top: 2px; }}
  .counter-row {{ display: flex; align-items: center; padding: 2px 0; font-size: 11px; }}
  .counter-label {{ width: 80px; color: var(--dim); }}
  .counter-bar {{ flex: 1; height: 8px; border-radius: 3px; margin: 0 4px; transition: width 0.3s ease; }}
  .counter-val {{ width: 24px; text-align: right; font-weight: bold; }}
  .price-row {{ text-align: center; padding: 6px 0; font-size: 14px; }}
  .sound-row {{ font-size: 11px; color: var(--cyan); padding: 2px 0; }}

  /* ── Alerts ──────────────────────────────────────── */
  .alert-item {{ padding: 3px 4px; border-bottom: 1px solid rgba(255,255,255,0.05); border-radius: 3px; transition: background 0.15s; cursor: pointer; }}
  .alert-item:hover {{ background: rgba(255,255,255,0.04); }}
  .alert-time {{ color: var(--dim); font-size: 10px; }}
  .alert-line1 {{ font-size: 11px; }}
  .alert-line2 {{ font-size: 10px; color: var(--dim); }}
  .alert-item.ignition {{ border-left: 3px solid; padding-left: 6px; }}
  .alert-item.ignition.bull {{ border-left-color: var(--lgreen); }}
  .alert-item.ignition.bear {{ border-left-color: var(--lred); }}

  /* ── Tick feed ───────────────────────────────────── */
  .tick-item {{ display: flex; align-items: center; padding: 2px 4px; font-size: 11px; border-radius: 3px; transition: background 0.1s; }}
  .tick-item:hover {{ background: rgba(255,255,255,0.04); }}
  .tick-item.whale {{ background: rgba(255,255,255,0.02); }}
  .tick-time {{ color: var(--dim); width: 64px; }}
  .tick-arrow {{ width: 14px; text-align: center; font-weight: bold; }}
  .tick-price {{ width: 70px; text-align: right; }}
  .tick-tag {{ width: 60px; font-size: 10px; font-weight: bold; }}
  .tick-amount {{ width: 50px; text-align: right; }}
  .tick-shares {{ color: var(--dim); }}

  /* ── Status bar ──────────────────────────────────── */
  .status-bar {{ background: var(--bg2); border-top: 1px solid var(--border); padding: 3px 10px; font-size: 10px; color: var(--dim); display: flex; align-items: center; gap: 12px; flex-shrink: 0; }}
  .status-dot {{ width: 7px; height: 7px; border-radius: 50%; display: inline-block; margin-right: 4px; }}
  .status-dot.live {{ background: var(--green); animation: blink-dot 2s ease-in-out infinite; }}
  .status-dot.paused {{ background: var(--yellow); }}
  .status-dot.disconnected {{ background: var(--red); }}
  @keyframes blink-dot {{ 0%,100% {{ opacity: 1; }} 50% {{ opacity: 0.4; }} }}

  /* ── Scrollbar ───────────────────────────────────── */
  ::-webkit-scrollbar {{ width: 5px; }}
  ::-webkit-scrollbar-track {{ background: transparent; }}
  ::-webkit-scrollbar-thumb {{ background: #333; border-radius: 3px; }}
  ::-webkit-scrollbar-thumb:hover {{ background: #444; }}

  /* ── Responsive ──────────────────────────────────── */
  @media (max-width: 900px) {{
    .main {{ grid-template-columns: 1fr; grid-template-rows: 1fr 1fr 1fr; }}
    .col-left, .col-mid {{ border-right: none; border-bottom: 1px solid var(--border); }}
  }}
</style>
</head>
<body>

<!-- Tab bar -->
<div class="tab-bar">
  <span class="logo">WHALE RADAR</span>
  <div id="tabs"></div>
  <span class="tab-sep"></span>
  <button id="ind-auto" class="toolbar-btn on" title="Auto-switch to ignition tab">A</button>
  <button id="ind-sound" class="toolbar-btn on" title="Toggle sound alerts"></button>
  <button id="ind-voice" class="toolbar-btn on" title="Toggle voice alerts"></button>
  <button id="ind-pause" class="toolbar-btn" title="Pause/Resume feed"></button>
  <button id="btn-clear" class="toolbar-btn" title="Clear alerts">Clear</button>
  <button id="ind-lang" class="toolbar-btn" title="Toggle language"></button>
</div>

<!-- Main 3-column -->
<div class="main">
  <!-- Left: Order book -->
  <div class="col-left">
    <div class="panel" style="flex:1">
      <div class="panel-header"><span id="ob-title">ORDER BOOK</span><span id="ob-spread" style="font-size:10px"></span></div>
      <div class="panel-body" id="order-book"></div>
    </div>
  </div>
  <!-- Middle: Signal radar + alerts -->
  <div class="col-mid">
    <div class="panel" style="flex:55">
      <div class="panel-header" id="signal-radar-title">SIGNAL RADAR</div>
      <div class="panel-body" id="signal-radar"></div>
    </div>
    <div class="panel" style="flex:45">
      <div class="panel-header"><span id="alerts-title">ALERTS</span> <span id="alert-count" style="color:var(--yellow);margin-left:4px"></span></div>
      <div class="panel-body" id="alerts"></div>
    </div>
  </div>
  <!-- Right: Tick feed -->
  <div class="col-right">
    <div class="panel" style="flex:1">
      <div class="panel-header" id="tick-title">TICK FEED</div>
      <div class="panel-body" id="tick-feed"></div>
    </div>
  </div>
</div>

<!-- Status bar -->
<div class="status-bar">
  <span style="display:flex;align-items:center"><span id="status-dot" class="status-dot live"></span><span id="status-label">LIVE</span></span>
  <span id="status-conn">WS: connected</span>
  <span style="flex:1"></span>
  <span id="status-events">Ev:0</span>
  <span id="status-time"></span>
</div>

<script>{bundle_js}</script>
</body>
</html>"##);

    fs::write(&html_path, html).expect("failed to write webui.html");

    println!("cargo:rerun-if-changed=webui/src/app.ts");
    println!("cargo:rerun-if-changed=webui/src/types.ts");
    println!("cargo:rerun-if-changed=webui/src/state.ts");
    println!("cargo:rerun-if-changed=webui/src/dom.ts");
    println!("cargo:rerun-if-changed=webui/src/tabs.ts");
    println!("cargo:rerun-if-changed=webui/src/orderbook.ts");
    println!("cargo:rerun-if-changed=webui/src/radar.ts");
    println!("cargo:rerun-if-changed=webui/src/alerts.ts");
    println!("cargo:rerun-if-changed=webui/src/tickfeed.ts");
    println!("cargo:rerun-if-changed=webui/src/status.ts");
    println!("cargo:rerun-if-changed=webui/src/keyboard.ts");
    println!("cargo:rerun-if-changed=webui/src/i18n.ts");
    println!("cargo:rerun-if-changed=webui/package.json");
}

#[cfg(not(windows))]
fn dirs_home_bun() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    format!("{home}/.bun/bin/bun")
}
