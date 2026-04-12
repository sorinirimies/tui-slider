#!/usr/bin/env nu
# Pre-publish readiness check for tui-slider.
# Usage: nu scripts/check_publish.nu

def main [] {
    let green = (ansi green)
    let red = (ansi red)
    let yellow = (ansi yellow)
    let cyan = (ansi cyan)
    let reset = (ansi reset)
    let bold = (ansi attr_bold)

    print ""
    print $"($cyan)($bold)╔══════════════════════════════════════════════════╗($reset)"
    print $"($cyan)($bold)║     🎚️  tui-slider — pre-publish check          ║($reset)"
    print $"($cyan)($bold)╚══════════════════════════════════════════════════╝($reset)"
    print ""

    mut errors = 0

    # ── Formatting ──────────────────────────────────────────────
    print $"($cyan)── Formatting ──($reset)"
    print -n $"  checking cargo fmt ... "
    let fmt = (do { cargo fmt -- --check } | complete)
    if $fmt.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    run `cargo fmt` to fix formatting($reset)"
        $errors += 1
    }
    print ""

    # ── Linting ─────────────────────────────────────────────────
    print $"($cyan)── Linting ──($reset)"
    print -n $"  checking clippy (all-targets, all-features) ... "
    let clip = (do { cargo clippy --all-targets --all-features -- -D warnings } | complete)
    if $clip.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    clippy reported warnings or errors — see output above($reset)"
        $errors += 1
    }
    print ""

    # ── Tests ───────────────────────────────────────────────────
    print $"($cyan)── Tests ──($reset)"
    print -n $"  running test suite (all-features, all-targets) ... "
    let test = (do { cargo test --all-features --all-targets } | complete)
    if $test.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    one or more tests failed($reset)"
        $errors += 1
    }
    print ""

    # ── Documentation ───────────────────────────────────────────
    print $"($cyan)── Documentation ──($reset)"
    print -n $"  building docs (no-deps, all-features) ... "
    let doc = (do { cargo doc --no-deps --all-features } | complete)
    if $doc.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    documentation build failed($reset)"
        $errors += 1
    }
    print ""

    # ── Examples ────────────────────────────────────────────────
    print $"($cyan)── Examples ──($reset)"
    print -n $"  building all examples ... "
    let examples = (do { cargo build --examples } | complete)
    if $examples.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    one or more examples failed to build($reset)"
        $errors += 1
    }
    print ""

    # ── Required Files ──────────────────────────────────────────
    print $"($cyan)── Required Files ──($reset)"
    let required_files = [
        "README.md"
        "LICENSE"
        "Cargo.toml"
        "CHANGELOG.md"
        "cliff.toml"
    ]
    for file in $required_files {
        print -n $"  checking ($file) ... "
        if ($file | path exists) {
            print $"($green)✓($reset)"
        } else {
            print $"($red)✗ missing($reset)"
            $errors += 1
        }
    }
    print ""

    # ── Cargo.lock ──────────────────────────────────────────────
    print $"($cyan)── Cargo.lock ──($reset)"
    print -n $"  checking Cargo.lock present ... "
    if ("Cargo.lock" | path exists) {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗ missing($reset)"
        print $"($yellow)    run `cargo generate-lockfile` to create it($reset)"
        $errors += 1
    }
    print ""

    # ── Publish Dry-Run ─────────────────────────────────────────
    print $"($cyan)── Publish Dry-Run ──($reset)"
    print -n $"  running cargo publish --dry-run ... "
    let dry = (do { cargo publish --dry-run --allow-dirty } | complete)
    if $dry.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        print $"($yellow)    dry-run failed — check packaging issues($reset)"
        $errors += 1
    }
    print ""

    # ── Summary ─────────────────────────────────────────────────
    print $"($cyan)($bold)──────────────────────────────────────────────────($reset)"
    if $errors == 0 {
        print $"($green)($bold)✓ All checks passed — ready to publish! 🚀($reset)"
        print $"($cyan)  Next step: just bump <version>($reset)"
    } else {
        let label = if $errors == 1 { "check" } else { "checks" }
        print $"($red)($bold)✗ ($errors) ($label) failed — please fix before publishing.($reset)"
        exit 1
    }
}
