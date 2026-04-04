// Signal radar panel — with confidence bar, whale volume, spoof alerts

import type { TabSnapshot } from './types';
import { el } from './dom';

export function render(v: TabSnapshot): void {
  const radarEl = el('signal-radar');
  el('tick-title').textContent = `TICK FEED  ${v.symbol}`;

  const scoreColor = v.composite_score > 0 ? 'var(--lgreen)' : v.composite_score < 0 ? 'var(--lred)' : 'var(--dim)';
  const scorePct = ((v.composite_score + 100) / 200 * 100).toFixed(1);
  const stateColor = v.state.includes('L-') ? 'var(--lgreen)' : v.state.includes('S-') ? 'var(--lred)' : v.state === 'NEUTRAL' ? 'var(--dim)' : '#555';
  const actionColor = v.action.includes('Bull') ? 'var(--lgreen)' : v.action.includes('Bear') ? 'var(--lred)' : 'var(--dim)';
  const chgColor = v.change_pct > 0 ? 'var(--lgreen)' : v.change_pct < 0 ? 'var(--lred)' : 'var(--dim)';
  const hmmColor = v.hmm_state === 'H-IGNITE' ? 'var(--lgreen)' : v.hmm_state === 'H-ACCUM' ? 'var(--cyan)' : v.hmm_state === 'H-DIST' ? 'var(--lred)' : 'var(--dim)';
  const maxWhale = Math.max(v.whale_buys, v.whale_sells, v.long_ignitions, v.short_ignitions, 1);

  // Confidence color
  const conf = v.signal_confidence;
  const confColor = conf >= 70 ? 'var(--lgreen)' : conf >= 40 ? 'var(--yellow)' : 'var(--lred)';

  // Spoof/cluster warning flags
  const hasSpoof = v.suspicious_adds > 0 || v.suspicious_removes > 0;
  const hasCluster = v.cluster_count > 0;

  // Velocity indicator
  const vel = v.trade_velocity;
  const velClass = vel > 60 ? 'spike' : vel > 30 ? 'elevated' : '';
  const velColor = vel > 60 ? 'var(--red)' : vel > 30 ? 'var(--yellow)' : 'var(--dim)';

  radarEl.innerHTML = `
    <div class="score-bar-wrap">
      <span class="score-label" style="color:${scoreColor}">${v.composite_score >= 0 ? '+' : ''}${v.composite_score}</span>
      <div class="score-bar">
        <div class="mid"></div>
        <div class="fill" style="background:${scoreColor};left:${v.composite_score >= 0 ? '50%' : scorePct + '%'};right:${v.composite_score >= 0 ? (100 - parseFloat(scorePct)) + '%' : '50%'}"></div>
      </div>
    </div>

    <!-- Signal confidence bar -->
    <div class="conf-bar-wrap">
      <div class="conf-label"><span style="color:var(--dim)">Confidence</span><span style="color:${confColor}">${conf.toFixed(0)}%</span></div>
      <div class="conf-bar"><div class="conf-fill" style="background:${confColor};width:${conf.toFixed(0)}%"></div></div>
    </div>

    <div class="market-state" style="color:${stateColor}">${v.state_icon} ${v.state}</div>
    <div class="hmm-state">HMM <span style="color:${hmmColor}">${v.hmm_state_icon} ${v.hmm_state}</span> <span class="${v.hmm_agrees ? 'hmm-agree' : 'hmm-div'}">${v.hmm_agrees ? '\u2713 AGREE' : '\u2717 DIV'}</span></div>
    <div class="action-box" style="border-color:${actionColor}30">
      <div class="action-label" style="color:${actionColor}">${v.action_label_zh}</div>
      <div class="sl-label">${v.stop_loss !== null ? 'S/L: ' + v.stop_loss.toFixed(2) : 'S/L: \u2014'}</div>
    </div>

    <!-- Spoof / cluster warnings -->
    ${hasSpoof ? `<div class="spoof-alert">\u26A0 SPOOF: ${v.suspicious_removes} vanish + ${v.suspicious_adds} surge</div>` : ''}
    ${hasCluster ? `<div class="spoof-alert">\u26A0 CLUSTER: ${v.cluster_count}x whale burst (wash?)</div>` : ''}

    <!-- Whale volume breakdown (volume + count) -->
    <div style="margin-top:4px">
      <div class="counter-row"><span class="counter-label">WhaleBuy \u25B2</span><div class="counter-bar" style="background:rgba(105,240,174,0.3);width:${(v.whale_buys / maxWhale * 100).toFixed(0)}%"></div><span class="counter-val" style="color:var(--lgreen)">${v.whale_buy_volume_m.toFixed(1)}M (${v.whale_buys})</span></div>
      <div class="counter-row"><span class="counter-label">WhaleSell\u25BC</span><div class="counter-bar" style="background:rgba(255,82,82,0.3);width:${(v.whale_sells / maxWhale * 100).toFixed(0)}%"></div><span class="counter-val" style="color:var(--lred)">${v.whale_sell_volume_m.toFixed(1)}M (${v.whale_sells})</span></div>
      <div class="counter-row"><span class="counter-label">IgnLong  \u25B2</span><div class="counter-bar" style="background:rgba(76,175,80,0.3);width:${(v.long_ignitions / maxWhale * 100).toFixed(0)}%"></div><span class="counter-val" style="color:var(--green)">${v.long_ignitions}</span></div>
      <div class="counter-row"><span class="counter-label">IgnShort \u25BC</span><div class="counter-bar" style="background:rgba(239,83,80,0.3);width:${(v.short_ignitions / maxWhale * 100).toFixed(0)}%"></div><span class="counter-val" style="color:var(--red)">${v.short_ignitions}</span></div>
    </div>

    <div class="price-row">
      <span style="color:#fff;font-weight:bold;font-size:16px">${v.last_price.toFixed(2)}</span>
      <span style="color:${chgColor}">${v.change_pct >= 0 ? '+' : ''}${v.change_pct.toFixed(2)}%</span>
      <span style="color:var(--dim)">Ticks:${v.total_ticks}</span>
    </div>

    <!-- Velocity indicator -->
    <div class="velocity-ind ${velClass}" style="color:${velColor}">Velocity ${vel.toFixed(1)} t/min ${vel > 60 ? '\u26A1' : vel > 30 ? '\u25B2' : ''}</div>

    <!-- Absorption score -->
    ${v.absorption_score > 20 ? `<div style="font-size:10px;color:var(--cyan);padding:1px 0">Absorption ${v.absorption_score.toFixed(0)}%</div>` : ''}

    ${v.last_sound ? `<div class="sound-row">${v.last_sound.label} ${v.last_sound.timestamp}</div>` : ''}
  `;
}
