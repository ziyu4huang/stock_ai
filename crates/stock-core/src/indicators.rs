use crate::types::*;

// ── single-value indicators (existing, unchanged) ─────────────────────────

pub fn calc_rsi(closes: &[f64], period: usize) -> f64 {
    if closes.len() < period + 1 {
        return 50.0;
    }
    let mut g = 0.0;
    let mut l = 0.0;
    let n = closes.len();
    for i in (n - period)..n {
        let d = closes[i] - closes[i - 1];
        if d > 0.0 {
            g += d;
        } else {
            l += d.abs();
        }
    }
    let ag = g / period as f64;
    let al = l / period as f64;
    if al == 0.0 {
        return 100.0;
    }
    100.0 - (100.0 / (1.0 + ag / al))
}

pub fn calc_ema(data: &[f64], p: usize) -> Vec<f64> {
    if data.is_empty() {
        return vec![];
    }
    let k = 2.0 / (p as f64 + 1.0);
    let mut e = vec![0.0; data.len()];
    e[0] = data[0];
    for i in 1..data.len() {
        e[i] = data[i] * k + e[i - 1] * (1.0 - k);
    }
    e
}

pub fn calc_macd(closes: &[f64]) -> (f64, f64, f64) {
    let e12 = calc_ema(closes, 12);
    let e26 = calc_ema(closes, 26);
    if e12.len() < 26 {
        return (0.0, 0.0, 0.0);
    }
    let ml: Vec<f64> = e12.iter().zip(&e26).map(|(a, b)| a - b).collect();
    let sl = calc_ema(&ml, 9);
    let n = ml.len();
    (ml[n - 1], sl[n - 1], ml[n - 1] - sl[n - 1])
}

pub fn calc_bb(closes: &[f64], p: usize, m: f64) -> (f64, f64, f64) {
    if closes.len() < p {
        let l = closes.last().copied().unwrap_or(0.0);
        return (l, l, l);
    }
    let s = &closes[closes.len() - p..];
    let mean = s.iter().sum::<f64>() / p as f64;
    let var = s.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / p as f64;
    let sd = var.sqrt();
    (mean + m * sd, mean, mean - m * sd)
}

// ── series indicators ────────────────────────────────────────────────────

pub fn calc_rsi_series(closes: &[f64], period: usize) -> Vec<f64> {
    let n = closes.len();
    let mut rsi = vec![0.0; n];
    if n <= period {
        return rsi;
    }
    let mut ag = 0.0;
    let mut al = 0.0;
    for i in 1..=period {
        let d = closes[i] - closes[i - 1];
        if d > 0.0 { ag += d; } else { al += d.abs(); }
    }
    ag /= period as f64;
    al /= period as f64;
    rsi[period] = if al == 0.0 { 100.0 } else { 100.0 - 100.0 / (1.0 + ag / al) };
    for i in (period + 1)..n {
        let d = closes[i] - closes[i - 1];
        ag = (ag * (period - 1) as f64 + d.max(0.0)) / period as f64;
        al = (al * (period - 1) as f64 + (-d).max(0.0)) / period as f64;
        rsi[i] = if al == 0.0 { 100.0 } else { 100.0 - 100.0 / (1.0 + ag / al) };
    }
    rsi
}

pub fn calc_macd_series(closes: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let e12 = calc_ema(closes, 12);
    let e26 = calc_ema(closes, 26);
    let macd_line: Vec<f64> = e12.iter().zip(&e26).map(|(a, b)| a - b).collect();
    let signal_line = calc_ema(&macd_line, 9);
    let hist: Vec<f64> = macd_line.iter().zip(&signal_line).map(|(a, b)| a - b).collect();
    (macd_line, signal_line, hist)
}

pub fn calc_bb_series(closes: &[f64], period: usize, mult: f64) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let n = closes.len();
    let mut upper = vec![0.0; n];
    let mut mid = vec![0.0; n];
    let mut lower = vec![0.0; n];
    for i in (period - 1)..n {
        let s = &closes[i + 1 - period..=i];
        let mean = s.iter().sum::<f64>() / period as f64;
        let var = s.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / period as f64;
        let sd = var.sqrt();
        upper[i] = mean + mult * sd;
        mid[i] = mean;
        lower[i] = mean - mult * sd;
    }
    // fill warmup with last close
    for i in 0..(period - 1) {
        upper[i] = closes[i];
        mid[i] = closes[i];
        lower[i] = closes[i];
    }
    (upper, mid, lower)
}

// ── signal detectors ─────────────────────────────────────────────────────

fn sig(kind: SignalKind, dir: SignalDirection, strength: SignalStrength, score: i32, reason: String) -> DaySignal {
    DaySignal { kind, direction: dir, strength, score, reason }
}

