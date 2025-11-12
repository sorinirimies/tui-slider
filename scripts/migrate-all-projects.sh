#!/usr/bin/env bash
# Batch migration script for multiple TUI projects to Gitea
# Usage: ./migrate-all-projects.sh [gitea-host]

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

# Helper functions
error() {
    echo -e "${RED}❌ Error: $1${NC}" >&2
    exit 1
}

success() {
    echo -e "${GREEN}✅ $1${NC}"
}

info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

heading() {
    echo ""
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}  $1${NC}"
    echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Get Gitea host
if [ -n "$1" ]; then
    GITEA_HOST="$1"
else
    echo "Enter your Gitea host (e.g., gitea.example.com):"
    read -r GITEA_HOST
fi

if [ -z "$GITEA_HOST" ]; then
    error "Gitea host is required"
fi

# Get username
echo "Enter your Gitea username:"
read -r GITEA_USER

if [ -z "$GITEA_USER" ]; then
    error "Gitea username is required"
fi

# Projects to migrate
PROJECTS=(
    "tui-slider"
    "tui-piechart"
    "tui-checkbox"
)

# Base directory
BASE_DIR="$HOME/Projects"

# Find migration script
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
MIGRATE_SCRIPT="$SCRIPT_DIR/migrate-to-gitea.sh"

if [ ! -f "$MIGRATE_SCRIPT" ]; then
    error "Migration script not found: $MIGRATE_SCRIPT"
fi

# Summary
heading "Batch Migration to Gitea"
echo ""
info "Gitea Host: $GITEA_HOST"
info "Username: $GITEA_USER"
info "Base Directory: $BASE_DIR"
info "Projects to migrate: ${#PROJECTS[@]}"
echo ""

for project in "${PROJECTS[@]}"; do
    echo "  • $project"
done

echo ""
read -p "Continue with migration? (y/N) " -n 1 -r
echo ""
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    info "Migration cancelled"
    exit 0
fi

# Track results
SUCCESSFUL=()
FAILED=()
SKIPPED=()

# Migrate each project
for project in "${PROJECTS[@]}"; do
    heading "Migrating: $project"

    PROJECT_DIR="$BASE_DIR/$project"
    GITEA_URL="git@$GITEA_HOST:$GITEA_USER/$project.git"

    # Check if project exists
    if [ ! -d "$PROJECT_DIR" ]; then
        warning "Project directory not found: $PROJECT_DIR"
        SKIPPED+=("$project")
        continue
    fi

    # Check if it's a git repo
    if [ ! -d "$PROJECT_DIR/.git" ]; then
        warning "Not a git repository: $PROJECT_DIR"
        SKIPPED+=("$project")
        continue
    fi

    # Run migration
    echo ""
    info "Running migration for $project..."
    info "URL: $GITEA_URL"
    echo ""

    if "$MIGRATE_SCRIPT" "$PROJECT_DIR" "$GITEA_URL"; then
        success "Successfully migrated $project"
        SUCCESSFUL+=("$project")
    else
        error "Failed to migrate $project"
        FAILED+=("$project")
    fi

    echo ""
    read -p "Continue to next project? (Y/n) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        warning "Stopping migration"
        break
    fi
done

# Final summary
heading "Migration Summary"
echo ""

if [ ${#SUCCESSFUL[@]} -gt 0 ]; then
    success "Successfully migrated (${#SUCCESSFUL[@]}):"
    for project in "${SUCCESSFUL[@]}"; do
        echo -e "  ${GREEN}✓${NC} $project"
    done
    echo ""
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    error "Failed to migrate (${#FAILED[@]}):"
    for project in "${FAILED[@]}"; do
        echo -e "  ${RED}✗${NC} $project"
    done
    echo ""
fi

if [ ${#SKIPPED[@]} -gt 0 ]; then
    warning "Skipped (${#SKIPPED[@]}):"
    for project in "${SKIPPED[@]}"; do
        echo -e "  ${YELLOW}○${NC} $project"
    done
    echo ""
fi

# Next steps
if [ ${#SUCCESSFUL[@]} -gt 0 ]; then
    heading "Next Steps"
    echo ""
    info "For each migrated project, you can now:"
    echo ""
    echo "1. Push to all remotes:"
    echo "   cd $BASE_DIR/[project]"
    echo "   just push-all"
    echo ""
    echo "2. Verify remotes:"
    echo "   just remotes"
    echo ""
    echo "3. View available commands:"
    echo "   just --list"
    echo ""
    echo "4. Setup secondary Gitea (optional):"
    echo "   just setup-gitea2 git@gitea2.com:user/[project].git"
    echo ""
fi

# Create summary file
SUMMARY_FILE="$HOME/gitea-migration-$(date +%Y%m%d-%H%M%S).txt"
cat > "$SUMMARY_FILE" << EOF
Gitea Migration Summary
Date: $(date)
Gitea Host: $GITEA_HOST
Username: $GITEA_USER

Successful Migrations (${#SUCCESSFUL[@]}):
$(printf '%s\n' "${SUCCESSFUL[@]}" | sed 's/^/  - /')

Failed Migrations (${#FAILED[@]}):
$(printf '%s\n' "${FAILED[@]}" | sed 's/^/  - /')

Skipped (${#SKIPPED[@]}):
$(printf '%s\n' "${SKIPPED[@]}" | sed 's/^/  - /')

Next Steps:
1. Test each project: cd ~/Projects/[project] && just push-all
2. Verify remotes: just remotes
3. Review documentation in each project
EOF

success "Summary saved to: $SUMMARY_FILE"
echo ""

# Final message
heading "Migration Complete!"
echo ""
success "Batch migration finished!"
echo ""
info "Total projects: ${#PROJECTS[@]}"
success "Successful: ${#SUCCESSFUL[@]}"
if [ ${#FAILED[@]} -gt 0 ]; then
    error "Failed: ${#FAILED[@]}"
fi
if [ ${#SKIPPED[@]} -gt 0 ]; then
    warning "Skipped: ${#SKIPPED[@]}"
fi
echo ""
success "Happy dual-hosting! 🚀"
