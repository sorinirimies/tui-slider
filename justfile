# List available commands
default:
    @just --list

# Run all tests
test:
    cargo test --all-features

# Run tests with coverage
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Check code formatting
fmt-check:
    cargo fmt --all -- --check

# Format code
fmt:
    cargo fmt --all

# Run clippy lints
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all checks (fmt, clippy, test)
check: fmt-check clippy test

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

# Run all examples
examples: example-horizontal example-vertical

# Generate documentation
doc:
    cargo doc --all-features --no-deps --open

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Generate changelog
changelog:
    git cliff -o CHANGELOG.md

# Publish to crates.io (dry-run)
publish-dry:
    cargo publish --dry-run

# Publish to crates.io
publish:
    cargo publish

# Watch and run tests on changes
watch-test:
    cargo watch -x test

# Watch and run specific example on changes
watch-example EXAMPLE:
    cargo watch -x "run --example {{EXAMPLE}}"

# Bump version (requires version number, e.g., just bump-version 0.2.0)
bump-version VERSION:
    ./scripts/bump_version.sh {{VERSION}}

# Generate demo GIF for horizontal slider using VHS
vhs-horizontal:
    vhs examples/vhs/horizontal.tape

# Generate demo GIF for vertical slider using VHS
vhs-vertical:
    vhs examples/vhs/vertical.tape

# Generate all demo GIFs
vhs-all: vhs-horizontal vhs-vertical

# Clean generated VHS outputs
vhs-clean:
    rm -f examples/vhs/target/*.gif examples/vhs/target/*.mp4
