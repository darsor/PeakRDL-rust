#!/usr/bin/env bash
# Build a self-contained PeakRDL-rust binary using PyInstaller.
#
# Usage:
#   ./scripts/build_binary.sh                  # output to dist/
#   ./scripts/build_binary.sh --output-dir /tmp/out
#
# The resulting binary will be at <output-dir>/peakrdl.
# A SHA-256 checksum file is written alongside it.

set -euo pipefail
set -x # debugging

OUTPUT_DIR="dist"
BINARY_NAME="peakrdl-rust"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --output-dir)
      OUTPUT_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

mkdir -p "$OUTPUT_DIR"

echo "Building ${BINARY_NAME} binary..."
echo "  Output dir : ${OUTPUT_DIR}"

uv run --with pyinstaller pyinstaller \
  --onefile \
  --name ${BINARY_NAME} \
  --distpath "${OUTPUT_DIR}" \
  --workpath "${OUTPUT_DIR}/.build" \
  --specpath "${OUTPUT_DIR}/.build" \
  --collect-all peakrdl_rust \
  --collect-all peakrdl \
  --collect-all systemrdl \
  --copy-metadata peakrdl_rust \
  scripts/entrypoint.py

BINARY="${OUTPUT_DIR}/${BINARY_NAME}"

echo ""
echo "Binary built: ${BINARY}"

# Write checksum.
if command -v sha256sum &>/dev/null; then
  sha256sum "${BINARY}" | tee "${BINARY}.sha256"
elif command -v shasum &>/dev/null; then
  # macOS
  shasum -a 256 "${BINARY}" | tee "${BINARY}.sha256"
else
  echo "Warning: no sha256 tool found, skipping checksum" >&2
fi
