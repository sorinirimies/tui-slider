#!/usr/bin/env nu
# Nightly dependency upgrade script for tui-slider.
#
# Phases:
#   1. cargo upgrade --incompatible allow   (rewrite version pins in Cargo.toml)
#   2. cargo update                         (resolve fresh Cargo.lock)
#   3. Quality gate: fmt → clippy → tests
#   4. Commit strategy depending on what changed and whether the gate passed
#
# Usage:
#   nu scripts/upgrade_deps.nu
#   nu scripts/upgrade_deps.nu --dry-run
#   nu scripts/upgrade_deps.nu --bot-name "my-bot" --bot-email "bot@example.com"

# ── Helpers (top-level so they don't depend on main's scope) ──────────

# Check whether a tracked file has uncommitted changes relative to HEAD.
def is_dirty [path: string] {
    let result = (do { run-external "git" "diff" "--quiet" "HEAD" "--" $path } | complete)
    $result.exit_code != 0
}

# Build a short human-readable label from a list of file paths.
def commit_label [files: list] {
    $files | each {|f| $f | path basename } | str join " + "
}

# Return true only when every element in the list is true.
def all_passed [results: list] {
    $results | all {|r| $r }
}

# Run a single quality-gate check.  Prints a one-line pass / fail and
# returns a bool.  On failure the first 20 lines of output are shown.
def run_check [label: string, action: closure] {
    let green = (ansi green)
    let red   = (ansi red)
    let yellow = (ansi yellow)
    let reset = (ansi reset)

    print -n $"  ($label) ... "
    let result = (do $action | complete)
    if $result.exit_code == 0 {
        print $"($green)✓($reset)"
        return true
    }

    print $"($red)✗($reset)"
    # Combine stdout + stderr, take the first 20 lines as a preview.
    let output = (
        [$result.stdout $result.stderr]
        | where {|s| ($s | str trim | is-not-empty) }
        | str join "\n"
    )
    let lines = ($output | lines)
    let n = [($lines | length) 20] | math min
    let preview = ($lines | first $n | str join "\n")
    if ($preview | str trim | is-not-empty) {
        print $"($yellow)($preview)($reset)"
    }
    false
}

# Stage files, commit with the given message, and push.
# In dry-run mode only prints what *would* happen.
def do_commit_and_push [
    files: list,
    msg: string,
    branch: string,
    remote: string,
    dry_run: bool,
] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let reset  = (ansi reset)

    let label = (commit_label $files)

    if $dry_run {
        print $"($yellow)  [dry-run] would stage: ($label)($reset)"
        print $"($yellow)  [dry-run] would commit: ($msg)($reset)"
        print $"($yellow)  [dry-run] would push ($branch) → ($remote)($reset)"
        return
    }

    # Stage
    for f in $files { run-external "git" "add" $f }

    # Commit
    let commit_result = (do { run-external "git" "commit" "-m" $msg } | complete)
    if $commit_result.exit_code != 0 {
        print $"($red)  ✗ git commit failed($reset)"
        print $commit_result.stderr
        error make { msg: "git commit failed" }
    }
    print $"($green)  ✓ Committed: ($msg)($reset)"

    # Push
    print -n $"  Pushing ($branch) → ($remote) ... "
    let push_result = (do { run-external "git" "push" $remote $branch } | complete)
    if $push_result.exit_code != 0 {
        print $"($red)✗($reset)"
        print $push_result.stderr
        error make { msg: "git push failed" }
    }
    print $"($green)✓($reset)"
}

# ── Main ──────────────────────────────────────────────────────────────

