# tui-slider - A simple TUI slider component library for ratatui
# Install just: cargo install just
# Install git-cliff: cargo install git-cliff
# Usage: just <task>

# Configuration
# Set these to match your setup
GITEA_REMOTE := "gitea"
GITEA_REMOTE_2 := "gitea2"  # Optional second Gitea instance
PUBLISH_TARGET := "github"  # Options: github, gitea, gitea2

# Default task - show available commands
default:
    @just --list

# Install required tools (just, git-cliff)
install-tools:
    @echo "Installing required tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ All tools installed!"

# Build the library
build:
    cargo build --all-features

# Build release version
build-release:
    cargo build --release --all-features

# Run horizontal slider example
example-horizontal:
    cargo run --example horizontal

# Run vertical slider example
example-vertical:
    cargo run --example vertical

# Run custom slider example
example-custom:
    cargo run --example custom

# Run thumb toggle example
example-thumb:
    cargo run --example thumb_toggle

# Run all examples
examples: example-horizontal example-vertical example-custom example-thumb

# Run all tests
test:
    cargo test --all-features

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt --all

# Check if code is formatted
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (fmt, clippy, test)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# Clean build artifacts
clean:
    cargo clean

# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found. Install with: cargo install git-cliff"; exit 1; }

# Generate full changelog from all tags
changelog: check-git-cliff
    @echo "Generating full changelog..."
    git-cliff -o CHANGELOG.md
    @echo "✅ Changelog generated!"

# Generate changelog for unreleased commits only
changelog-unreleased: check-git-cliff
    @echo "Generating unreleased changelog..."
    git-cliff --unreleased --prepend CHANGELOG.md
    @echo "✅ Unreleased changelog generated!"

# Generate changelog for specific version tag
changelog-version version: check-git-cliff
    @echo "Generating changelog for version {{version}}..."
    git-cliff --tag v{{version}} -o CHANGELOG.md
    @echo "✅ Changelog generated for version {{version}}!"

# Preview changelog without writing to file
changelog-preview: check-git-cliff
    @git-cliff

# Preview unreleased changes
changelog-preview-unreleased: check-git-cliff
    @git-cliff --unreleased

# Generate changelog for latest tag only
changelog-latest: check-git-cliff
    @echo "Generating changelog for latest tag..."
    git-cliff --latest -o CHANGELOG.md
    @echo "✅ Latest changelog generated!"

# Update changelog with all commits (force regenerate)
changelog-update: check-git-cliff
    @echo "Regenerating complete changelog from all tags..."
    git-cliff --output CHANGELOG.md
    @echo "✅ Changelog updated from all git history!"

# Bump version (usage: just bump 0.2.0)
bump version: check-git-cliff
    @echo "Bumping version to {{version}}..."
    @# Update version in Cargo.toml
    @sed -i.bak 's/^version = ".*"/version = "{{version}}"/' Cargo.toml && rm Cargo.toml.bak
    @# Update Cargo.lock
    @cargo build
    @# Generate changelog for this version
    @git-cliff --tag v{{version}} -o CHANGELOG.md
    @# Commit changes
    @git add Cargo.toml Cargo.lock CHANGELOG.md
    @git commit -m "chore(release): bump version to {{version}}"
    @# Create git tag
    @git tag -a v{{version}} -m "Release v{{version}}"
    @echo "✅ Version bumped to {{version}}!"
    @echo "📝 Changelog updated"
    @echo "🏷️  Tag v{{version}} created"
    @echo ""
    @echo "Pushing to configured remotes..."
    @just push-release-configured
    @echo "✅ Release v{{version}} pushed to all configured remotes!"

# Quick release check: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io from configured target
publish:
    @echo "Publishing from {{PUBLISH_TARGET}}..."
    @just publish-from-{{PUBLISH_TARGET}}

# Publish from GitHub (default)
publish-from-github:
    @echo "Publishing to crates.io from GitHub..."
    cargo publish

# Publish from Gitea (if you have Gitea Actions set up)
publish-from-gitea:
    @echo "Publishing to crates.io from Gitea..."
    @echo "⚠️  Make sure CRATES_IO_TOKEN is set in Gitea secrets"
    cargo publish

# Publish from secondary Gitea
publish-from-gitea2:
    @echo "Publishing to crates.io from {{GITEA_REMOTE_2}}..."
    @echo "⚠️  Make sure CRATES_IO_TOKEN is set in {{GITEA_REMOTE_2}} secrets"
    cargo publish

# Dry-run publish to test
publish-dry:
    cargo publish --dry-run

# Push release to GitHub only
push-release:
    @echo "Pushing release to GitHub..."
    git push origin main
    git push origin --tags
    @echo "✅ Release pushed to GitHub!"

