#!/bin/bash
# Automated version bump script for tui-slider
# Usage: ./scripts/bump_version.sh <new_version>
# Example: ./scripts/bump_version.sh 0.2.0

set -e

# Cross-platform in-place sed helper
# macOS (BSD sed) requires an explicit empty extension: sed -i ''
# Linux (GNU sed) uses bare: sed -i
sed_inplace() {
    if [[ "$(uname)" == "Darwin" ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version number required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

NEW_VERSION=$1

# Validate version format (semantic versioning)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version must be in format: X.Y.Z or X.Y.Z-suffix (e.g., 0.2.0 or 0.2.0-beta.1)"
    exit 1
fi

echo -e "${CYAN}════════════════════════════════════════${NC}"
echo -e "${CYAN}  tui-slider Version Bump${NC}"
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo ""

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
echo -e "Current version: ${YELLOW}${CURRENT_VERSION}${NC}"
echo -e "New version:     ${GREEN}${NEW_VERSION}${NC}"
echo ""

# Ask for confirmation
read -p "Continue with version bump? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Aborted${NC}"
    exit 0
fi

echo ""
echo -e "${CYAN}Step 1/8: Updating Cargo.toml...${NC}"
sed_inplace "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" Cargo.toml
echo -e "${GREEN}✓ Cargo.toml updated${NC}"

echo ""
echo -e "${CYAN}Step 2/8: Updating README.md badges...${NC}"
if grep -q "version-[0-9]*\.[0-9]*\.[0-9]*-blue" README.md 2>/dev/null; then
    sed_inplace -E "s/version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?-blue/version-${NEW_VERSION}-blue/" README.md
    echo -e "${GREEN}✓ README.md updated${NC}"
else
    echo -e "${YELLOW}⚠ No version badge found in README.md${NC}"
fi

echo ""
echo -e "${CYAN}Step 3/8: Updating Cargo.lock...${NC}"
cargo update -p tui-slider
echo -e "${GREEN}✓ Cargo.lock updated${NC}"

echo ""
echo -e "${CYAN}Step 4/8: Running cargo fmt...${NC}"
cargo fmt
echo -e "${GREEN}✓ Code formatted${NC}"

echo ""
echo -e "${CYAN}Step 5/8: Running cargo clippy...${NC}"
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo -e "${RED}✗ Clippy found issues. Please fix them before continuing.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Clippy passed${NC}"

echo ""
echo -e "${CYAN}Step 6/8: Running tests...${NC}"
if ! cargo test --all-features --all-targets; then
    echo -e "${RED}✗ Tests failed. Please fix them before continuing.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ All tests passed${NC}"

echo ""
echo -e "${CYAN}Step 7/8: Generating CHANGELOG.md...${NC}"
if command -v git-cliff &> /dev/null; then
    git-cliff --tag "v${NEW_VERSION}" -o CHANGELOG.md
    echo -e "${GREEN}✓ Changelog generated${NC}"
else
    echo -e "${YELLOW}⚠ git-cliff not found. Skipping changelog generation.${NC}"
    echo -e "${YELLOW}  Install it with: cargo install git-cliff${NC}"
fi

echo ""
echo -e "${CYAN}Step 8/8: Creating git commit and tag...${NC}"

# Check if there are changes to commit
if git diff --quiet Cargo.toml Cargo.lock README.md CHANGELOG.md 2>/dev/null; then
    echo -e "${YELLOW}⚠ No changes to commit${NC}"
else
    git add Cargo.toml Cargo.lock README.md CHANGELOG.md

    git commit -m "chore: bump version to ${NEW_VERSION}

- Update version in Cargo.toml and README.md
- Update Cargo.lock
- Generate updated CHANGELOG.md"

    echo -e "${GREEN}✓ Changes committed${NC}"
fi

# Create tag
if git rev-parse "v${NEW_VERSION}" >/dev/null 2>&1; then
    echo -e "${YELLOW}⚠ Tag v${NEW_VERSION} already exists${NC}"
else
    git tag -a "v${NEW_VERSION}" -m "Release v${NEW_VERSION}

This release includes all changes documented in CHANGELOG.md for version ${NEW_VERSION}."
    echo -e "${GREEN}✓ Tag v${NEW_VERSION} created${NC}"
fi

echo ""
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo -e "${GREEN}✓ Version bump complete! 🚀${NC}"
echo -e "${CYAN}════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "  1. Review the changes:"
echo -e "     ${CYAN}git show${NC}"
echo -e ""
echo -e "  2. Push to remote:"
echo -e "     ${CYAN}git push origin main${NC}"
echo -e "     ${CYAN}git push origin v${NEW_VERSION}${NC}"
echo -e ""
echo -e "  3. Publish to crates.io:"
echo -e "     ${CYAN}cargo publish${NC}"
echo -e ""
echo -e "  4. Create GitHub release at:"
echo -e "     ${CYAN}https://github.com/YOUR_USERNAME/tui-slider/releases/new?tag=v${NEW_VERSION}${NC}"
echo ""
