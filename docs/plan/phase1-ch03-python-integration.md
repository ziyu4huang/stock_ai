# Ch 3 — Python HMM Integration

> Tighten Python subprocess calls, add model management endpoints.

---

## 3.1 Standardize Subprocess Output

**Current:** `GET /api/backtest/:symbol` calls `python3 -m quant_cli analyze`, reads JSON file from `output/`.

**Improvements:**
- Add timeout (30s default) to subprocess call
- Capture both stdout and stderr
- Return structured error JSON on failure: `{"error": "...", "stderr": "..."}`
- Ensure `output/` directory exists before Python writes to it

---

## 3.2 Backtest Parameters via Query String

**Current:** Hardcoded `--period 1y`.

**New:**
```
GET /api/backtest/:symbol?period=1y&states=3&source=yfinance
```

Maps to:
```bash
python3 -m quant_cli analyze {symbol} --period {period} --states {states} --source {source}
```

Default values: period=1y, states=3, source=yfinance

---

## 3.3 Model Management Endpoints

### Train & Save
```
POST /api/model/train/:symbol?states=3&source=yfinance
```
Runs: `python3 -m quant_cli train {symbol} --save model.pkl --states {states}`
Returns: `{"symbol": "TSLA", "status": "trained", "model_path": "..."}`

### List Models
```
GET /api/model/list
```
Scans `output/*.pkl`, returns: `[{"symbol": "TSLA", "trained_at": "2026-04-01", "states": 3}]`

### Delete Model
```
DELETE /api/model/:symbol
```
Removes the corresponding `.pkl` file.

---

## 3.4 Report Endpoint (already works)

```
GET /api/report/:symbol
```
- Calls `python3 -m quant_cli report {symbol} --period 1y`
- Returns HTML from `output/report_{symbol}.html`
- No changes needed, just ensure output dir creation

---

## Python CLI Commands (reference)

```bash
# Full analysis (JSON output)
python3 -m quant_cli analyze SYMBOL [--source yfinance|av] [--states N] [--period 1y]

# Train & save model
python3 -m quant_cli train SYMBOL --save model.pkl

# Run backtest with saved model
python3 -m quant_cli backtest SYMBOL --model model.pkl

# Generate HTML report
python3 -m quant_cli report SYMBOL [--period 1y]
```

---

## Files to Modify

1. `src/handlers.rs` — extend get_backtest with query params, add model endpoints
2. `src/types.rs` — add BacktestQuery, ModelQuery structs
