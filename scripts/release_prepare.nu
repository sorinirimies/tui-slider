#!/usr/bin/env nu
# Prepare a release: update Cargo.toml version, regenerate CHANGELOG.md,
# and write RELEASE_NOTES.md with full release notes for GitHub/Gitea release body.
#
# This extracts the heavy release-note-generation logic that was previously
# inline in the GitHub/Gitea release workflow into a standalone, testable script.
#
# Usage: nu scripts/release_prepare.nu <tag>
# Example: nu scripts/release_prepare.nu v0.4.0

# Build the full RELEASE_NOTES.md content as a list of lines joined by newline.
def build_release_notes [
    version: string,
    last_tag: string,
    cliff_changes: string,
]: nothing -> string {
    mut lines = []

    # ── Title ───────────────────────────────────────────────────
    $lines = ($lines | append $"# tui-slider ($version)")
    $lines = ($lines | append "")

    # ── What's New ──────────────────────────────────────────────
    $lines = ($lines | append "## 🚀 What's New")
    $lines = ($lines | append "")

    if ($last_tag | is-empty) {
        $lines = ($lines | append "### 🎉 Initial Release")
    } else {
        $lines = ($lines | append $"### 📝 Changes since ($last_tag):")
    }
    $lines = ($lines | append "")

    # Append git-cliff generated changes
    $lines = ($lines | append $cliff_changes)
    $lines = ($lines | append "")

    # ── Installation ────────────────────────────────────────────
    $lines = ($lines | append "## 📦 Installation")
    $lines = ($lines | append "")
    $lines = ($lines | append "Add this to your `Cargo.toml`:")
    $lines = ($lines | append "")
    $lines = ($lines | append "```toml")
    $lines = ($lines | append "[dependencies]")
    $lines = ($lines | append $"tui-slider = \"($version)\"")
    $lines = ($lines | append "```")
    $lines = ($lines | append "")
    $lines = ($lines | append "Or install with cargo:")
    $lines = ($lines | append "")
    $lines = ($lines | append "```bash")
    $lines = ($lines | append "cargo add tui-slider")
    $lines = ($lines | append "```")
    $lines = ($lines | append "")

    # ── Quick Start ─────────────────────────────────────────────
    $lines = ($lines | append "## 🚀 Quick Start")
    $lines = ($lines | append "")
    $lines = ($lines | append "```rust")
    $lines = ($lines | append "use ratatui::prelude::*;")
    $lines = ($lines | append "use tui_slider::{Slider, SliderState, SliderOrientation};")
    $lines = ($lines | append "")
    $lines = ($lines | append "let mut state = SliderState::new(50.0, 0.0, 100.0);")
    $lines = ($lines | append "let slider = Slider::from_state(&state)")
    $lines = ($lines | append "    .orientation(SliderOrientation::Horizontal)")
    $lines = ($lines | append "    .label(\"Volume\")")
    $lines = ($lines | append "    .show_value(true);")
    $lines = ($lines | append "```")

    $lines | str join "\n"
}