# Push release to all configured remotes
push-release-all:
    @just push-release-configured

# Internal: Push release based on configuration
push-release-configured:
    #!/usr/bin/env bash
    echo "Pushing release to configured remotes..."
    git push origin main
    git push origin --tags
    git push {{GITEA_REMOTE}} main
    git push {{GITEA_REMOTE}} --tags
    if git remote | grep -q "^{{GITEA_REMOTE_2}}$"; then
        git push {{GITEA_REMOTE_2}} main
        git push {{GITEA_REMOTE_2}} --tags
    fi
    echo "✅ Release pushed to all remotes!"

# Full release workflow: check, bump version, and push
release version: release-check (bump version)
    @echo ""
    @echo "🎉 Release v{{version}} complete!"
    @echo ""
    @echo "Next step:"
    @echo "  Publish to crates.io: just publish"
    @echo ""

# Complete release workflow including push and publish
release-full version: (release version)
    @echo ""
    @echo "Publishing to crates.io from {{PUBLISH_TARGET}}..."
    @just publish
    @echo ""
    @echo "✅ Release v{{version}} complete on all platforms!"

# Release to specific platform
release-to platform version: release-check (bump version)
    @echo ""
    @echo "🎉 Release v{{version}} prepared!"
    @echo "Publishing to {{platform}}..."
    @just publish-from-{{platform}}
    @echo ""
    @echo "✅ Release v{{version}} complete on {{platform}}!"

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Generate documentation
doc:
    cargo doc --all-features --no-deps --open

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x "run --example horizontal"

# Watch and run tests on changes
watch-test:
    cargo watch -x test

# Watch and run specific example on changes
watch-example EXAMPLE:
    cargo watch -x "run --example {{EXAMPLE}}"

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: push to origin (GitHub)
push:
    git push origin main

# Git: push to primary Gitea
push-gitea:
    git push {{GITEA_REMOTE}} main

# Git: push to secondary Gitea (if configured)
push-gitea2:
    #!/usr/bin/env bash
    if git remote | grep -q "^{{GITEA_REMOTE_2}}$"; then
        git push {{GITEA_REMOTE_2}} main
        echo "✅ Pushed to {{GITEA_REMOTE_2}}!"
    else
        echo "⚠️  Remote {{GITEA_REMOTE_2}} not configured"
    fi

# Git: push to both GitHub and all Gitea instances
push-all:
    #!/usr/bin/env bash
    git push origin main
    git push {{GITEA_REMOTE}} main
    if git remote | grep -q "^{{GITEA_REMOTE_2}}$"; then
        git push {{GITEA_REMOTE_2}} main
    fi
    echo "✅ Pushed to all configured remotes!"

# Git: push tags to origin
push-tags:
    git push --tags

# Git: push tags to all remotes
push-tags-all:
    #!/usr/bin/env bash
    git push origin --tags
    git push {{GITEA_REMOTE}} --tags
    if git remote | grep -q "^{{GITEA_REMOTE_2}}$"; then
        git push {{GITEA_REMOTE_2}} --tags
    fi
    echo "✅ Tags pushed to all configured remotes!"

# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show git-cliff info
cliff-info:
    @echo "Git-cliff configuration:"
    @echo "  Config file: cliff.toml"
    @echo "  Installed: $(command -v git-cliff >/dev/null 2>&1 && echo '✅ Yes' || echo '❌ No (run: just install-tools)')"
    @command -v git-cliff >/dev/null 2>&1 && git-cliff --version || true

# Show project info
info:
    @echo "Project: tui-slider"
    @echo "Version: $(just version)"
    @echo "Author: Sorin Irimies <sorinirimies@gmail.com>"
    @echo "License: MIT"
    @echo "Repository (GitHub): https://github.com/sorinirimies/tui-slider"
    @echo "Publish Target: {{PUBLISH_TARGET}}"
    @echo ""
    @echo "Configured Gitea remotes:"
    @git remote -v | grep gitea || echo "  No Gitea remotes configured"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Setup primary Gitea remote
setup-gitea url:
    @echo "Adding primary Gitea remote ({{GITEA_REMOTE}})..."
    git remote add {{GITEA_REMOTE}} {{url}}
    @echo "✅ {{GITEA_REMOTE}} remote added!"
    @echo "Test with: just push-gitea"

# Setup secondary Gitea remote
setup-gitea2 url:
    @echo "Adding secondary Gitea remote ({{GITEA_REMOTE_2}})..."
    git remote add {{GITEA_REMOTE_2}} {{url}}
    @echo "✅ {{GITEA_REMOTE_2}} remote added!"
    @echo "Test with: just push-gitea2"

