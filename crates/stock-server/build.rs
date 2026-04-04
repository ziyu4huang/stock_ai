use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // webui is inside this crate: crates/stock-server/webui
    let webui_dir = Path::new(&manifest_dir).join("webui");
    let webui_src = webui_dir.join("src/app.tsx");
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

    // bun build app.tsx -> bundle.js (IIFE)
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

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Stock AI</title>
  <style>
    *, *::before, *::after {{ margin:0; padding:0; box-sizing:border-box; }}
    html, body {{ width:100%; height:100%; overflow:hidden;
                  background:#0d0e11; font-family:system-ui,-apple-system,sans-serif; color:#ccd; }}

    /* toolbar */
    #tb {{
      position:fixed; top:0; left:0; right:0; height:44px; z-index:30;
      background:#13151a; border-bottom:1px solid #21242e;
      display:flex; align-items:center; gap:6px; padding:0 12px;
    }}
    .brand {{ font-weight:700; font-size:14px; color:#5b8def; white-space:nowrap; }}
    .tsep  {{ width:1px; height:18px; background:#21242e; margin:0 2px; }}
    .pbtn {{
      background:#1a1d25; color:#99aabb; border:1px solid #272c38;
      padding:4px 10px; border-radius:5px; font-size:11px; cursor:pointer;
    }}
    .pbtn:hover {{ background:#222733; color:#dde; }}
    .pbtn.active {{ background:#1a2a50; color:#6a9eff; border-color:#2a4080; }}
    #bt-btn {{
      background:#2a1a10; color:#e8962a; border:1px solid #4a3018;
      padding:4px 10px; border-radius:5px; font-size:11px; cursor:pointer;
    }}
    #bt-btn:hover {{ background:#3a2818; }}
    #rp-btn {{
      background:#1a2a20; color:#3dbb6a; border:1px solid #1a4a28;
      padding:4px 10px; border-radius:5px; font-size:11px; cursor:pointer;
    }}
    #rp-btn:hover {{ background:#2a3a28; }}
    .ind-btn {{
      background:#1a1d25; color:#7788aa; border:1px solid #272c38;
      padding:4px 9px; border-radius:5px; font-size:11px; cursor:pointer;
    }}
    .ind-btn:hover {{ background:#222733; color:#dde; }}
    .ind-btn.active {{ background:#1a2a4a; color:#7ab0ff; border-color:#2a4080; }}
    #tb-right {{ margin-left:auto; display:flex; gap:6px; align-items:center; }}
    #wl-toggle {{
      display:none; background:#1a1d25; color:#99aabb; border:1px solid #272c38;
      padding:4px 8px; border-radius:5px; font-size:13px; cursor:pointer;
    }}

    /* watchlist sidebar */
    #watchlist {{
      position:fixed; top:44px; left:0; width:220px; bottom:0; z-index:20;
      background:#0f1014; border-right:1px solid #181b22;
      display:flex; flex-direction:column; overflow:hidden;
    }}
    #wl-header {{
      padding:8px 10px; border-bottom:1px solid #181b22;
      font-size:11px; font-weight:600; color:#778; text-transform:uppercase; letter-spacing:.5px;
    }}
    #wl-add {{
      display:flex; gap:4px; padding:6px 8px; border-bottom:1px solid #181b22;
    }}
    #wl-add-input {{
      flex:1; background:#1a1d25; color:#dde; border:1px solid #272c38;
      padding:4px 8px; border-radius:4px; font-size:11px; outline:none;
    }}
    #wl-add-input:focus {{ border-color:#5b8def; }}
    #wl-add-btn {{
      background:#1a2a50; color:#6a9eff; border:1px solid #2a4080;
      padding:4px 8px; border-radius:4px; font-size:11px; cursor:pointer;
    }}
    #wl-items {{ flex:1; overflow-y:auto; padding:4px 0; }}
    .wl-item {{
      display:flex; align-items:center; gap:6px; padding:5px 10px;
      cursor:pointer; font-size:12px; color:#99aabb; transition:background .15s;
    }}
    .wl-item:hover {{ background:#161922; color:#dde; }}
    .wl-sym {{ font-weight:600; min-width:60px; }}
    .wl-name {{ flex:1; font-size:10px; color:#556; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }}
    .wl-dot {{ width:6px; height:6px; border-radius:50%; flex-shrink:0; }}
    .wl-del {{ color:#334; font-size:11px; cursor:pointer; padding:0 2px; }}
    .wl-del:hover {{ color:#e84848; }}
    #scan-area {{ padding:6px 8px; border-top:1px solid #181b22; }}
    #scan-btn {{
      width:100%; background:#1a2a20; color:#3dbb6a; border:1px solid #1a4a28;
      padding:5px; border-radius:4px; font-size:11px; cursor:pointer; font-weight:600;
    }}
    #scan-btn:hover {{ background:#2a3a28; }}
    #scan-btn:disabled {{ opacity:.5; cursor:wait; }}
    #scan-results {{ padding:4px 8px; }}

    /* scanner view */
    #scanner-view {{
      display:none; position:fixed; top:44px; left:220px; right:0; bottom:0; z-index:18;
      background:#0d0e11; flex-direction:column; overflow:hidden;
    }}
    #scanner-bar {{
      display:flex; align-items:center; gap:6px; padding:6px 12px;
      background:#13151a; border-bottom:1px solid #21242e;
    }}
    .sbtn {{
      background:#1a1d25; color:#99aabb; border:1px solid #272c38;
      padding:4px 12px; border-radius:5px; font-size:11px; cursor:pointer;
    }}
    .sbtn:hover {{ background:#222733; color:#dde; }}
    .sbtn.active {{ background:#1a2a50; color:#6a9eff; border-color:#2a4080; }}
    #scan-all-btn {{
      margin-left:auto; background:#1a2a20; color:#3dbb6a; border:1px solid #1a4a28;
      padding:4px 12px; border-radius:5px; font-size:11px; cursor:pointer; font-weight:600;
    }}
    #scan-all-btn:hover {{ background:#2a3a28; }}
    #scan-all-btn:disabled {{ opacity:.5; cursor:wait; }}
    #scan-table {{
      flex:1; overflow-y:auto; padding:0;
    }}
    .scan-row {{
      display:flex; align-items:center; gap:10px; padding:8px 14px;
      border-bottom:1px solid #181b22; cursor:pointer; transition:background .15s;
    }}
    .scan-row:hover {{ background:#161922; }}
    .scan-sym {{ font-weight:700; min-width:70px; color:#dde; }}
    .scan-name {{ min-width:60px; color:#556; font-size:10px; }}
    .scan-price {{ min-width:60px; color:#99a; font-size:12px; }}
    .scan-score {{
      font-weight:700; font-size:14px; min-width:40px; text-align:right;
      padding:2px 8px; border-radius:4px;
    }}
    .scan-dir {{
      font-weight:700; font-size:11px; min-width:40px; text-align:center;
      padding:2px 8px; border-radius:3px;
    }}
    .scan-sigs {{ flex:1; font-size:10px; color:#667; overflow:hidden; text-overflow:ellipsis; white-space:nowrap; }}
    #scanner-chart-wrap {{ flex:1; display:none; flex-direction:column; overflow:hidden; position:relative; }}
    #scanner-chart {{ flex:1; width:100%; }}
    #replay-bar {{
      display:none; align-items:center; gap:8px; padding:6px 12px;
      background:#13151a; border-top:1px solid #21242e;
    }}
    .rbtn {{
      background:#1a1d25; color:#99aabb; border:1px solid #272c38;
      padding:3px 10px; border-radius:4px; font-size:11px; cursor:pointer;
    }}
    .rbtn:hover {{ background:#222733; color:#dde; }}
    #replay-speed {{ width:80px; }}
    #replay-stats {{ font-size:11px; color:#778; margin-left:8px; }}
    #signal-detail {{
      position:absolute; bottom:50px; right:10px; z-index:25;
      background:#13151acc; border:1px solid #21242e; border-radius:6px;
      padding:10px; min-width:240px; max-width:360px; display:none;
      box-shadow:0 4px 16px rgba(0,0,0,.5);
    }}
    #signal-detail h4 {{ font-size:12px; color:#dde; margin-bottom:6px; }}
    .sd-item {{ font-size:11px; padding:2px 0; }}
    .sd-item .sd-kind {{ font-weight:600; }}
    .sd-item .sd-reason {{ color:#667; }}

    /* main layout */
    #main {{
      position:fixed; top:44px; left:220px; right:0; bottom:0;
      display:flex; overflow:hidden;
    }}

    /* chart area */
    #chart-wrap {{ flex:1; position:relative; overflow:hidden; }}
    #chart {{ width:100%; height:100%; }}
    #overlay {{
      position:absolute; inset:0; display:none; align-items:center; justify-content:center;
      font-size:13px; color:#556; z-index:5;
    }}

    /* sidebar stats */
    #stats {{
      width:280px; background:#13151a; border-left:1px solid #1e2028;
      padding:14px; overflow-y:auto;
    }}
    .sl {{ font-size:10px; color:#445; text-transform:uppercase; letter-spacing:.6px; margin-top:12px; }}
    .sl:first-child {{ margin-top:0; }}
    .sv {{ font-size:20px; font-weight:700; color:#dde; margin-top:1px; }}
    .ss {{ font-size:11px; color:#667; margin-top:1px; }}
    .up {{ color:#3dbb6a; }}
    .dn {{ color:#e84848; }}

    /* signal panel */
    #signal-panel {{ margin-top:12px; }}
    #signal-list {{ max-height:200px; overflow-y:auto; }}
    .signal-item {{
      display:flex; align-items:center; gap:6px; padding:4px 8px; margin:2px 0;
      border-radius:4px; font-size:11px;
    }}
    .signal-type {{ font-weight:700; min-width:40px; }}
    .signal-date {{ color:#667; flex:1; }}
    .signal-conf {{ color:#99a; }}

    /* backtest panel */
    #backtest-panel {{
      position:fixed; top:44px; right:280px; width:340px; max-height:calc(100vh - 54px);
      background:#13151a; border:1px solid #21242e; border-radius:8px;
      box-shadow:0 6px 24px rgba(0,0,0,.5); z-index:40; display:none; flex-direction:column;
      overflow:hidden;
    }}
    #bt-header {{
      display:flex; align-items:center; justify-content:space-between;
      padding:8px 12px; border-bottom:1px solid #1e2028;
      font-weight:600; font-size:13px; color:#dde;
    }}
    #bt-close {{ background:none; border:none; color:#556; cursor:pointer; font-size:16px; }}
    #bt-content {{ padding:12px; overflow-y:auto; }}

    /* live 1m bar */
    #live-bar {{
      display:none; position:fixed; top:44px; left:220px; right:0; height:28px; z-index:19;
      background:#0d1117; border-bottom:1px solid #1e2028;
      align-items:center; gap:10px; padding:0 12px;
    }}
    #live-regime {{
      font-size:11px; font-weight:700; padding:2px 10px; border-radius:12px;
      border:1px solid #334; color:#6b7280; background:#6b728018;
      white-space:nowrap;
    }}
    #live-status {{ font-size:10px; color:#445; }}
    #live-states {{ font-size:10px; color:#445; flex:1; }}
    #live-refresh {{
      background:#1a2a20; color:#3dbb6a; border:1px solid #1a4a28;
      padding:2px 8px; border-radius:4px; font-size:10px; cursor:pointer;
    }}

    /* config panel */
    #cfg-btn {{
      background:#1a1d25; color:#99aabb; border:1px solid #272c38;
      padding:4px 8px; border-radius:5px; font-size:13px; cursor:pointer;
    }}
    #cfg-btn:hover {{ background:#222733; color:#dde; }}
    #cfg-panel {{
      display:none; position:fixed; top:38px; right:12px; z-index:60;
      background:#13151a; border:1px solid #21242e; border-radius:8px;
      box-shadow:0 6px 24px rgba(0,0,0,.5); padding:14px; min-width:220px;
    }}
    #cfg-panel.open {{ display:block; }}
    #cfg-panel label {{ font-size:10px; color:#556; text-transform:uppercase; letter-spacing:.5px; display:block; margin-bottom:4px; }}
    #cfg-panel select {{
      width:100%; background:#1a1d25; color:#dde; border:1px solid #272c38;
      padding:6px 8px; border-radius:4px; font-size:12px; outline:none;
    }}
    #cfg-panel select:focus {{ border-color:#5b8def; }}
    #cfg-status {{ font-size:10px; color:#3dbb6a; margin-top:6px; min-height:14px; }}

    /* ── mobile (≤768px) ─────────────────────────────────────── */
    @media (max-width: 768px) {{
      #wl-toggle {{ display:block; }}
      #watchlist {{
        transform: translateX(-100%); transition: transform .2s;
        top:44px; z-index:50; box-shadow:4px 0 16px rgba(0,0,0,.6);
      }}
      #watchlist.open {{ transform: translateX(0); }}
      #main {{ left:0 !important; flex-direction:column; top:44px; }}
      #chart-wrap {{ flex:1 1 60%; min-height:0; }}
      #stats {{
        width:100% !important; border-left:none; border-top:1px solid #1e2028;
        flex:0 0 auto; max-height:40vh; overflow-y:auto;
      }}
      #backtest-panel {{ right:0; left:0; width:auto; }}
      #tb {{ flex-wrap:wrap; height:auto; padding:6px 8px; gap:4px; }}
      #tb-right {{ flex-wrap:wrap; }}
      .tsep {{ display:none; }}
    }}
  </style>
