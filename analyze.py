#!/usr/bin/env python3
"""
Backward-compatible entry point — delegates to quant_cli.
Usage: python analyze.py [symbol...]  (defaults to 2330.TW)
"""

import sys
from quant_cli.__main__ import main

if __name__ == "__main__":
    # Inject 'analyze' subcommand + default symbol if none given
    args = sys.argv[1:] if len(sys.argv) > 1 else ["2330.TW"]
    sys.argv = [sys.argv[0], "analyze", "--output", "analysis_result.json"] + args
    main()
