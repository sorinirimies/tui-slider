# Justfile Best Practices & Patterns

This document describes the patterns and best practices used in this project's `justfile`.

## Table of Contents

- [Getting Started](#getting-started)
- [Fail Early Pattern](#fail-early-pattern)
- [Command Dependencies](#command-dependencies)
- [Naming Conventions](#naming-conventions)
- [Common Patterns](#common-patterns)

## Getting Started

### Automatic Setup

Run the interactive setup script to create or enhance your justfile:

```bash
./scripts/setup-just.sh
```

This script will:
- **Install `just`** if not already installed (via cargo or system package managers)
- **Create a new justfile** if one doesn't exist, with common commands for your project type
- **Enhance existing justfile** by adding missing useful commands (optional)
- **Install optional tools** like git-cliff for changelog generation
- **Set up shell completion** for better developer experience
- **Create backups** before modifying any files
- **Implement fail-early pattern** for version bumping and releases

The setup script detects your project type (Rust, Node.js, etc.) and creates appropriate commands automatically.

### Manual Creation

If you prefer to create a justfile manually, start with:

```just
# Default task - show available commands
default:
    @just --list
```

Then add commands specific to your project's needs.

## Fail Early Pattern

### Overview

The "fail early" pattern ensures that quality checks pass **before** making any permanent changes like version bumps or releases. This prevents creating broken releases and keeps the git history clean.

### Implementation

```just
# ❌ BAD: No checks before version bump
bump version:
    @./scripts/bump_version.sh {{version}}

# ✅ GOOD: Run all checks first
bump version: check-all check-git-cliff
    @./scripts/bump_version.sh {{version}}
```

### Why This Matters

1. **Prevents broken releases**: Code must pass all tests before tagging
2. **Keeps git history clean**: No need for "fix broken release" commits
3. **Saves time**: Fail fast instead of discovering issues after pushing
4. **Enforces quality**: Makes checks mandatory, not optional

### Dependency Chain

```
just bump 0.2.0
    ↓
check-all
    ↓
    ├── fmt-check  (verify code is formatted)
    ├── clippy     (check for lint issues)
    └── test       (run all tests)
    ↓
check-git-cliff (verify changelog tool is installed)
    ↓
bump_version.sh (actually bump the version)
```

If **any** step fails, the entire process stops immediately.

### Complete Example

```just
# Check if code is formatted
fmt-check:
    cargo fmt --check

# Run clippy linter
clippy:
    cargo clippy -- -D warnings

# Run tests
test:
    cargo test

# Run all checks (fail early pattern)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found"; exit 1; }

# Bump version (runs checks first)
bump version: check-all check-git-cliff
    @./scripts/bump_version.sh {{version}}

# Release (depends on bump, which depends on check-all)
release version: (bump version)
    git push origin main
    git push origin v{{version}}
```

## Command Dependencies

### Basic Syntax

Just uses dependencies to run commands in order:

```just
# command: dependency1 dependency2
release version: bump push
    @echo "Release complete!"
```

### Chaining Dependencies

Commands can depend on other commands that have their own dependencies:

```just
check-all: fmt-check clippy test

bump version: check-all check-git-cliff
    @./scripts/bump_version.sh {{version}}

release version: (bump version)
    git push origin main
    git push origin v{{version}}
```

Here `release` → `bump` → `check-all` → `fmt-check`, `clippy`, `test`

### Parameterized Dependencies

Use parentheses for dependencies that take parameters:

```just
# Pass the version parameter to bump
release version: (bump version)
    git push origin v{{version}}
```

## Naming Conventions

### Command Names

- Use **kebab-case**: `check-all`, `fmt-check`, `push-all`
- Be descriptive: `release-gitea` instead of `rel-g`
- Group related commands: `push`, `push-gitea`, `push-all`

### Parameter Names

- Use **snake_case** for parameters: `{{version}}`, `{{gitea_url}}`
- Make them self-documenting: `{{message}}` instead of `{{msg}}`

### Command Groups

Organize commands by prefix:

```just
# Building
build
build-release

# Testing
test
test-coverage

# Formatting
fmt
fmt-check

# Git operations
pull
pull-gitea
pull-all
push
push-gitea
push-all
```

## Common Patterns

### 1. Default Command

Always provide a helpful default:

```just
# Default task - show available commands
default:
    @just --list
```

### 2. Check Commands

Separate checking from fixing:

```just
# Format code
fmt:
    cargo fmt

# Check if code is formatted (read-only)
fmt-check:
    cargo fmt --check
```

### 3. Aggregate Commands

Create convenience commands that run multiple related tasks:

```just
# Run all checks
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"
```

### 4. Tool Verification

Check for required tools before using them:

```just
# Check if git-cliff is installed
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found"; exit 1; }

# Use the check as a dependency
changelog: check-git-cliff
    git-cliff -o CHANGELOG.md
```

### 5. Silent Commands

Use `@` prefix to hide command echo (cleaner output):

```just
# ❌ Shows: echo "Building..." then the message
version:
    echo "Version 0.1.0"

# ✅ Only shows: Version 0.1.0
version:
    @echo "Version 0.1.0"
```

### 6. Parameterized Commands

Document parameters in comments:

```just
# Bump version (usage: just bump 0.2.0)
bump version: check-all
    @./scripts/bump_version.sh {{version}}

# Commit with message (usage: just commit "Add feature")
commit message:
    git commit -m "{{message}}"
```

### 7. Multi-Remote Git Operations

Support multiple git remotes gracefully:

```just
# Pull from GitHub
pull:
    git pull origin main

# Pull from Gitea
pull-gitea:
    git pull gitea main

# Pull from both remotes
pull-all:
    git pull gitea main
    git pull origin main
    @echo "✅ Pulled from both remotes!"

# Push to GitHub
push:
    git push origin main

# Push to Gitea
push-gitea:
    git push gitea main

# Push to both remotes
push-all:
    git push origin main
    git push gitea main
    @echo "✅ Pushed to both remotes!"
```

### 8. Version Information

Display project metadata:

```just
# Show current version
version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

# Show project information
info:
    @echo "Project: $(grep '^name = ' Cargo.toml | head -1 | sed 's/name = "\(.*\)"/\1/')"
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = /Version: /' | tr -d '"'
```

## Release Workflow Pattern

A complete release workflow with fail-early checks:

```just
# 1. Check all code quality (fail early)
check-all: fmt-check clippy test
    @echo "✅ All checks passed!"

# 2. Verify tools are available
check-git-cliff:
    @command -v git-cliff >/dev/null 2>&1 || { echo "❌ git-cliff not found"; exit 1; }

# 3. Bump version (depends on checks)
bump version: check-all check-git-cliff
    @./scripts/bump_version.sh {{version}}

# 4. Release to GitHub (depends on bump)
release version: (bump version)
    git push origin main
    git push origin v{{version}}
    @echo "✅ Released v{{version}} to GitHub!"

# 5. Release to Gitea (depends on bump)
release-gitea version: (bump version)
    git push gitea main
    git push gitea v{{version}}
    @echo "✅ Released v{{version}} to Gitea!"

# 6. Release to all remotes (depends on bump)
release-all version: (bump version)
    git push origin main
    git push gitea main
    git push origin v{{version}}
    git push gitea v{{version}}
    @echo "✅ Released v{{version}} to all remotes!"
```

### Workflow Benefits

1. **Single command releases**: `just release-all 0.2.0`
2. **Automatic quality checks**: Can't release broken code
3. **Consistent process**: Same steps every time
4. **Clear dependencies**: Easy to understand what happens when

## Tips & Tricks

### Debugging

Show what a command will do without running it:

```bash
just --dry-run release 0.2.0
```

### Documentation

View the recipe for a command:

```bash
just --show release
```

### List Commands

```bash
just --list
# or simply
just
```

### Evaluate Variables

```bash
just --evaluate version
```

### Completion

Set up shell completion for better DX:

```bash
# Bash
mkdir -p ~/.local/share/bash-completion/completions
just --completions bash > ~/.local/share/bash-completion/completions/just

# Zsh
mkdir -p ~/.zsh/completion
just --completions zsh > ~/.zsh/completion/_just
# Add to ~/.zshrc: fpath=(~/.zsh/completion $fpath)

# Fish
just --completions fish > ~/.config/fish/completions/just.fish

# PowerShell
just --completions powershell > (Join-Path $PROFILE.CurrentUserAllHosts '..' Completions just-completion.ps1)
```

Or use the setup script: `./scripts/setup-just.sh`

## Project-Specific Commands

### Rust Projects

```just
# Build
build:
    cargo build

build-release:
    cargo build --release

# Test
test:
    cargo test

# Format
fmt:
    cargo fmt

fmt-check:
    cargo fmt --check

# Lint
clippy:
    cargo clippy -- -D warnings

# Documentation
doc:
    cargo doc --no-deps --open

# Clean
clean:
    cargo clean
```

### Node.js Projects

```just
# Install dependencies
install:
    npm install

# Build
build:
    npm run build

# Test
test:
    npm test

# Lint
lint:
    npm run lint

# Format
fmt:
    npm run format

fmt-check:
    npm run format:check
```

### Python Projects

```just
# Setup virtual environment
venv:
    python -m venv venv

# Install dependencies
install:
    pip install -r requirements.txt

# Test
test:
    pytest

# Lint
lint:
    ruff check .

# Format
fmt:
    black .

fmt-check:
    black --check .
```

## References

- [Official just documentation](https://just.systems/man/en/)
- [GitHub repository](https://github.com/casey/just)
- [Setup script](../scripts/setup-just.sh) - Automated installation and configuration

## Contributing

When adding new commands to the justfile:

1. ✅ Follow the naming conventions
2. ✅ Use the fail-early pattern for destructive operations
3. ✅ Add descriptive comments
4. ✅ Group related commands together
5. ✅ Test dependencies work correctly
6. ✅ Update this documentation if introducing new patterns

## Auto-Generation

The `setup-just.sh` script can automatically generate a justfile with common commands based on your project type. It will:

- Detect if your project is Rust, Node.js, Python, etc.
- Create appropriate build, test, and check commands
- Implement the fail-early pattern for version bumping
- Add git operations for multiple remotes
- Include documentation and info commands

Run `./scripts/setup-just.sh` to get started!