</head>
<body>

<div id="tb">
  <button id="wl-toggle" onclick="toggleWatchlist()">☰</button>
  <span class="brand">Stock AI</span>
  <div class="tsep"></div>
  <button class="pbtn period-btn" onclick="toggleLive1d(this)">1D</button>
  <button class="pbtn active period-btn" onclick="setPeriod(7,this)">1W</button>
  <button class="pbtn period-btn" onclick="setPeriod(30,this)">1M</button>
  <button class="pbtn period-btn" onclick="setPeriod(90,this)">3M</button>
  <button class="pbtn period-btn" onclick="setPeriod(180,this)">6M</button>
  <button class="pbtn period-btn" onclick="setPeriod(365,this)">1Y</button>
  <div class="tsep"></div>
  <button class="ind-btn" onclick="toggleIndicator('rsi',this)">RSI</button>
  <button class="ind-btn" onclick="toggleIndicator('macd',this)">MACD</button>
  <button class="ind-btn" onclick="toggleIndicator('boll',this)">BB</button>
  <div class="tsep"></div>
  <button class="pbtn" id="scanner-btn" onclick="toggleScanner()">Scanner</button>
  <div id="tb-right">
    <button id="bt-btn" onclick="runBacktest()">HMM Backtest</button>
    <button id="rp-btn" onclick="openReport()">HTML Report</button>
    <button id="cfg-btn" onclick="toggleConfig()">&#9881;</button>
  </div>
