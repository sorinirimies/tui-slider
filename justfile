# tui-slider - A simple TUI slider component library for ratatui
#
# Prerequisites: cargo install just && cargo install nu
# Usage: just <task> or just --list
# Patterns: See docs/JUSTFILE_PATTERNS.md for best practices

# Default task - show available commands
default:
    @just --list

# Install required tools (just, git-cliff)
install-tools:
    @echo "Installing required tools..."
    @command -v just >/dev/null 2>&1 || cargo install just
    @command -v git-cliff >/dev/null 2>&1 || cargo install git-cliff
    @echo "✅ All tools installed!"

# Build the project
build:
    cargo build

# Build release version
build-release:
    cargo build --release

# Run horizontal slider example
run:
    cargo run --example horizontal

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

# Run tests
test:
    cargo test

# Run tests with coverage
test-coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

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
# Note: Runs check-all first to ensure code quality before version bump (fail early)
bump version: check-all check-git-cliff
    @echo "Bumping version to {{version}}..."
    nu scripts/bump_version.nu {{version}}

# Quick release: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Run pre-publish readiness checks
check-publish:
    nu scripts/check_publish.nu

# Upgrade dependencies (nightly-style: upgrade pins, sync lock, quality gate)
upgrade-deps:
    nu scripts/upgrade_deps.nu --dry-run

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated

# Generate documentation
doc:
    cargo doc --no-deps --open

# Watch and auto-run on file changes (requires cargo-watch)
watch:
    cargo watch -x "run --example horizontal"

# Git: commit current changes
commit message:
    git add .
    git commit -m "{{message}}"

# Git: pull from GitHub (origin)
pull:
    git pull origin main

# Git: pull from Gitea Microlab
pull-gitea-microlab:
    git pull gitea-microlab main

# Git: pull from Gitea (nexus-lab instance)
pull-gitea-nexus-lab:
    git pull gitea-nexus-lab main

# Git: pull from Gitea Starscream
pull-gitea-starscream:
    git pull gitea-starscream main

# Git: pull from all remotes (Gitea first, then GitHub)
pull-all:
    git pull gitea-microlab main
    git pull gitea-starscream main
    git pull gitea-nexus-lab main
    git pull origin main
    @echo "✅ Pulled from Gitea Microlab, Gitea Starscream, Gitea (nexus-lab), and GitHub!"

# Git: push to GitHub (origin)
push:
    git push origin main

# Git: push to Gitea Microlab
push-gitea-microlab:
    git push gitea-microlab main

# Git: push to Gitea (nexus-lab instance)
push-gitea-nexus-lab:
    git push gitea-nexus-lab main

# Git: push to Gitea Starscream
push-gitea-starscream:
    git push gitea-starscream main

# Git: push to GitHub, Gitea, and Gitea (nexus-lab)
push-all:
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    @echo "✅ Pushed to GitHub, Gitea Microlab, Gitea Starscream, and Gitea (nexus-lab)!"

# Git: push tags to GitHub
push-tags:
    git push origin --tags

# Git: push tags to all remotes
push-tags-all:
    git push origin --tags
    git push gitea-microlab --tags
    git push gitea-starscream --tags
    git push gitea-nexus-lab --tags
    @echo "✅ Tags pushed to GitHub, Gitea Microlab, Gitea Starscream, and Gitea (nexus-lab)!"

# Full release workflow: bump version and push to GitHub
release version: (bump version)
    @echo "Pushing to GitHub..."
    git push origin main
    git push origin v{{version}}
    @echo "✅ Release v{{version}} complete on GitHub!"

# Full release workflow: bump version and push to Gitea Microlab
release-gitea-microlab version: (bump version)
    @echo "Pushing to Gitea Microlab..."
    git push gitea-microlab main
    git push gitea-microlab v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea Microlab!"

# Full release workflow: bump version and push to Gitea (nexus-lab instance)
release-gitea-nexus-lab version: (bump version)
    @echo "Pushing to Gitea (nexus-lab)..."
    git push gitea-nexus-lab main
    git push gitea-nexus-lab v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea (nexus-lab)!"

# Full release workflow: bump version and push to Gitea Starscream
release-gitea-starscream version: (bump version)
    @echo "Pushing to Gitea Starscream..."
    git push gitea-starscream main
    git push gitea-starscream v{{version}}
    @echo "✅ Release v{{version}} complete on Gitea Starscream!"

# Full release workflow: bump version and push to GitHub, Gitea, and Gitea (nexus-lab)
release-all version: (bump version)
    @echo "Pushing to all remotes..."
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    git push origin v{{version}}
    git push gitea-microlab v{{version}}
    git push gitea-starscream v{{version}}
    git push gitea-nexus-lab v{{version}}
    @echo "✅ Release v{{version}} complete on all remotes!"

# Push release to all remotes (without bumping)
push-release-all:
    @echo "Pushing release to all remotes..."
    git push origin main
    git push gitea-microlab main
    git push gitea-starscream main
    git push gitea-nexus-lab main
    git push origin --tags
    git push gitea-microlab --tags
    git push gitea-starscream --tags
    git push gitea-nexus-lab --tags
    @echo "✅ Release pushed to all remotes!"

