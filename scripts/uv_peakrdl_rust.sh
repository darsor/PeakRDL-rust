#!/usr/bin/env sh
uv run --no-project --with peakrdl-rust[cli]==0.6.2 peakrdl rust "$@"