</div>

<div id="live-bar">
  <span id="live-regime">NOISE 雜訊震盪</span>
  <span id="live-states"></span>
  <span id="live-status">--</span>
  <button id="live-refresh" onclick="(window.refresh1mNow||function(){{}})()">⟳</button>
</div>

<div id="watchlist">
  <div id="wl-header">Watchlist</div>
  <div id="wl-add">
    <input id="wl-add-input" placeholder="Add symbol...">
    <button id="wl-add-btn" onclick="addToWatchlist()">+</button>
  </div>
  <div id="wl-items"></div>
  <div id="scan-area">
    <button id="scan-btn" onclick="scanAll()">Scan All</button>
    <div id="scan-results"></div>
  </div>
</div>

<div id="main">
  <div id="chart-wrap">
    <div id="chart"></div>
    <div id="overlay">Loading...</div>
  </div>
  <div id="stats">
    <div class="sl">Symbol</div>
    <div class="sv" id="s-sym">--</div>
    <div class="sl">Price</div>
    <div class="sv" id="s-price">--</div>
    <div class="ss" id="s-change">--</div>
    <div class="sl">Volume</div>
    <div class="sv" id="s-vol" style="font-size:15px">--</div>
    <div class="sl">High / Low</div>
    <div class="sv" id="s-hl" style="font-size:14px">--</div>
    <div class="sl">RSI (14)</div>
    <div class="sv" id="s-rsi">--</div>
    <div class="sl">MACD</div>
    <div class="sv" id="s-macd" style="font-size:13px">--</div>
    <div class="sl">Signals</div>
    <div id="signal-panel">
      <div id="signal-list"><div style="color:#445;font-size:11px">No signals yet</div></div>
    </div>
    <div class="sl" id="signal-legend-title" style="display:none">Signal Legend (1D)</div>
    <div id="signal-legend" style="display:none">
      <div style="display:flex;align-items:center;gap:5px;margin:2px 0">
        <span style="color:#22c55e;font-size:14px">&#9650;</span>
        <span style="font-size:10px;color:#99a">Buy — RSI crossed below 30 (oversold)</span>
      </div>
      <div style="display:flex;align-items:center;gap:5px;margin:2px 0">
        <span style="color:#ef4444;font-size:14px">&#9660;</span>
        <span style="font-size:10px;color:#99a">Sell — RSI crossed above 70 (overbought)</span>
      </div>
      <div style="display:flex;align-items:center;gap:5px;margin:2px 0">
        <span style="color:#22c55e;font-size:14px">&#9650;</span>
        <span style="font-size:10px;color:#99a">Buy — MACD histogram crossed positive</span>
      </div>
      <div style="display:flex;align-items:center;gap:5px;margin:2px 0">
        <span style="color:#ef4444;font-size:14px">&#9660;</span>
        <span style="font-size:10px;color:#99a">Sell — MACD histogram crossed negative</span>
      </div>
      <div style="margin-top:4px;padding-top:4px;border-top:1px solid #1e2028">
        <div style="font-size:10px;color:#556">Click any marker on chart to see detail</div>
      </div>
    </div>
  </div>
