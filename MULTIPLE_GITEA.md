# Multiple Gitea Instances & Selective Publishing

Complete guide for hosting on GitHub + multiple Gitea instances with selective publishing control.

## Overview

This setup allows you to:
- Host on GitHub (public/primary)
- Mirror to multiple Gitea instances (private, backup, team)
- Choose where to build and publish (GitHub Actions or Gitea Actions)
- Push to all remotes with one command

## Architecture

```
                    ┌─────────────┐
                    │   Local     │
                    │   Git Repo  │
                    └──────┬──────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
              ▼            ▼            ▼
       ┌──────────┐ ┌──────────┐ ┌──────────┐
       │ GitHub   │ │ Gitea 1  │ │ Gitea 2  │
       │ (Public) │ │(Personal)│ │  (Team)  │
       └────┬─────┘ └────┬─────┘ └────┬─────┘
            │            │            │
         ✓ CI/CD     ✓ CI/CD      ✓ CI/CD
         ✓ Releases  ✓ Backup     ✓ Backup
         ✓ Public    ✓ Private    ✓ Private
```

## Use Cases

### Use Case 1: Personal + Team Gitea
- **GitHub**: Public repository, community access
- **Gitea 1**: Personal backup on home server
- **Gitea 2**: Team server at work

### Use Case 2: Multiple Backups
- **GitHub**: Primary
- **Gitea 1**: Home server backup
- **Gitea 2**: Cloud VPS backup

### Use Case 3: Private Development
- **Gitea 1**: Private development server
- **Gitea 2**: Staging/testing server
- **GitHub**: Public releases only

## Quick Setup

### 1. Configure Justfile

Edit the top of your `justfile`:

```just
# Configuration
GITEA_REMOTE := "gitea"          # Primary Gitea (required)
GITEA_REMOTE_2 := "gitea2"       # Secondary Gitea (optional)
PUBLISH_TARGET := "github"       # Options: github, gitea, gitea2
```

### 2. Add Gitea Remotes

```bash
# Add primary Gitea
just setup-gitea git@gitea1.example.com:user/tui-slider.git

# Add secondary Gitea (optional)
just setup-gitea2 git@gitea2.example.com:user/tui-slider.git

# Verify
just remotes
```

### 3. Push to All Remotes

```bash
# Push to all configured remotes
just push-all

# Or individually
just push           # GitHub only
just push-gitea     # Primary Gitea only
just push-gitea2    # Secondary Gitea only
```

## Detailed Configuration

### Remote Names

The default remote names are:
- `origin` - GitHub (always)
- `gitea` - Primary Gitea
- `gitea2` - Secondary Gitea

You can customize these by editing the justfile configuration.

### Publish Target

The `PUBLISH_TARGET` setting controls where `just publish` publishes from:

```just
PUBLISH_TARGET := "github"   # Publish from GitHub Actions
PUBLISH_TARGET := "gitea"    # Publish from primary Gitea Actions
PUBLISH_TARGET := "gitea2"   # Publish from secondary Gitea Actions
```

This is useful when you want to:
- Build on private infrastructure
- Use specific CI/CD features
- Comply with organizational policies

## Available Commands

### Push Commands

```bash
just push           # Push to GitHub only
just push-gitea     # Push to primary Gitea
just push-gitea2    # Push to secondary Gitea
just push-all       # Push to ALL configured remotes

just push-tags-all  # Push tags to all remotes
```

### Release Commands

```bash
# Standard release (pushes to all, publishes from PUBLISH_TARGET)
just release 0.2.0

# Full automated release (includes publish)
just release-full 0.2.0

# Release to specific platform
just release-to github 0.2.0
just release-to gitea 0.2.0
just release-to gitea2 0.2.0
```

### Sync Commands

```bash
# Sync individual Gitea with GitHub
just sync-gitea
just sync-gitea2

# Sync all Gitea instances with GitHub
just sync-all-gitea
```

### Info Commands

```bash
just info           # Show project info and configuration
just remotes        # List all configured remotes
just help-gitea     # Show Gitea setup help
```