def main [
    tag: string,   # Release tag, e.g. v0.4.0
] {
    let green  = (ansi green)
    let cyan   = (ansi cyan)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let reset  = (ansi reset)
    let bold   = (ansi attr_bold)

    # ── Banner ──────────────────────────────────────────────────
    print ""
    print $"($cyan)($bold)╔══════════════════════════════════════════════════╗($reset)"
    print $"($cyan)($bold)║     🎚️  tui-slider — release prepare             ║($reset)"
    print $"($cyan)($bold)╚══════════════════════════════════════════════════╝($reset)"
    print ""

    # ── Extract version ─────────────────────────────────────────
    let version = ($tag | str replace --regex '^v' '')
    print $"($cyan)Tag:     ($reset)($yellow)($tag)($reset)"
    print $"($cyan)Version: ($reset)($green)($version)($reset)"
    print ""

    # ── Step 1: Update Cargo.toml ───────────────────────────────
    print $"($cyan)Step 1/5:($reset) Updating Cargo.toml version..."

    let cargo_toml = (open Cargo.toml --raw)
    let updated_toml = ($cargo_toml | str replace --regex 'version = "[^"]+"' $'version = "($version)"')
    $updated_toml | save --force Cargo.toml

    print $"  ($green)✓($reset) Cargo.toml version set to ($green)($version)($reset)"
    print ""

    # Keep Cargo.lock's own package entry in sync so a later
    # `cargo test/build --locked` doesn't fail on a stale lockfile.
    let lock_sync = (do { cargo update -p tui-slider } | complete)
    if $lock_sync.exit_code != 0 {
        print $"  ($red)✗ Failed to sync Cargo.lock($reset)"
        print $"  ($red)($lock_sync.stderr)($reset)"
        exit 1
    }
    print $"  ($green)✓($reset) Cargo.lock synced"
    print ""

    # ── Step 2: Regenerate full CHANGELOG.md ────────────────────
    print $"($cyan)Step 2/5:($reset) Regenerating CHANGELOG.md with git-cliff..."

    let cliff_full = (do { git-cliff --config cliff.toml --tag $tag --output CHANGELOG.md } | complete)
    if $cliff_full.exit_code != 0 {
        print $"  ($red)✗ git-cliff failed to generate CHANGELOG.md($reset)"
        print $"  ($red)($cliff_full.stderr)($reset)"
        exit 1
    }

    print $"  ($green)✓($reset) CHANGELOG.md regenerated"
    print ""

    # ── Step 3: Generate diff changelog (CLIFF_CHANGES.md) ─────
    print $"($cyan)Step 3/5:($reset) Generating release diff changelog..."

    let last_tag_result = (do { git describe --tags --abbrev=0 "HEAD^" } | complete)
    let last_tag = if $last_tag_result.exit_code == 0 {
        $last_tag_result.stdout | str trim
    } else {
        ""
    }

    if ($last_tag | is-empty) {
        print $"  ($yellow)ℹ($reset) No previous tag found — treating as initial release"
        let cliff_diff = (do { git-cliff --config cliff.toml --tag $tag --strip header } | complete)
        if $cliff_diff.exit_code != 0 {
            print $"  ($red)✗ git-cliff failed to generate diff changelog($reset)"
            print $"  ($red)($cliff_diff.stderr)($reset)"
            exit 1
        }
        $cliff_diff.stdout | save --force CLIFF_CHANGES.md
    } else {
        print $"  ($cyan)Previous tag:($reset) ($yellow)($last_tag)($reset)"
        let range = $"($last_tag)..($tag)"
        let cliff_diff = (do { git-cliff --config cliff.toml $range --strip header } | complete)
        if $cliff_diff.exit_code != 0 {
            print $"  ($red)✗ git-cliff failed to generate diff changelog($reset)"
            print $"  ($red)($cliff_diff.stderr)($reset)"
            exit 1
        }
        $cliff_diff.stdout | save --force CLIFF_CHANGES.md
    }

    print $"  ($green)✓($reset) CLIFF_CHANGES.md generated"
    print ""

    # ── Step 4: Build RELEASE_NOTES.md ──────────────────────────
    print $"($cyan)Step 4/5:($reset) Building RELEASE_NOTES.md..."

    let cliff_changes = (open CLIFF_CHANGES.md --raw | str trim)
    let release_notes = (build_release_notes $version $last_tag $cliff_changes)
    $release_notes | save --force RELEASE_NOTES.md

    print $"  ($green)✓($reset) RELEASE_NOTES.md written"
    print ""

    # ── Step 5: Clean up temp files ─────────────────────────────
    print $"($cyan)Step 5/5:($reset) Cleaning up temporary files..."

    rm CLIFF_CHANGES.md
    print $"  ($green)✓($reset) Removed CLIFF_CHANGES.md"
    print ""

    # ── Summary ─────────────────────────────────────────────────
    print $"($cyan)($bold)──────────────────────────────────────────────────($reset)"
    print $"($green)($bold)✓ Release ($tag) prepared successfully! 🚀($reset)"
    print $"($cyan)($bold)──────────────────────────────────────────────────($reset)"
    print ""
    print $"($yellow)Artifacts:($reset)"
    print $"  • ($cyan)Cargo.toml($reset)      — version updated to ($green)($version)($reset)"
    print $"  • ($cyan)CHANGELOG.md($reset)    — full changelog regenerated"
    print $"  • ($cyan)RELEASE_NOTES.md($reset) — release body ready for GitHub/Gitea"
    print ""
    print $"($yellow)Next steps:($reset)"
    print $"  1. Review changes:  ($cyan)git diff($reset)"
    print $"  2. Commit:          ($cyan)git add Cargo.toml CHANGELOG.md RELEASE_NOTES.md && git commit -m \"chore: prepare release ($tag)\"($reset)"
    print $"  3. Tag:             ($cyan)git tag -a ($tag) -m \"Release ($tag)\"($reset)"
    print $"  4. Push:            ($cyan)git push origin main && git push origin ($tag)($reset)"
    print ""
}