</div>

<div id="scanner-view">
  <div id="scanner-bar">
    <button class="sbtn active" id="sub-list-btn" onclick="setScannerSub('list')">Scan List</button>
    <button class="sbtn" id="sub-analysis-btn" onclick="setScannerSub('analysis')">Analysis</button>
    <button class="sbtn" id="sub-replay-btn" onclick="setScannerSub('replay')">Replay</button>
    <div style="flex:1"></div>
    <button class="rbtn" id="scan-all-btn2" onclick="loadScanResults()">Scan All</button>
    <button class="sbtn" onclick="toggleScanner()" style="color:#e84848">Close</button>
  </div>
  <div id="scan-table" style="flex:1;overflow-y:auto;padding:8px;"></div>
  <div id="scanner-chart-wrap" style="flex:1;display:none;position:relative;">
    <div id="scanner-chart" style="width:100%;height:100%;"></div>
    <div id="replay-controls" style="display:none;position:absolute;bottom:40px;left:12px;right:12px;display:none;align-items:center;gap:8px;padding:6px 10px;background:#13151acc;border-radius:6px;">
      <button class="sbtn" onclick="replayPlay()">&#9654; Play</button>
      <button class="sbtn" onclick="replayPause()">&#9208; Pause</button>
      <button class="sbtn" onclick="replayStep()">&#8594; Step</button>
      <span style="font-size:10px;color:#556">Speed:</span>
      <input id="replay-speed" type="range" min="100" max="3000" value="1000" step="100" onchange="setReplaySpeed(this.value)">
      <span id="replay-stats"></span>
    </div>
    <div id="signal-detail">
      <h4 id="sd-title">Signals</h4>
      <div id="sd-list"></div>
    </div>
  </div>
</div>

<div id="backtest-panel">
  <div id="bt-header"><span>HMM Regime Analysis</span><button id="bt-close" onclick="document.getElementById('backtest-panel').style.display='none'">&times;</button></div>
  <div id="bt-content">Click HMM Backtest to run analysis</div>
</div>

<div id="cfg-panel">
  <label>Data Source</label>
  <select id="cfg-backend" onchange="setConfig(this.value)">
    <option value="yahoo">Yahoo (auto-fallback to AV)</option>
    <option value="yahoo-only">Yahoo only</option>
    <option value="av">Alpha Vantage only</option>
  </select>
  <div id="cfg-status"></div>
</div>

<script>{bundle_js}</script>
</body>
</html>"#);

    fs::write(&html_path, html).expect("failed to write webui.html");

    println!("cargo:rerun-if-changed=webui/src/app.tsx");
    println!("cargo:rerun-if-changed=webui/package.json");
}

#[cfg(not(windows))]
fn dirs_home_bun() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".into());
    format!("{home}/.bun/bin/bun")
}
