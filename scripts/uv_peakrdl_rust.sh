#!/usr/bin/env sh
uv run --no-project --with peakrdl-rust[cli]==0.5.1 peakrdl rust "$@"