# Sync primary Gitea with GitHub (force)
sync-gitea:
    @echo "Syncing {{GITEA_REMOTE}} with GitHub..."
    git push {{GITEA_REMOTE}} main --force
    git push {{GITEA_REMOTE}} --tags --force
    @echo "✅ {{GITEA_REMOTE}} synced!"

# Sync secondary Gitea with GitHub (force)
sync-gitea2:
    #!/usr/bin/env bash
    if git remote | grep -q "^{{GITEA_REMOTE_2}}$"; then
        echo "Syncing {{GITEA_REMOTE_2}} with GitHub..."
        git push {{GITEA_REMOTE_2}} main --force
        git push {{GITEA_REMOTE_2}} --tags --force
        echo "✅ {{GITEA_REMOTE_2}} synced!"
    else
        echo "⚠️  Remote {{GITEA_REMOTE_2}} not configured"
    fi

# Sync all Gitea instances with GitHub
sync-all-gitea:
    @just sync-gitea
    @just sync-gitea2

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Generate demo GIF for horizontal slider using VHS
vhs-horizontal:
    @echo "Running VHS tape to generate horizontal demo..."
    vhs examples/vhs/horizontal.tape
    @echo "✅ Demo generated at examples/vhs/target/horizontal.gif"

# Generate demo GIF for vertical slider using VHS
vhs-vertical:
    @echo "Running VHS tape to generate vertical demo..."
    vhs examples/vhs/vertical.tape
    @echo "✅ Demo generated at examples/vhs/target/vertical.gif"

# Generate demo GIF for custom slider using VHS
vhs-custom:
    @echo "Running VHS tape to generate custom demo..."
    vhs examples/vhs/custom.tape
    @echo "✅ Demo generated at examples/vhs/target/custom.gif"

# Generate demo GIF for thumb toggle using VHS
vhs-thumb:
    @echo "Running VHS tape to generate thumb toggle demo..."
    vhs examples/vhs/thumb_toggle.tape
    @echo "✅ Demo generated at examples/vhs/target/thumb_toggle.gif"

# Generate all demo GIFs
vhs-all: vhs-horizontal vhs-vertical vhs-custom vhs-thumb
    @echo "✅ All demo GIFs generated!"

# Clean generated VHS outputs
vhs-clean:
    rm -f examples/vhs/target/*.gif examples/vhs/target/*.mp4

# Run CI checks locally (same as GitHub Actions)
ci: check-all build-release examples
    @echo "✅ All CI checks passed!"

# Show help for release workflow
help-release:
    @echo "Release Workflow:"
    @echo ""
    @echo "1. Prepare and push release (all remotes):"
    @echo "   just release 0.2.0"
    @echo ""
    @echo "2. Publish to crates.io from configured target ({{PUBLISH_TARGET}}):"
    @echo "   just publish"
    @echo ""
    @echo "Or do everything in one step:"
    @echo "   just release-full 0.2.0"
    @echo ""
    @echo "Release to specific platform:"
    @echo "   just release-to github 0.2.0"
    @echo "   just release-to gitea 0.2.0"
    @echo ""
    @echo "Multiple Gitea instances:"
    @echo "   just setup-gitea <url>      # Primary Gitea"
    @echo "   just setup-gitea2 <url>     # Secondary Gitea"
    @echo "   just push-all               # Push to all"
    @echo "   just sync-all-gitea         # Sync all with GitHub"
    @echo ""
    @echo "Configure publish target by editing PUBLISH_TARGET in justfile"
    @echo "Preview unreleased changes:"
    @echo "   just changelog-preview-unreleased"

# Show Gitea configuration help
help-gitea:
    @echo "Gitea Multi-Instance Setup:"
    @echo ""
    @echo "Configuration (edit justfile):"
    @echo "  GITEA_REMOTE := \"gitea\"      # Primary Gitea"
    @echo "  GITEA_REMOTE_2 := \"gitea2\"    # Secondary Gitea (optional)"
    @echo "  PUBLISH_TARGET := \"github\"    # Where to publish from"
    @echo ""
    @echo "Setup remotes:"
    @echo "  just setup-gitea git@gitea1.com:user/repo.git"
    @echo "  just setup-gitea2 git@gitea2.com:user/repo.git"
    @echo ""
    @echo "Push commands:"
    @echo "  just push-gitea       # Push to primary Gitea"
    @echo "  just push-gitea2      # Push to secondary Gitea"
    @echo "  just push-all         # Push to all remotes"
    @echo ""
    @echo "Sync commands:"
    @echo "  just sync-gitea       # Sync primary with GitHub"
    @echo "  just sync-gitea2      # Sync secondary with GitHub"
    @echo "  just sync-all-gitea   # Sync all Gitea instances"
    @echo ""
    @echo "View configuration:"
    @echo "  just info             # Show current setup"
    @echo "  just remotes          # List all remotes"