pub fn detect_rsi(rsi: &[f64], i: usize) -> Option<DaySignal> {
    if i < 1 { return None; }
    let prev = rsi[i - 1];
    let cur = rsi[i];
    // Oversold reversal: was below 30, now crossed back above
    if prev < 30.0 && cur >= 30.0 {
        let s = if prev < 25.0 { (SignalStrength::Strong, 30) } else { (SignalStrength::Medium, 20) };
        return Some(sig(SignalKind::RsiReversal, SignalDirection::Buy, s.0, s.1,
            format!("RSI(14) crossed up from {:.1} to {:.1}", prev, cur)));
    }
    // Overbought reversal: was above 70, now crossed back below
    if prev > 70.0 && cur <= 70.0 {
        let s = if prev > 75.0 { (SignalStrength::Strong, -30) } else { (SignalStrength::Medium, -20) };
        return Some(sig(SignalKind::RsiReversal, SignalDirection::Sell, s.0, s.1,
            format!("RSI(14) crossed down from {:.1} to {:.1}", prev, cur)));
    }
    None
}

pub fn detect_macd(ml: &[f64], sl: &[f64], hist: &[f64], i: usize) -> Vec<DaySignal> {
    let mut out = Vec::new();
    if i < 1 { return out; }
    // Histogram zero cross
    if hist[i - 1] < 0.0 && hist[i] >= 0.0 {
        out.push(sig(SignalKind::MacdHistCross, SignalDirection::Buy, SignalStrength::Medium, 20,
            format!("MACD histogram crossed positive ({:.4}→{:.4})", hist[i-1], hist[i])));
    }
    if hist[i - 1] >= 0.0 && hist[i] < 0.0 {
        out.push(sig(SignalKind::MacdHistCross, SignalDirection::Sell, SignalStrength::Medium, -20,
            format!("MACD histogram crossed negative ({:.4}→{:.4})", hist[i-1], hist[i])));
    }
    // Signal line crossover
    if ml[i - 1] < sl[i - 1] && ml[i] >= sl[i] {
        out.push(sig(SignalKind::MacdSignalCross, SignalDirection::Buy, SignalStrength::Medium, 15,
            format!("MACD crossed above signal ({:.4} vs {:.4})", ml[i], sl[i])));
    }
    if ml[i - 1] >= sl[i - 1] && ml[i] < sl[i] {
        out.push(sig(SignalKind::MacdSignalCross, SignalDirection::Sell, SignalStrength::Medium, -15,
            format!("MACD crossed below signal ({:.4} vs {:.4})", ml[i], sl[i])));
    }
    out
}

pub fn detect_bb(closes: &[f64], upper: &[f64], lower: &[f64], i: usize) -> Option<DaySignal> {
    if i < 20 { return None; }
    let c = closes[i];
    let u = upper[i];
    let l = lower[i];
    let width = u - l;
    // Check if width is in bottom 20% of last 60 bars
    let lookback = 60.min(i);
    let start = i + 1 - lookback;
    let mut widths: Vec<f64> = (start..=i).map(|j| upper[j] - lower[j]).collect();
    widths.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pct20 = widths[widths.len() / 5];
    let is_squeeze = width <= pct20;

    if c > u && is_squeeze {
        return Some(sig(SignalKind::BbBreakout, SignalDirection::Buy, SignalStrength::Strong, 30,
            format!("BB squeeze breakout above upper band ({:.2} > {:.2})", c, u)));
    }
    if c < l && is_squeeze {
        return Some(sig(SignalKind::BbBreakout, SignalDirection::Sell, SignalStrength::Strong, -30,
            format!("BB squeeze breakout below lower band ({:.2} < {:.2})", c, l)));
    }
    // Touch upper/lower without squeeze
    if i >= 1 && closes[i - 1] <= u && c > u {
        return Some(sig(SignalKind::BbSqueeze, SignalDirection::Buy, SignalStrength::Medium, 20,
            format!("Price touched upper BB ({:.2})", c)));
    }
    if i >= 1 && closes[i - 1] >= l && c < l {
        return Some(sig(SignalKind::BbSqueeze, SignalDirection::Sell, SignalStrength::Medium, -20,
            format!("Price touched lower BB ({:.2})", c)));
    }
    None
}

