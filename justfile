# tui-slider - A simple TUI slider component library for ratatui
# Install just: cargo install just
# Install git-cliff: cargo install git-cliff
# Usage: just <task>

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
    @echo "Pushing to remote..."
    @git push origin main
    @git push origin v{{version}}
    @echo "✅ Release v{{version}} pushed to remote!"

# Quick release check: format, check, test, and build
release-check: fmt clippy test build-release
    @echo "✅ Ready for release!"

# Publish to crates.io (dry run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Push release to remote
push-release:
    @echo "Pushing release to remote..."
    git push origin main
    git push origin --tags
    @echo "✅ Release pushed to remote!"

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
    @echo "Pushing to remote..."
    @just push-release
    @echo ""
    @echo "Publishing to crates.io..."
    @just publish
    @echo ""
    @echo "✅ Release v{{version}} complete!"

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

# Git: push to origin
push:
    git push origin main

# Git: push tags
push-tags:
    git push --tags

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
    @echo "Repository: https://github.com/sorinirimies/tui-slider"

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
    @echo "1. Prepare release (checks, version bump, changelog):"
    @echo "   just release 0.2.0"
    @echo ""
    @echo "2. Review changes and push:"
    @echo "   just push-release"
    @echo ""
    @echo "3. Publish to crates.io:"
    @echo "   just publish"
    @echo ""
    @echo "Or do everything in one step:"
    @echo "   just release-full 0.2.0"
    @echo ""
    @echo "Preview unreleased changes:"
    @echo "   just changelog-preview-unreleased"