## Workflow Examples

### Example 1: Push to All

Daily development workflow:

```bash
# Make changes
git add .
git commit -m "feat: add new feature"

# Push to all remotes at once
just push-all
```

### Example 2: Release from GitHub

Standard public release:

```bash
# Set in justfile
PUBLISH_TARGET := "github"

# Create release (pushes to all, publishes from GitHub)
just release 0.2.0
```

### Example 3: Release from Private Gitea

Private release on your infrastructure:

```bash
# Set in justfile
PUBLISH_TARGET := "gitea"

# Create release (pushes to all, publishes from Gitea)
just release 0.2.0
```

### Example 4: Sync All

Keep all Gitea instances in sync with GitHub:

```bash
# Push latest to GitHub first
git push origin main

# Sync all Gitea instances
just sync-all-gitea
```

## CI/CD Configuration

### GitHub Actions

Keep your existing `.github/workflows/` as-is. No changes needed.

### Gitea Actions (Primary)

Create `.gitea/workflows/`:

```bash
mkdir -p .gitea/workflows
cp .github/workflows/ci.yml .gitea/workflows/
cp .github/workflows/release.yml .gitea/workflows/
```

Edit `.gitea/workflows/release.yml` to match your Gitea setup.

### Gitea Actions (Secondary)

For secondary Gitea, create `.gitea2/workflows/`:

```bash
mkdir -p .gitea2/workflows
cp .github/workflows/* .gitea2/workflows/
```

### Publishing to crates.io

Each platform needs the `CRATES_IO_TOKEN` secret:

**GitHub:**
1. Settings → Secrets → Actions
2. Add `CRATES_IO_TOKEN`

**Gitea 1:**
1. Settings → Secrets
2. Add `CRATES_IO_TOKEN`

**Gitea 2:**
1. Settings → Secrets
2. Add `CRATES_IO_TOKEN`

## Migration Workflow

### Migrate Existing Project

Use the migration script:

```bash
# From tui-slider directory
./scripts/migrate-to-gitea.sh /path/to/project git@gitea.example.com:user/project.git
```

Or manually:

```bash
cd /path/to/project

# Copy setup files
cp /path/to/tui-slider/justfile .
cp /path/to/tui-slider/*GITEA*.md .
cp /path/to/tui-slider/SSH_SETUP.md .

# Add remotes
git remote add gitea git@gitea1.com:user/project.git
git remote add gitea2 git@gitea2.com:user/project.git

# Push
just push-all
```

### Migrate Multiple Projects

Migrate all your projects at once:

```bash
# Create a migration script
for project in tui-piechart tui-checkbox tui-slider; do
    ./scripts/migrate-to-gitea.sh \
        ~/Projects/$project \
        git@gitea.example.com:username/$project.git
done
```

## Advanced Configuration

### Different Remotes Per Project

Each project can have different Gitea instances:

**tui-slider:**
```just
GITEA_REMOTE := "gitea-home"
GITEA_REMOTE_2 := "gitea-work"
```

**tui-piechart:**
```just
GITEA_REMOTE := "gitea-backup"
GITEA_REMOTE_2 := "gitea-team"
```

### Conditional Publishing

Publish based on branch or tag:

```just
# Only publish from main branch on GitHub
publish:
    #!/usr/bin/env bash
    BRANCH=$(git branch --show-current)
    if [ "$BRANCH" = "main" ]; then
        cargo publish
    else
        echo "Only publish from main branch"
    fi
```

### Custom Push Strategy

Push to different remotes based on branch:

```just
push:
    #!/usr/bin/env bash
    BRANCH=$(git branch --show-current)
    if [ "$BRANCH" = "main" ]; then
        just push-all
    else
        git push origin $BRANCH
    fi
```

## Troubleshooting

### Remote Not Found

```bash
# Check configured remotes
just remotes

# Add missing remote
just setup-gitea git@gitea.example.com:user/repo.git
```

### Push Fails to Secondary Gitea

```bash
# Check if remote exists
git remote -v | grep gitea2

# Add if missing
just setup-gitea2 git@gitea2.example.com:user/repo.git

# Test connection
git ls-remote gitea2
```