pub fn detect_volume(bars: &[Bar], i: usize) -> Option<DaySignal> {
    if i < 20 { return None; }
    let avg: f64 = bars[i - 20..i].iter().map(|b| b.volume as f64).sum::<f64>() / 20.0;
    if avg <= 0.0 { return None; }
    let vol = bars[i].volume as f64;
    let ratio = vol / avg;
    if ratio < 2.0 { return None; }
    let bullish = bars[i].close >= bars[i].open;
    let dir = if bullish { SignalDirection::Buy } else { SignalDirection::Sell };
    let (strength, score) = if ratio >= 5.0 {
        (SignalStrength::Strong, if bullish { 30 } else { -30 })
    } else if ratio >= 3.0 {
        (SignalStrength::Medium, if bullish { 20 } else { -20 })
    } else {
        (SignalStrength::Weak, if bullish { 10 } else { -10 })
    };
    Some(sig(SignalKind::VolumeSurge, dir, strength, score,
        format!("Volume {:.1}x average ({:.0} vs avg {:.0})", ratio, vol, avg)))
}

pub fn detect_engulfing(bars: &[Bar], i: usize) -> Option<DaySignal> {
    if i < 1 { return None; }
    let prev = &bars[i - 1];
    let cur = &bars[i];
    let prev_body = (prev.close - prev.open).abs();
    let cur_body = (cur.close - cur.open).abs();
    if prev_body == 0.0 || cur_body <= prev_body { return None; }

    let prev_bull = prev.close >= prev.open;
    let cur_bull = cur.close >= cur.open;
    // Bullish engulfing: prev bearish, cur bullish, cur body engulfs prev
    if !prev_bull && cur_bull && cur.open <= prev.close && cur.close >= prev.open {
        return Some(sig(SignalKind::EngulfingBull, SignalDirection::Buy, SignalStrength::Medium, 20,
            "Bullish engulfing pattern".into()));
    }
    // Bearish engulfing
    if prev_bull && !cur_bull && cur.open >= prev.close && cur.close <= prev.open {
        return Some(sig(SignalKind::EngulfingBear, SignalDirection::Sell, SignalStrength::Medium, -20,
            "Bearish engulfing pattern".into()));
    }
    None
}

pub fn detect_hammer(bars: &[Bar], i: usize) -> Option<DaySignal> {
    let b = &bars[i];
    let body = (b.close - b.open).abs();
    let range = b.high - b.low;
    if range <= 0.0 { return None; }
    let upper_shadow = b.high - b.close.max(b.open);
    let lower_shadow = b.open.min(b.close) - b.low;
    // Hammer: small body at top, long lower shadow (>=2x body)
    if body > 0.0 && lower_shadow >= 2.0 * body && upper_shadow <= body * 0.3 && b.close >= b.open {
        return Some(sig(SignalKind::Hammer, SignalDirection::Buy, SignalStrength::Medium, 20,
            "Hammer candlestick pattern".into()));
    }
    None
}

pub fn detect_shooting_star(bars: &[Bar], i: usize) -> Option<DaySignal> {
    let b = &bars[i];
    let body = (b.close - b.open).abs();
    let range = b.high - b.low;
    if range <= 0.0 { return None; }
    let upper_shadow = b.high - b.close.max(b.open);
    let lower_shadow = b.open.min(b.close) - b.low;
    // Shooting star: small body at bottom, long upper shadow
    if body > 0.0 && upper_shadow >= 2.0 * body && lower_shadow <= body * 0.3 && b.close < b.open {
        return Some(sig(SignalKind::ShootingStar, SignalDirection::Sell, SignalStrength::Medium, -20,
            "Shooting star candlestick pattern".into()));
    }
    None
}

pub fn detect_morning_star(bars: &[Bar], i: usize) -> Option<DaySignal> {
    if i < 2 { return None; }
    let b0 = &bars[i - 2]; // large bearish
    let b1 = &bars[i - 1]; // small body (indecision)
    let b2 = &bars[i];     // large bullish
    let body0 = (b0.close - b0.open).abs();
    let body1 = (b1.close - b1.open).abs();
    let body2 = (b2.close - b2.open).abs();
    let avg_body = (body0 + body2) / 2.0;
    if avg_body <= 0.0 { return None; }
    if b0.close >= b0.open { return None; } // b0 must be bearish
    if b2.close <= b2.open { return None; } // b2 must be bullish
    if body1 > avg_body * 0.33 { return None; } // b1 must be small
    if b2.close < (b0.open + b0.close) / 2.0 { return None; } // b2 must close into b0 body
    Some(sig(SignalKind::MorningStar, SignalDirection::Buy, SignalStrength::Strong, 30,
        "Morning star (3-candle) bullish reversal".into()))
}

