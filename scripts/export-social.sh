#!/bin/bash
# Export demo recording to social media formats using FFmpeg
set -euo pipefail

INPUT=${1:-"recordings/kafka-backup-demo.gif"}
OUTPUT_DIR=${2:-"recordings"}
BG_COLOR="#0F172A"

mkdir -p "$OUTPUT_DIR"

if [ ! -f "$INPUT" ]; then
    echo "Error: Input file not found: $INPUT"
    echo "Run record-demo.sh first to create a recording."
    exit 1
fi

echo "Exporting social media formats from: $INPUT"

# LinkedIn landscape (1920x1080) — best for feed visibility
echo "  LinkedIn landscape (1920x1080)..."
ffmpeg -y -i "$INPUT" \
    -vf "scale=1920:1080:force_original_aspect_ratio=decrease:flags=lanczos,pad=1920:1080:(ow-iw)/2:(oh-ih)/2:color=${BG_COLOR}" \
    -c:v libx264 -crf 18 -preset slow -pix_fmt yuv420p \
    "${OUTPUT_DIR}/linkedin.mp4" 2>/dev/null

# LinkedIn square (1080x1080) — alternative for square posts
echo "  LinkedIn square (1080x1080)..."
ffmpeg -y -i "$INPUT" \
    -vf "scale=1080:1080:force_original_aspect_ratio=decrease:flags=lanczos,pad=1080:1080:(ow-iw)/2:(oh-ih)/2:color=${BG_COLOR}" \
    -c:v libx264 -crf 18 -preset slow -pix_fmt yuv420p \
    "${OUTPUT_DIR}/linkedin-square.mp4" 2>/dev/null

# Twitter/X (1200x675 GIF)
echo "  Twitter/X (1200x675)..."
ffmpeg -y -i "$INPUT" \
    -vf "scale=1200:675:force_original_aspect_ratio=decrease:flags=lanczos,pad=1200:675:(ow-iw)/2:(oh-ih)/2:color=${BG_COLOR},fps=15" \
    "${OUTPUT_DIR}/twitter.gif" 2>/dev/null

# YouTube Shorts (1080x1920 vertical)
echo "  YouTube Shorts (1080x1920)..."
ffmpeg -y -i "$INPUT" \
    -vf "scale=1080:1920:force_original_aspect_ratio=decrease:flags=lanczos,pad=1080:1920:(ow-iw)/2:(oh-ih)/2:color=${BG_COLOR}" \
    -c:v libx264 -crf 18 -preset slow -pix_fmt yuv420p \
    "${OUTPUT_DIR}/shorts.mp4" 2>/dev/null

# GitHub README (800x500 optimised GIF)
echo "  GitHub README (800x500)..."
ffmpeg -y -i "$INPUT" \
    -vf "scale=800:500:flags=lanczos,fps=12" \
    -gifflags +transdiff \
    "${OUTPUT_DIR}/github.gif" 2>/dev/null

# Optimise GitHub GIF if gifsicle is available
if command -v gifsicle &> /dev/null; then
    gifsicle --optimize=3 --lossy=80 \
        "${OUTPUT_DIR}/github.gif" \
        -o "${OUTPUT_DIR}/github-optimised.gif"
    SIZE=$(du -h "${OUTPUT_DIR}/github-optimised.gif" | cut -f1)
    echo "  GitHub optimised: ${SIZE}"
fi

echo ""
echo "Exports complete:"
ls -lh "${OUTPUT_DIR}/linkedin.mp4" "${OUTPUT_DIR}/linkedin-square.mp4" "${OUTPUT_DIR}/twitter.gif" "${OUTPUT_DIR}/shorts.mp4" "${OUTPUT_DIR}/github.gif" 2>/dev/null