### Publishing to Wrong Platform

```bash
# Check current configuration
just info

# Edit justfile to change PUBLISH_TARGET
vim justfile  # or your editor

# Verify
just info
```

### Out of Sync Remotes

```bash
# Fetch from all
git fetch --all

# Check differences
git log origin/main..gitea/main
git log origin/main..gitea2/main

# Force sync all
just sync-all-gitea
```

## Security Considerations

### Separate Tokens

Use different `CRATES_IO_TOKEN` for each platform if needed:
- GitHub: Production token
- Gitea 1: Staging token
- Gitea 2: Development token

### Access Control

Configure different access levels:
- GitHub: Public read, restricted write
- Gitea 1: Private, personal access
- Gitea 2: Team access with branch protection

### SSH Keys

Use separate SSH keys per Gitea instance in `~/.ssh/config`:

```
Host gitea1.example.com
    HostName gitea1.example.com
    User git
    IdentityFile ~/.ssh/id_ed25519_gitea1

Host gitea2.example.com
    HostName gitea2.example.com
    User git
    IdentityFile ~/.ssh/id_ed25519_gitea2
```

## Best Practices

1. **Always use SSH** for Gitea remotes (password-free)
2. **Keep one source of truth** (usually GitHub)
3. **Sync regularly** (`just sync-all-gitea`)
4. **Document your setup** (update project README)
5. **Test before publishing** (`just publish-dry`)
6. **Use consistent naming** (gitea, gitea2, etc.)
7. **Backup your backups** (multiple Gitea instances)

## Project-Specific Examples

### tui-slider Setup

```just
GITEA_REMOTE := "gitea"
GITEA_REMOTE_2 := "gitea-backup"
PUBLISH_TARGET := "github"
```

```bash
just setup-gitea git@gitea.home:sorin/tui-slider.git
just setup-gitea2 git@gitea.vps:sorin/tui-slider.git
just push-all
```

### tui-piechart Setup

```just
GITEA_REMOTE := "gitea"
GITEA_REMOTE_2 := "gitea-team"
PUBLISH_TARGET := "gitea"
```

```bash
just setup-gitea git@gitea.home:sorin/tui-piechart.git
just setup-gitea2 git@gitea.work:team/tui-piechart.git
just push-all
```

### tui-checkbox Setup

```just
GITEA_REMOTE := "gitea"
PUBLISH_TARGET := "github"
# No secondary Gitea for this project
```

```bash
just setup-gitea git@gitea.home:sorin/tui-checkbox.git
just push-all
```

## Automation

### Git Hooks

Auto-push to all remotes after commit:

Create `.git/hooks/post-commit`:

```bash
#!/bin/bash
just push-all
```

### Pre-Push Hook

Verify before pushing:

Create `.git/hooks/pre-push`:

```bash
#!/bin/bash
just check-all
```

## Summary

You now have:
- ✅ Multiple Gitea instance support
- ✅ Selective publishing control
- ✅ Single-command push to all remotes
- ✅ Flexible CI/CD configuration
- ✅ Complete backup redundancy

## Quick Reference

```bash
# Setup
just setup-gitea <url>
just setup-gitea2 <url>

# Push
just push-all              # All remotes
just push-gitea            # Primary Gitea
just push-gitea2           # Secondary Gitea

# Release
just release 0.2.0         # All remotes, publish from PUBLISH_TARGET
just release-to github 0.2.0   # Specific platform

# Sync
just sync-all-gitea        # Sync all Gitea with GitHub

# Info
just info                  # Show configuration
just remotes               # List remotes
just help-gitea            # Gitea help
```

## Resources

- [GITEA_SETUP.md](./GITEA_SETUP.md) - Basic Gitea setup
- [SSH_SETUP.md](./SSH_SETUP.md) - SSH configuration
- [DUAL_HOSTING.md](./DUAL_HOSTING.md) - Dual hosting guide
- [justfile](./justfile) - All available commands

---

**Need help?** Run `just help-gitea` for quick reference.