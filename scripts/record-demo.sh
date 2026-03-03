#!/bin/bash
# Record a demo scenario using VHS (https://github.com/charmbracelet/vhs)
set -euo pipefail

SCENARIO=${1:-"scenarios/full-backup-restore.yaml"}
OUTPUT_DIR=${2:-"recordings"}
BINARY="./target/release/kafka-backup-monitor"

mkdir -p "$OUTPUT_DIR"

# Build release binary
echo "Building release binary..."
cargo build --release

# Calculate sleep duration from scenario (sum of all scene durations + buffer)
# Default to 65s for full demo
SLEEP_DURATION=${3:-"65s"}

# Create VHS tape file dynamically
TAPE_FILE=$(mktemp /tmp/kafka-backup-monitor-XXXXXX.tape)
cat > "$TAPE_FILE" << EOF
Output ${OUTPUT_DIR}/kafka-backup-demo.gif
Set FontSize 32
Set FontFamily "JetBrains Mono"
Set Width 2400
Set Height 1350
Set Theme "Catppuccin Mocha"
Set Padding 40
Set Framerate 24
Set TypingSpeed 0

Type "${BINARY} demo --scenario ${SCENARIO} --auto-start"
Enter
Sleep ${SLEEP_DURATION}
EOF

echo "Recording demo..."
vhs "$TAPE_FILE"

# Optimise GIF if gifsicle is available
if command -v gifsicle &> /dev/null; then
    echo "Optimising GIF..."
    gifsicle --optimize=3 --lossy=80 \
        "${OUTPUT_DIR}/kafka-backup-demo.gif" \
        -o "${OUTPUT_DIR}/kafka-backup-demo-optimised.gif"
    echo "Optimised: ${OUTPUT_DIR}/kafka-backup-demo-optimised.gif"
fi

rm -f "$TAPE_FILE"
echo "Recording saved to ${OUTPUT_DIR}/kafka-backup-demo.gif"
