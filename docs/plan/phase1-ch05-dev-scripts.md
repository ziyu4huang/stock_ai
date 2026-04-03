# Ch 5 — Dev Scripts & Tooling

> Convenience scripts for the Rust + Python + Bun development workflow.

---

## 5.1 scripts/setup.sh — First-Time Setup

```bash
#!/bin/bash
# Check prerequisites
command -v cargo  || { echo "Install Rust: https://rustup.rs"; exit 1; }
command -v bun    || { echo "Install Bun: https://bun.sh"; exit 1; }
command -v python3 || { echo "Install Python 3.9+"; exit 1; }

# Python deps
pip3 install -r requirements.txt

# Create data directory
mkdir -p ~/.stock_ai

# Build
cargo build

echo "Setup complete. Run: scripts/run.sh"
```

---

## 5.2 scripts/run.sh — Start Dev Server

```bash
#!/bin/bash
# Option A: plain cargo run
cargo run

# Option B: with auto-reload (if cargo-watch installed)
# cargo watch -x run
```

Single binary — no microservice orchestration needed. One process handles everything.

---

## 5.3 scripts/README.md — Command Reference

### Build & Run
```bash
# Build (also bundles frontend via build.rs)
cargo build

# Run (starts server on :3003)
cargo run

# Build release
cargo build --release
```

### Frontend Dev
```bash
# Install/update JS deps
cd webui && bun install

# Build frontend only (for debugging)
cd webui && bun build src/app.tsx --format iife --target browser --outfile /tmp/test.js
```

### Python CLI
```bash
# Full analysis
python3 -m quant_cli analyze TSLA --source yfinance --states 3

# Train model
python3 -m quant_cli train TSLA --save output/TSLA_model.pkl

# Run backtest
python3 -m quant_cli backtest TSLA --model output/TSLA_model.pkl

# Generate HTML report
python3 -m quant_cli report TSLA --period 1y
```

### Data
```bash
# Data directory
ls ~/.stock_ai/          # SQLite DB + output files

# Clean cache
rm ~/.stock_ai/data.db

# Clean Python output
rm -rf output/
```

---

## Files to Create

1. `scripts/setup.sh`
2. `scripts/run.sh`
3. `scripts/README.md`
