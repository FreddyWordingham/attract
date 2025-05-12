#!/usr/bin/env bash
set -euo pipefail

# Usage: ./create_movie.sh [pattern]
# e.g. ./create_movie.sh '*.png' or './create_movie.sh frame_*.png'

PATTERN="${1:-*.png}"
FRAMES_DIR="./output"
OUTPUT="movie.mpg"

ORIG_DIR="$(pwd)"
cleanup() { cd "$ORIG_DIR"; }
trap cleanup EXIT

cd "$FRAMES_DIR"
magick convert -delay 4 -loop 1 $PATTERN "$OUTPUT"

# Preview (On Linux change to 'xdg-open')
open "$OUTPUT"
