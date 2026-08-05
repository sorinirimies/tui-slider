#!/usr/bin/env nu
# Automated version bump script for tui-slider
# Usage: nu scripts/bump_version.nu [--yes] <new_version>
# Example: nu scripts/bump_version.nu 0.4.0
# Example: nu scripts/bump_version.nu --yes 0.4.0

def main [
    new_version: string,  # Target version in semver format (e.g., 0.4.0)
    --yes (-y),           # Skip confirmation prompt
] {
    # ── Colors ────────────────────────────────────────────────────────
    let red = (ansi red)
    let green = (ansi green)
    let yellow = (ansi yellow)
    let cyan = (ansi cyan)
    let bold = (ansi attr_bold)
    let reset = (ansi reset)

    # ── Validate version format ───────────────────────────────────────
    if not ($new_version =~ '^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$') {
        error make {
            msg: $"($red)Error: Invalid version format '($new_version)'($reset)\nVersion must be X.Y.Z or X.Y.Z-suffix \(e.g., 0.4.0 or 0.4.0-beta.1\)"
        }
    }

    # ── Header ────────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($cyan)  tui-slider Version Bump($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""

    # ── Read current version from Cargo.toml ──────────────────────────
    let cargo_content = (open Cargo.toml --raw)
    let current_version = (
        $cargo_content
        | lines
        | where {|l| $l =~ '^version\s*=' }
        | first
        | parse --regex 'version\s*=\s*"(?P<v>[^"]+)"'
        | get v
        | first
    )

    print $"  Current version: ($yellow)($current_version)($reset)"
    print $"  New version:     ($green)($new_version)($reset)"
    print ""

    # ── Guard: already at requested version ───────────────────────────
    if $current_version == $new_version {
        error make {
            msg: $"($red)Error: Already at version ($new_version)($reset)\nNothing to do."
        }
    }

    # ── Guard: tag already exists ─────────────────────────────────────
    let tag_name = $"v($new_version)"
    let existing_tags = (do { run-external "git" "tag" } | complete)
    if $existing_tags.exit_code != 0 {
        error make { msg: $"($red)Error: Failed to list git tags. Is this a git repository?($reset)" }
    }
    let tags = ($existing_tags.stdout | lines)
    if ($tags | any {|t| $t == $tag_name }) {
        error make {
            msg: $"($red)Error: Tag '($tag_name)' already exists($reset)\nDelete it first with: git tag -d ($tag_name)"
        }
    }

    # ── Confirmation ──────────────────────────────────────────────────
    if not $yes {
        let answer = (input $"($bold)Continue with version bump ($current_version) → ($new_version)? \(y/n\) ($reset)")
        if ($answer | str trim | str downcase) != "y" {
            print $"($yellow)Aborted.($reset)"
            return
        }
        print ""
    }

    # ── Step 1/8: Update Cargo.toml ───────────────────────────────────
    print $"($cyan)Step 1/8: Updating Cargo.toml...($reset)"
    let updated_cargo = (
        $cargo_content
        | str replace --regex '(?m)^version\s*=\s*"[^"]+"' $'version = "($new_version)"'
    )
    $updated_cargo | save --force Cargo.toml
    print $"($green)  ✓ Cargo.toml updated($reset)"

    # ── Step 2/8: Update README.md badges ─────────────────────────────
    print ""
    print $"($cyan)Step 2/8: Updating README.md badges...($reset)"
    if ("README.md" | path exists) {
        let readme_content = (open README.md --raw)
        if ($readme_content =~ 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?-blue') {
            let updated_readme = (
                $readme_content
                | str replace --all --regex 'version-[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?-blue' $"version-($new_version)-blue"
            )
            $updated_readme | save --force README.md
            print $"($green)  ✓ README.md badges updated($reset)"
        } else {
            print $"($yellow)  ⚠ No version badges found in README.md($reset)"
        }
    } else {
        print $"($yellow)  ⚠ README.md not found, skipping($reset)"
    }

    # ── Step 3/8: Update Cargo.lock ───────────────────────────────────
    print ""
    print $"($cyan)Step 3/8: Updating Cargo.lock...($reset)"
    let lock_result = (do { run-external "cargo" "update" "-p" "tui-slider" } | complete)
    if $lock_result.exit_code != 0 {
        print $"($red)  ✗ Failed to update Cargo.lock($reset)"
        print $lock_result.stderr
        error make { msg: $"($red)cargo update failed($reset)" }
    }
    print $"($green)  ✓ Cargo.lock updated($reset)"

    # ── Step 4/8: cargo fmt ───────────────────────────────────────────
    print ""
    print $"($cyan)Step 4/8: Running cargo fmt...($reset)"
    let fmt_result = (do { run-external "cargo" "fmt" } | complete)
    if $fmt_result.exit_code != 0 {
        print $"($red)  ✗ cargo fmt failed($reset)"
        print $fmt_result.stderr
        error make { msg: $"($red)cargo fmt failed($reset)" }
    }
    print $"($green)  ✓ Code formatted($reset)"

    # ── Step 5/8: cargo clippy ────────────────────────────────────────
    print ""
    print $"($cyan)Step 5/8: Running cargo clippy...($reset)"
    let clippy_result = (do { run-external "cargo" "clippy" "--all-targets" "--all-features" "--" "-D" "warnings" } | complete)
    if $clippy_result.exit_code != 0 {
        print $"($red)  ✗ Clippy found issues. Please fix them before continuing.($reset)"
        print $clippy_result.stderr
        error make { msg: $"($red)cargo clippy failed with warnings or errors($reset)" }
    }
    print $"($green)  ✓ Clippy passed($reset)"

    # ── Step 6/8: cargo test ──────────────────────────────────────────
    print ""
    print $"($cyan)Step 6/8: Running tests...($reset)"
    let test_result = (do { run-external "cargo" "test" "--all-features" "--all-targets" } | complete)
    if $test_result.exit_code != 0 {
        print $"($red)  ✗ Tests failed. Please fix them before continuing.($reset)"
        print $test_result.stderr
        error make { msg: $"($red)cargo test failed($reset)" }
    }
    print $"($green)  ✓ All tests passed($reset)"

    # ── Step 7/8: Generate CHANGELOG.md ───────────────────────────────
    print ""
    print $"($cyan)Step 7/8: Generating CHANGELOG.md...($reset)"
    if (which git-cliff | length) > 0 {
        let cliff_result = (do { run-external "git-cliff" "--tag" $tag_name "-o" "CHANGELOG.md" } | complete)
        if $cliff_result.exit_code != 0 {
            print $"($yellow)  ⚠ git-cliff exited with errors, changelog may be incomplete($reset)"
            print $cliff_result.stderr
        } else {
            print $"($green)  ✓ CHANGELOG.md generated($reset)"
        }
    } else {
        print $"($yellow)  ⚠ git-cliff not found — skipping changelog generation($reset)"
        print $"($yellow)    Install it with: cargo install git-cliff($reset)"
    }

    # ── Step 8/8: Git commit + tag ────────────────────────────────────
    print ""
    print $"($cyan)Step 8/8: Creating git commit and tag...($reset)"

    # Collect files that exist for staging
    let files_to_stage = (
        ["Cargo.toml" "Cargo.lock" "README.md" "CHANGELOG.md"]
        | where {|f| ($f | path exists) }
    )

    # Check if there are uncommitted changes in our target files
    let diff_args = (["git" "diff" "--quiet"] | append $files_to_stage)
    let diff_result = (do { run-external ($diff_args | first) ...($diff_args | skip 1) } | complete)

    if $diff_result.exit_code != 0 {
        # There are changes to commit
        let add_args = (["git" "add"] | append $files_to_stage)
        run-external ($add_args | first) ...($add_args | skip 1)

        let commit_msg = ([
            $"chore: bump version to ($new_version)"
            ""
            "- Update version in Cargo.toml and README.md"
            "- Update Cargo.lock"
            "- Generate updated CHANGELOG.md"
        ] | str join "\n")

        let commit_result = (do { run-external "git" "commit" "-m" $commit_msg } | complete)
        if $commit_result.exit_code != 0 {
            print $"($red)  ✗ git commit failed($reset)"
            print $commit_result.stderr
            error make { msg: $"($red)git commit failed($reset)" }
        }
        print $"($green)  ✓ Changes committed($reset)"
    } else {
        print $"($yellow)  ⚠ No changes to commit($reset)"
    }

    # Create annotated tag
    let tag_msg = ([
        $"Release ($tag_name)"
        ""
        $"Includes all changes documented in CHANGELOG.md for version ($new_version)."
    ] | str join "\n")

    let tag_result = (do { run-external "git" "tag" "-a" $tag_name "-m" $tag_msg } | complete)
    if $tag_result.exit_code != 0 {
        print $"($red)  ✗ Failed to create tag ($tag_name)($reset)"
        print $tag_result.stderr
        error make { msg: $"($red)git tag failed($reset)" }
    }
    print $"($green)  ✓ Tag ($tag_name) created($reset)"

    # ── Summary ───────────────────────────────────────────────────────
    print ""
    print $"($cyan)════════════════════════════════════════($reset)"
    print $"($green)  ✓ Version bump complete! 🚀($reset)"
    print $"($cyan)════════════════════════════════════════($reset)"
    print ""
    print $"($yellow)Next steps:($reset)"
    print ""
    print $"  1. Review the changes:"
    print $"     ($cyan)git show($reset)"
    print $"     ($cyan)git log --oneline -5($reset)"
    print ""
    print $"  2. Push to origin \(GitHub\):"
    print $"     ($cyan)git push origin main($reset)"
    print $"     ($cyan)git push origin ($tag_name)($reset)"
    print ""
    print $"  3. Push to gitea \(if configured\):"
    print $"     ($cyan)git push gitea main($reset)"
    print $"     ($cyan)git push gitea ($tag_name)($reset)"
    print ""
    print $"  4. Publish to crates.io:"
    print $"     ($cyan)cargo publish --dry-run($reset)"
    print $"     ($cyan)cargo publish($reset)"
    print ""
}
