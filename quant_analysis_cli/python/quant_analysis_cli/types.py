"""Shared types for quant_analysis_cli."""

from dataclasses import dataclass, field
from typing import List, Optional


@dataclass
class StateInfo:
    """HMM hidden state descriptor."""
    state: int
    label: str
    emoji: str
    count: int
    pct: float
    avg_daily_ret_pct: float
    avg_range_pct: float


@dataclass
class TradeRecord:
    """Single trade in a backtest."""
    date: str
    entry: float
    exit: float
    gross_pct: float
    cost_pct: float
    net_pct: float


@dataclass
class BacktestResult:
    """Day-trade backtest summary."""
    total_trades: int
    win_rate: float
    avg_gross_ret_pct: float = 0.0
    avg_net_ret_pct: float = 0.0
    total_net_pct: float = 0.0
    cost_per_trade_pct: float = 0.0
    last_5_trades: List[TradeRecord] = field(default_factory=list)


@dataclass
class AnalysisResult:
    """Complete analysis output for one symbol."""
    symbol: str
    name: str
    close: float
    current_state: int
    current_label: str
    rsi: float
    macd: float
    macd_signal: float
    macd_hist: float
    bb_upper: float
    bb_mid: float
    bb_lower: float
    states: List[dict]
    backtest: dict


@dataclass
class HMMModelMeta:
    """Metadata for a trained HMM model stored in SQLite."""
    id: Optional[int]
    symbol: str
    n_states: int
    source: str
    period: str
    bars_used: int
    bic: Optional[float]
    score: Optional[float]
    created_at: str
