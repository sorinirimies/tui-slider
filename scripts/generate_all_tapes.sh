#!/usr/bin/env bash

# Generate all VHS demo tapes
# This script automatically discovers all .tape files and generates their GIFs

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}🎬 Generating all VHS demo tapes...${NC}\n"

# Check if vhs is installed
if ! command -v vhs &> /dev/null; then
    echo -e "${RED}❌ Error: vhs is not installed${NC}"
    echo "Install it with: brew install vhs"
    exit 1
fi

# Find all .tape files in examples/vhs/
TAPE_DIR="examples/vhs"
OUTPUT_DIR="examples/vhs/output"

if [ ! -d "$TAPE_DIR" ]; then
    echo -e "${RED}❌ Error: $TAPE_DIR directory not found${NC}"
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Count total tapes
TOTAL_TAPES=$(find "$TAPE_DIR" -maxdepth 1 -name "*.tape" -type f | wc -l | tr -d ' ')

if [ "$TOTAL_TAPES" -eq 0 ]; then
    echo -e "${RED}❌ No .tape files found in $TAPE_DIR${NC}"
    exit 1
fi

echo -e "${BLUE}📼 Found $TOTAL_TAPES tape(s) to generate${NC}\n"

# Counter for progress
CURRENT=0
FAILED=0
SUCCEEDED=0

# Process each .tape file
while IFS= read -r tape_file; do
    CURRENT=$((CURRENT + 1))
    TAPE_NAME=$(basename "$tape_file" .tape)

    echo -e "${BLUE}[$CURRENT/$TOTAL_TAPES]${NC} Generating ${GREEN}$TAPE_NAME${NC}..."

    # Run vhs and capture output
    if vhs "$tape_file" 2>&1; then
        SUCCEEDED=$((SUCCEEDED + 1))
        echo -e "${GREEN}✅ Successfully generated $TAPE_NAME.gif${NC}\n"
    else
        FAILED=$((FAILED + 1))
        echo -e "${RED}❌ Failed to generate $TAPE_NAME${NC}\n"
    fi
done < <(find "$TAPE_DIR" -maxdepth 1 -name "*.tape" -type f | sort)

# Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}📊 Summary:${NC}"
echo -e "   Total tapes: $TOTAL_TAPES"
echo -e "   ${GREEN}Succeeded: $SUCCEEDED${NC}"
if [ "$FAILED" -gt 0 ]; then
    echo -e "   ${RED}Failed: $FAILED${NC}"
fi
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

if [ "$FAILED" -eq 0 ]; then
    echo -e "\n${GREEN}🎉 All demo GIFs generated successfully!${NC}"
    echo -e "${BLUE}📁 Output directory: $OUTPUT_DIR${NC}"
    exit 0
else
    echo -e "\n${RED}⚠️  Some tapes failed to generate${NC}"
    exit 1
fi
