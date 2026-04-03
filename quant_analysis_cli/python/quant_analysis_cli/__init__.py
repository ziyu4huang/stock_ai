"""quant_analysis_cli — HMM regime analysis, backtesting, and reporting.

Usage:
    PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli <command> [args]

Commands:
    analyze SYMBOL   Full pipeline: read data -> features -> HMM -> indicators -> backtest
    train SYMBOL     Train HMM and persist model to SQLite
    backtest SYMBOL  Load saved model, run backtest
    report SYMBOL    Full pipeline + HTML report
    signals SYMBOL   Compute trading signals
"""
