#!/usr/bin/env bash
# Setup script for Gitea dual hosting
# Usage: ./scripts/setup-gitea.sh <gitea-url>

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Check if URL is provided
if [ -z "$1" ]; then
    error "Usage: $0 <gitea-url>

Examples (SSH recommended):
  $0 git@gitea.example.com:username/tui-slider.git
  $0 https://gitea.example.com/username/tui-slider.git"
fi

GITEA_URL="$1"

# Check if URL uses SSH
if [[ "$GITEA_URL" == git@* ]]; then
    USE_SSH=true
    # Extract hostname from git@hostname:path format
    GITEA_HOST=$(echo "$GITEA_URL" | sed 's/git@\([^:]*\):.*/\1/')
else
    USE_SSH=false
fi

info "Setting up Gitea dual hosting for tui-slider"
echo ""

# Check if git is installed
if ! command -v git &> /dev/null; then
    error "git is not installed. Please install git first."
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    error "Not a git repository. Please run this script from the tui-slider directory."
fi

# Check SSH setup if using SSH
if [ "$USE_SSH" = true ]; then
    info "Using SSH connection to $GITEA_HOST"

    # Check if SSH keys exist
    if [ ! -f ~/.ssh/id_rsa ] && [ ! -f ~/.ssh/id_ed25519 ] && [ ! -f ~/.ssh/id_ecdsa ]; then
        warning "No SSH keys found in ~/.ssh/"
        echo ""
        info "To generate SSH keys:"
        echo "  ssh-keygen -t ed25519 -C \"your_email@example.com\""
        echo ""
        info "Then add the public key to your Gitea instance:"
        echo "  1. Copy key: cat ~/.ssh/id_ed25519.pub"
        echo "  2. Gitea UI → Settings → SSH/GPG Keys → Add Key"
        echo ""
        read -p "Do you want to continue anyway? (y/N) " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            error "SSH keys required for SSH URLs. Setup SSH keys first."
        fi
    else
        success "SSH keys found"

        # Test SSH connection
        info "Testing SSH connection to $GITEA_HOST..."
        if ssh -T git@"$GITEA_HOST" 2>&1 | grep -q "successfully authenticated\|Hi there"; then
            success "SSH connection successful!"
        else
            warning "Could not verify SSH connection"
            info "This is normal if:"
            echo "  • SSH key not yet added to Gitea"
            echo "  • Gitea server uses custom SSH config"
            echo ""
            info "To add your SSH key to Gitea:"
            if [ -f ~/.ssh/id_ed25519.pub ]; then
                echo "  1. Copy: cat ~/.ssh/id_ed25519.pub"
            elif [ -f ~/.ssh/id_rsa.pub ]; then
                echo "  1. Copy: cat ~/.ssh/id_rsa.pub"
            fi
            echo "  2. Gitea UI → Settings → SSH/GPG Keys → Add Key"
            echo ""
        fi
    fi
else
    info "Using HTTPS connection"
    warning "HTTPS will prompt for credentials. SSH is recommended for better workflow."
fi
echo ""

# Check if gitea remote already exists
if git remote | grep -q "^gitea$"; then
    warning "Gitea remote already exists. Updating URL..."
    git remote set-url gitea "$GITEA_URL"
    success "Gitea remote URL updated"
else
    info "Adding Gitea remote..."
    git remote add gitea "$GITEA_URL"
    success "Gitea remote added"
fi

# Show current remotes
info "Current remotes:"
git remote -v | grep -E "^(origin|gitea)" | sed 's/^/  /'
echo ""

# Test Gitea connection
info "Testing Gitea repository connection..."
if git ls-remote gitea > /dev/null 2>&1; then
    success "Successfully connected to Gitea repository!"
else
    warning "Could not connect to Gitea repository."
    echo ""
    info "This is normal if the repository doesn't exist yet."
    echo ""
    info "To create the repository on Gitea:"
    echo "  1. Log in to your Gitea instance"
    echo "  2. Click '+' → New Repository"
    echo "  3. Repository name: tui-slider"
    echo "  4. Do NOT initialize with README"
    echo "  5. Click 'Create Repository'"
    echo "  6. Then run: just push-gitea (or git push gitea --all)"
    echo ""
fi

# Offer to push to Gitea
read -p "Do you want to push all branches and tags to Gitea now? (y/N) " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    info "Pushing to Gitea..."

    # Push all branches
    if git push gitea --all 2>&1; then
        success "All branches pushed to Gitea"
    else
        warning "Failed to push branches. The repository might not exist yet."
    fi

    # Push all tags
    if git push gitea --tags 2>&1; then
        success "All tags pushed to Gitea"
    else
        warning "Failed to push tags."
    fi
fi
echo ""

# Create .gitea directory for Gitea Actions (if needed)
if [ ! -d ".gitea" ]; then
    read -p "Do you want to set up Gitea Actions (CI/CD)? (y/N) " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        info "Creating .gitea/workflows directory..."
        mkdir -p .gitea/workflows

        # Copy GitHub workflows to Gitea
        if [ -d ".github/workflows" ]; then
            info "Copying workflows from .github to .gitea..."
            cp -r .github/workflows/* .gitea/workflows/
            success "Workflows copied to .gitea/workflows/"
            warning "Note: You may need to adjust these workflows for your Gitea setup"
        fi
    fi
fi
echo ""

# Summary
success "Gitea setup complete!"
echo ""
info "Next steps:"
echo "  1. Push to both remotes: just push-all"
echo "  2. For releases: just release <version>"
echo "  3. View remotes: just remotes"
echo "  4. Sync Gitea manually: just sync-gitea"
echo ""
info "Documentation:"
echo "  • Dual hosting guide: DUAL_HOSTING.md"
echo "  • Justfile commands: just --list"
echo ""
info "Quick commands:"
echo "  • Push to GitHub: git push origin main"
echo "  • Push to Gitea: git push gitea main (or: just push-gitea)"
echo "  • Push to both: just push-all"
echo ""
if [ "$USE_SSH" = true ]; then
    success "SSH setup complete - no passwords needed! 🔑"
else
    info "Note: HTTPS will prompt for credentials. Consider switching to SSH:"
    echo "  git remote set-url gitea git@$GITEA_HOST:username/tui-slider.git"
    echo ""
fi
success "Happy coding! 🚀"