def main [
    --bot-name: string  = "github-actions[bot]",                          # Git author / committer name
    --bot-email: string = "github-actions[bot]@users.noreply.github.com", # Git author / committer email
    --remote: string    = "origin",                                       # Git remote to push to
    --dry-run (-n),                                                       # Show what would be committed without pushing
] {
    # ── Colors ────────────────────────────────────────────────────────
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let cyan   = (ansi cyan)
    let bold   = (ansi attr_bold)
    let reset  = (ansi reset)

    # ── Banner ────────────────────────────────────────────────────────
    print ""
    print $"($cyan)($bold)╔══════════════════════════════════════════════════╗($reset)"
    print $"($cyan)($bold)║     🎚️  tui-slider — nightly dependency upgrade  ║($reset)"
    print $"($cyan)($bold)╚══════════════════════════════════════════════════╝($reset)"
    print ""

    if $dry_run {
        print $"($yellow)($bold)  ▶ DRY-RUN mode — nothing will be committed or pushed($reset)"
        print ""
    }

    # ── Pre-flight: ensure working tree is clean ──────────────────────
    print $"($cyan)── Pre-flight ──($reset)"
    let wt = (do { run-external "git" "status" "--porcelain" } | complete)
    if ($wt.stdout | str trim | is-not-empty) {
        print $"($red)  ✗ Working tree is dirty. Commit or stash changes first.($reset)"
        exit 1
    }
    print $"($green)  ✓ Working tree clean($reset)"
    print ""

    # Record current branch for the push at the end.
    let branch = ((do { run-external "git" "rev-parse" "--abbrev-ref" "HEAD" } | complete).stdout | str trim)

    # ── Phase 1: cargo upgrade ────────────────────────────────────────
    print $"($cyan)── Phase 1 · cargo upgrade ──($reset)"
    print -n "  cargo upgrade --incompatible allow ... "
    let upgrade = (do { run-external "cargo" "upgrade" "--incompatible" "allow" } | complete)
    if $upgrade.exit_code != 0 {
        print $"($red)✗($reset)"
        print $"($red)($upgrade.stderr)($reset)"
        error make { msg: "cargo upgrade failed" }
    }
    print $"($green)✓($reset)"
    print ""

    # ── Phase 2: cargo update ─────────────────────────────────────────
    print $"($cyan)── Phase 2 · cargo update ──($reset)"
    print -n "  cargo update ... "
    let update = (do { run-external "cargo" "update" } | complete)
    if $update.exit_code != 0 {
        print $"($red)✗($reset)"
        print $"($red)($update.stderr)($reset)"
        error make { msg: "cargo update failed" }
    }
    print $"($green)✓($reset)"
    print ""

    # ── Detect what changed ───────────────────────────────────────────
    let toml_changed = (is_dirty "Cargo.toml")
    let lock_changed = (is_dirty "Cargo.lock")

    let toml_status = if $toml_changed { $"($yellow)yes($reset)" } else { "no" }
    let lock_status = if $lock_changed { $"($yellow)yes($reset)" } else { "no" }

    print $"($cyan)── Changes detected ──($reset)"
    print $"  Cargo.toml changed: ($toml_status)"
    print $"  Cargo.lock changed: ($lock_status)"
    print ""

    if (not $toml_changed) and (not $lock_changed) {
        print $"($green)($bold)  ✓ All dependencies are already up-to-date. Nothing to do.($reset)"
        print ""
        exit 0
    }

    # ── Phase 3: Quality gate ─────────────────────────────────────────
    print $"($cyan)── Phase 3 · Quality gate ──($reset)"

    let fmt_ok = (run_check "cargo fmt --check" {
        run-external "cargo" "fmt" "--" "--check"
    })
    let clippy_ok = (run_check "cargo clippy" {
        run-external "cargo" "clippy" "--all-targets" "--all-features" "--" "-D" "warnings"
    })
    let test_ok = (run_check "cargo test" {
        run-external "cargo" "test" "--all-features" "--all-targets"
    })

    let gate_passed = (all_passed [$fmt_ok $clippy_ok $test_ok])
    print ""

    if $gate_passed {
        print $"($green)($bold)  ✓ Quality gate passed($reset)"
    } else {
        print $"($red)($bold)  ✗ Quality gate failed($reset)"
    }
    print ""

    # ── Phase 4: Commit strategy ──────────────────────────────────────
    print $"($cyan)── Phase 4 · Commit ──($reset)"

    # Configure git identity for the bot.
    run-external "git" "config" "user.name"  $bot_name
    run-external "git" "config" "user.email" $bot_email

    if $gate_passed and $toml_changed {
        # ── Gate PASSED + Cargo.toml changed → commit Cargo.toml + Cargo.lock
        let files = ["Cargo.toml" "Cargo.lock"]
        let msg = "chore(deps): upgrade dependencies (Cargo.toml + Cargo.lock)"

        print $"  Strategy: ($green)commit (commit_label $files)($reset)"
        do_commit_and_push $files $msg $branch $remote $dry_run

    } else if (not $gate_passed) and $toml_changed {
        # ── Gate FAILED + Cargo.toml changed → revert Cargo.toml, re-sync lock, commit lock only
        print $"  Strategy: ($yellow)revert Cargo.toml, commit Cargo.lock only($reset)"

        # Restore Cargo.toml to HEAD
        run-external "git" "checkout" "HEAD" "--" "Cargo.toml"
        print $"($yellow)  ↩ Cargo.toml reverted to HEAD($reset)"

        # Re-sync the lock file against the restored Cargo.toml
        print -n "  Re-syncing Cargo.lock ... "
        let resync = (do { run-external "cargo" "update" } | complete)
        if $resync.exit_code != 0 {
            print $"($red)✗($reset)"
            print $resync.stderr
            exit 1
        }
        print $"($green)✓($reset)"

        # Commit only the lock file if it still has changes
        if (is_dirty "Cargo.lock") {
            let msg = "chore(deps): update Cargo.lock (incompatible upgrades reverted)"
            do_commit_and_push ["Cargo.lock"] $msg $branch $remote $dry_run
        } else {
            print $"($yellow)  Cargo.lock unchanged after revert — nothing to commit($reset)"
        }

        # Always exit non-zero when the gate failed
        print ""
        print $"($red)($bold)  ✗ Exiting with failure \(quality gate did not pass\)($reset)"
        print ""
        exit 1

    } else if $gate_passed and (not $toml_changed) and $lock_changed {
        # ── Gate PASSED + only Cargo.lock changed → commit lock only
        let msg = "chore(deps): update Cargo.lock"

        print $"  Strategy: ($green)commit Cargo.lock only($reset)"
        do_commit_and_push ["Cargo.lock"] $msg $branch $remote $dry_run

    } else if (not $gate_passed) and (not $toml_changed) and $lock_changed {
        # ── Gate FAILED + only lock changed → commit lock (compatible updates are safe),
        #    but exit non-zero so CI notices
        let msg = "chore(deps): update Cargo.lock"

        print $"  Strategy: ($yellow)commit Cargo.lock only \(gate failed, compatible updates only\)($reset)"
        do_commit_and_push ["Cargo.lock"] $msg $branch $remote $dry_run

        print ""
        print $"($red)($bold)  ✗ Exiting with failure \(quality gate did not pass\)($reset)"
        print ""
        exit 1
    }

    # ── Summary ───────────────────────────────────────────────────────
    print ""
    print $"($cyan)($bold)══════════════════════════════════════════════════($reset)"
    if $gate_passed {
        print $"($green)($bold)  ✓ Dependency upgrade complete 🚀($reset)"
    }
    if $dry_run {
        print $"($yellow)    \(dry-run — no changes were pushed\)($reset)"
    }
    print $"($cyan)($bold)══════════════════════════════════════════════════($reset)"
    print ""
}