pub fn detect_evening_star(bars: &[Bar], i: usize) -> Option<DaySignal> {
    if i < 2 { return None; }
    let b0 = &bars[i - 2];
    let b1 = &bars[i - 1];
    let b2 = &bars[i];
    let body0 = (b0.close - b0.open).abs();
    let body1 = (b1.close - b1.open).abs();
    let body2 = (b2.close - b2.open).abs();
    let avg_body = (body0 + body2) / 2.0;
    if avg_body <= 0.0 { return None; }
    if b0.close <= b0.open { return None; } // b0 must be bullish
    if b2.close >= b2.open { return None; } // b2 must be bearish
    if body1 > avg_body * 0.33 { return None; }
    if b2.close > (b0.open + b0.close) / 2.0 { return None; }
    Some(sig(SignalKind::EveningStar, SignalDirection::Sell, SignalStrength::Strong, -30,
        "Evening star (3-candle) bearish reversal".into()))
}

pub fn detect_ema_cross(ema5: &[f64], ema20: &[f64], i: usize) -> Option<DaySignal> {
    if i < 1 { return None; }
    if ema5[i - 1] < ema20[i - 1] && ema5[i] >= ema20[i] {
        return Some(sig(SignalKind::EmaCross, SignalDirection::Buy, SignalStrength::Medium, 20,
            "EMA(5) crossed above EMA(20)".into()));
    }
    if ema5[i - 1] >= ema20[i - 1] && ema5[i] < ema20[i] {
        return Some(sig(SignalKind::EmaCross, SignalDirection::Sell, SignalStrength::Medium, -20,
            "EMA(5) crossed below EMA(20)".into()));
    }
    None
}

pub fn detect_gap(bars: &[Bar], i: usize) -> Option<DaySignal> {
    if i < 1 { return None; }
    let prev_close = bars[i - 1].close;
    if prev_close <= 0.0 { return None; }
    let gap_pct = (bars[i].open - prev_close) / prev_close * 100.0;
    if gap_pct > 1.0 {
        return Some(sig(SignalKind::GapUp, SignalDirection::Buy, SignalStrength::Weak, 10,
            format!("Gap up {:.1}% from previous close {:.2}", gap_pct, prev_close)));
    }
    if gap_pct < -1.0 {
        return Some(sig(SignalKind::GapDown, SignalDirection::Sell, SignalStrength::Weak, -10,
            format!("Gap down {:.1}% from previous close {:.2}", gap_pct, prev_close)));
    }
    None
}

// ── orchestrator ─────────────────────────────────────────────────────────

fn bar_date(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|t| t.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn clamp_score(score: i32) -> i32 {
    score.max(-100).min(100)
}

pub fn analyze_daytrade(bars: &[Bar]) -> DayTradeAnalysis {
    let n = bars.len();
    let symbol = if n > 0 { String::new() } else { String::new() };
    if n < 30 {
        return DayTradeAnalysis {
            symbol,
            bars: bars.to_vec(),
            signals: vec![],
            latest_score: 0,
            latest_direction: SignalDirection::Neutral,
        };
    }

    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let rsi = calc_rsi_series(&closes, 14);
    let (ml, sl, hist) = calc_macd_series(&closes);
    let (bbu, _bbm, bbl) = calc_bb_series(&closes, 20, 2.0);
    let ema5 = calc_ema(&closes, 5);
    let ema20 = calc_ema(&closes, 20);

    let warmup = 26; // MACD needs 26 bars
    let mut all_signals: Vec<BarSignals> = Vec::new();

    for i in warmup..n {
        let mut bar_sigs: Vec<DaySignal> = Vec::new();

        if let Some(s) = detect_rsi(&rsi, i) { bar_sigs.push(s); }
        bar_sigs.extend(detect_macd(&ml, &sl, &hist, i));
        if let Some(s) = detect_bb(&closes, &bbu, &bbl, i) { bar_sigs.push(s); }
        if let Some(s) = detect_volume(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_engulfing(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_hammer(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_shooting_star(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_morning_star(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_evening_star(bars, i) { bar_sigs.push(s); }
        if let Some(s) = detect_ema_cross(&ema5, &ema20, i) { bar_sigs.push(s); }
        if let Some(s) = detect_gap(bars, i) { bar_sigs.push(s); }

        if !bar_sigs.is_empty() {
            let composite = clamp_score(bar_sigs.iter().map(|s| s.score).sum());
            all_signals.push(BarSignals {
                idx: i,
                date: bar_date(bars[i].time),
                signals: bar_sigs,
                score: composite,
            });
        }
    }

    let (latest_score, latest_direction) = all_signals.last()
        .map(|bs| {
            let dir = if bs.score > 0 { SignalDirection::Buy }
                else if bs.score < 0 { SignalDirection::Sell }
                else { SignalDirection::Neutral };
            (bs.score, dir)
        })
        .unwrap_or((0, SignalDirection::Neutral));

    DayTradeAnalysis {
        symbol: String::new(), // filled by caller
        bars: bars.to_vec(),
        signals: all_signals,
        latest_score,
        latest_direction,
    }
}
