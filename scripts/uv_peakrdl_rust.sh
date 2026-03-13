#!/usr/bin/env sh
uv run --no-project --with peakrdl-rust[cli]==0.6.1 peakrdl rust "$@"
