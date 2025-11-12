# Quick Gitea Setup Guide

This guide helps you set up dual hosting for tui-slider on GitHub and your Gitea instance.

## Prerequisites

- ✅ Existing GitHub repository (already set up)
- ✅ Access to your Gitea instance
- ✅ SSH keys configured (recommended) or HTTPS credentials

## Quick Setup

### Option 1: Automated Setup (Recommended)

Run the setup script:

```bash
# With SSH (recommended)
./scripts/setup-gitea.sh git@gitea.yourdomain.com:username/tui-slider.git

# Or with HTTPS
./scripts/setup-gitea.sh https://gitea.yourdomain.com/username/tui-slider.git
```

The script will:
- Add Gitea as a remote
- Test the connection
- Optionally push all branches and tags
- Set up Gitea Actions (CI/CD)

### Option 2: Manual Setup

```bash
# 1. Add Gitea remote
git remote add gitea git@gitea.yourdomain.com:username/tui-slider.git

# 2. Push all branches
git push gitea --all

# 3. Push all tags
git push gitea --tags

# 4. Verify
git remote -v
```

## Daily Usage

### Push to Both Remotes

```bash
# Using justfile (recommended)
just push-all

# Or manually
git push origin main
git push gitea main
```

### Create a Release

```bash
# Automatically pushes to both GitHub and Gitea
just release 0.2.0
```

### Check Remote Status

```bash
just remotes
```

## Available Commands

```bash
just push           # Push to GitHub only
just push-gitea     # Push to Gitea only
just push-all       # Push to both remotes
just push-tags-all  # Push tags to both remotes
just sync-gitea     # Force sync Gitea with GitHub
just remotes        # Show configured remotes
```

## Gitea Configuration

### 1. Create Repository on Gitea

1. Log in to your Gitea instance
2. Click "+" → "New Repository"
3. Repository name: `tui-slider`
4. Description: "A simple TUI slider component library for ratatui"
5. Visibility: Public or Private (your choice)
6. **Do NOT** initialize with README (we'll push from local)
7. Click "Create Repository"

### 2. Configure SSH Keys (Recommended)

```bash
# Generate SSH key if you don't have one
ssh-keygen -t ed25519 -C "your_email@example.com"

# Copy public key
cat ~/.ssh/id_ed25519.pub

# Add to Gitea:
# Settings → SSH/GPG Keys → Add Key
```

### 3. Configure Gitea Actions (Optional)

If your Gitea instance supports Actions (Gitea 1.19+):

```bash
# Create Gitea workflows directory
mkdir -p .gitea/workflows

# Copy GitHub workflows
cp .github/workflows/ci.yml .gitea/workflows/
cp .github/workflows/release.yml .gitea/workflows/

# Commit
git add .gitea/
git commit -m "ci: add Gitea Actions workflows"
git push gitea main
```

## Syncing Strategies

### Strategy 1: Always Push to Both (Recommended)

```bash
# Set up in .git/config
git remote set-url --add --push origin git@gitea.yourdomain.com:username/tui-slider.git
```

Now `git push origin main` pushes to both!

### Strategy 2: Use Justfile Commands

```bash
# Already configured in justfile
just push-all      # Pushes to both
just release 0.2.0 # Releases to both
```

### Strategy 3: GitHub Primary, Gitea Mirror

```bash
# Push to GitHub first
git push origin main

# Then sync to Gitea
just sync-gitea
```

## CI/CD on Gitea

### Gitea Actions (GitHub Actions Compatible)

If using Gitea Actions, workflows in `.gitea/workflows/` run automatically.

Configure runners:
1. Gitea Admin Panel → Actions → Runners
2. Follow runner setup instructions
3. Workflows will execute on push/PR

### Drone CI

If using Drone CI, create `.drone.yml`:

```yaml
kind: pipeline
type: docker
name: default

steps:
  - name: test
    image: rust:latest
    commands:
      - cargo fmt -- --check
      - cargo clippy -- -D warnings
      - cargo test --all-features
```

### Woodpecker CI

If using Woodpecker, create `.woodpecker.yml`:

```yaml
pipeline:
  test:
    image: rust:latest
    commands:
      - cargo test --all-features
```

## Troubleshooting

### Permission Denied (SSH)

```bash
# Test SSH connection
ssh -T git@gitea.yourdomain.com

# If fails, add SSH key to Gitea
cat ~/.ssh/id_ed25519.pub
# Copy and add in Gitea Settings → SSH Keys
```

### Authentication Failed (HTTPS)

```bash
# Use token authentication
git remote set-url gitea https://username:token@gitea.yourdomain.com/username/tui-slider.git

# Generate token in Gitea:
# Settings → Applications → Generate New Token
```

### Push Failed - Repository Not Found

```bash
# Make sure repository exists on Gitea
# Create it via Gitea UI first, then:
git push gitea --all --force
```

### Remotes Out of Sync

```bash
# Fetch from both
git fetch --all

# Force sync Gitea with GitHub
just sync-gitea
```

## Verification

Check your setup:

```bash
# 1. View remotes
just remotes

# Should show:
# origin  git@github.com:sorinirimies/tui-slider.git
# gitea   git@gitea.yourdomain.com:username/tui-slider.git

# 2. Test push to Gitea
git push gitea main --dry-run

# 3. Check branches on both remotes
git branch -r
```

## Migration Back to Single Remote

If you want to remove Gitea hosting:

```bash
# Remove Gitea remote
git remote remove gitea

# Update justfile commands (revert to single remote)
# Edit justfile: remove gitea-related commands
```

## Resources

- 📖 [Full Dual Hosting Guide](./DUAL_HOSTING.md)
- 🔧 [Justfile Commands](./justfile) - Run `just --list`
- 🚀 [Release Workflow](./RELEASE.md)
- 🌐 [Gitea Documentation](https://docs.gitea.io/)

## Quick Reference

```bash
# Setup
./scripts/setup-gitea.sh git@gitea.example.com:user/tui-slider.git

# Daily use
just push-all                # Push to both
just release 0.2.0           # Release to both

# Maintenance
just sync-gitea              # Sync Gitea with GitHub
just remotes                 # Show remotes

# Manual
git push origin main         # GitHub only
git push gitea main          # Gitea only
```

## Success Criteria

You've successfully set up Gitea when:

- ✅ `just remotes` shows both origin and gitea
- ✅ `git push gitea main` works without errors
- ✅ `just push-all` pushes to both remotes
- ✅ Repository is visible on your Gitea instance
- ✅ `just release X.Y.Z` pushes to both GitHub and Gitea

Need help? Check [DUAL_HOSTING.md](./DUAL_HOSTING.md) for detailed information.