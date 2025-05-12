#!/usr/bin/env bash
set -euo pipefail

# Usage: ./create_gif.sh [pattern]
# e.g. ./create_gif.sh '*.png' or './create_gif.sh clifford_*.png'

PATTERN="${1:-*.png}"
FRAMES_DIR="./output"
OUTPUT="animation.gif"

ORIG_DIR="$(pwd)"
cleanup() { cd "$ORIG_DIR"; }
trap cleanup EXIT

cd "$FRAMES_DIR"
magick convert -delay 4 -loop 0 $PATTERN "$OUTPUT"

# Preview (On Linux change to 'xdg-open')
open "$OUTPUT"