# Sync Gitea Microlab with GitHub (force)
sync-gitea-microlab:
    @echo "Syncing Gitea Microlab with GitHub..."
    git push gitea-microlab main --force
    git push gitea-microlab --tags --force
    @echo "✅ Gitea Microlab synced!"

# Sync Gitea Starscream with GitHub (force)
sync-gitea-starscream:
    @echo "Syncing Gitea Starscream with GitHub..."
    git push gitea-starscream main --force
    git push gitea-starscream --tags --force
    @echo "✅ Gitea Starscream synced!"

# Sync Gitea (nexus-lab instance) with GitHub (force)
sync-gitea-nexus-lab:
    @echo "Syncing Gitea (nexus-lab) with GitHub..."
    git push gitea-nexus-lab main --force
    git push gitea-nexus-lab --tags --force
    @echo "✅ Gitea (nexus-lab) synced!"

# Show configured remotes
remotes:
    @echo "Configured git remotes:"
    @git remote -v

# Setup Gitea remote (provide your Gitea URL)
setup-gitea url:
    @echo "Adding Gitea remote..."
    git remote add gitea-microlab {{url}}
    @echo "✅ Gitea remote added!"
    @echo "Test with: git push gitea-microlab main"

# Show current version
version:
    @nu scripts/version.nu

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
    @echo "Author: Sorin Albu-Irimies"
    @echo "License: MIT"

# View changelog
view-changelog:
    @cat CHANGELOG.md

# Run the VHS tape to generate demo GIF for horizontal slider
vhs-horizontal:
    @echo "Running VHS tape to generate horizontal demo..."
    vhs examples/vhs/horizontal.tape
    @echo "✅ Demo generated at examples/vhs/output/horizontal.gif"

# Run the VHS tape to generate demo GIF for vertical slider
vhs-vertical:
    @echo "Running VHS tape to generate vertical demo..."
    vhs examples/vhs/vertical.tape
    @echo "✅ Demo generated at examples/vhs/output/vertical.gif"

# Run the VHS tape to generate demo GIF for custom slider
vhs-custom:
    @echo "Running VHS tape to generate custom demo..."
    vhs examples/vhs/custom_symbols.tape
    @echo "✅ Demo generated at examples/vhs/output/custom.gif"

# Run the VHS tape to generate demo GIF for handles (with/without comparison)
vhs-handles:
    @echo "Running VHS tape to generate handles demo..."
    vhs examples/vhs/handles.tape
    @echo "✅ Demo generated at examples/vhs/output/handles.gif"

# Run the VHS tape to generate demo GIF for borders (styles and colors)
vhs-borders:
    @echo "Running VHS tape to generate borders demo..."
    vhs examples/vhs/borders.tape
    @echo "✅ Demo generated at examples/vhs/output/borders.gif"

# Run the VHS tape to generate the main tui-slider demo GIF (horizontal and vertical sliders showcase)
vhs-tui-slider:
    @echo "Running VHS tape to generate tui-slider demo..."
    vhs examples/vhs/tui_slider.tape
    @echo "✅ Demo generated at examples/vhs/output/tui-slider.gif"

# Run the VHS tape to generate demo GIF for step sizes
vhs-step-sizes:
    @echo "Running VHS tape to generate step sizes demo..."
    vhs examples/vhs/step_sizes.tape
    @echo "✅ Demo generated at examples/vhs/output/step_sizes.gif"

# Run the VHS tape to generate demo GIF for title alignment
vhs-title-alignment:
    @echo "Running VHS tape to generate title alignment demo..."
    vhs examples/vhs/title_alignment.tape
    @echo "✅ Demo generated at examples/vhs/output/title_alignment.gif"

# Run the VHS tape to generate demo GIF for value alignment
vhs-value-alignment:
    @echo "Running VHS tape to generate value alignment demo..."
    vhs examples/vhs/value_alignment.tape
    @echo "✅ Demo generated at examples/vhs/output/value_alignment.gif"

# Run the VHS tape to generate demo GIF for horizontal bar alignment
vhs-horizontal-bar-alignment:
    @echo "Running VHS tape to generate horizontal bar alignment demo..."
    vhs examples/vhs/horizontal_bar_alignment.tape
    @echo "✅ Demo generated at examples/vhs/output/horizontal_bar_alignment.gif"

# Run the VHS tape to generate demo GIF for vertical positioning
vhs-vertical-positioning:
    @echo "Running VHS tape to generate vertical positioning demo..."
    vhs examples/vhs/vertical_positioning.tape
    @echo "✅ Demo generated at examples/vhs/output/vertical_positioning.gif"



# Run all VHS tapes to generate all demo GIFs (automatically discovers all .tape files)
vhs-all:
    @echo "🎬 Running automated VHS tape generation..."
    nu scripts/generate_all_tapes.nu

# Run all VHS tapes manually (legacy - explicitly lists each tape)
vhs-all-manual: vhs-tui-slider vhs-horizontal vhs-vertical vhs-custom vhs-handles vhs-borders vhs-step-sizes vhs-title-alignment vhs-value-alignment vhs-vertical-positioning vhs-horizontal-bar-alignment
    @echo "✅ All demo GIFs generated!"
