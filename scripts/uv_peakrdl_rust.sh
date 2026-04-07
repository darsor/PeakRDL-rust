#!/usr/bin/env sh
uv run --no-project --with peakrdl-rust[cli]==0.7.2 peakrdl rust "$@"
