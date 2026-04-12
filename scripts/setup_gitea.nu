#!/usr/bin/env nu
# Set up Gitea as a second remote for dual-hosting (GitHub + Gitea).
# Usage: nu scripts/setup_gitea.nu <gitea-url>
#
# Examples (SSH recommended):
#   nu scripts/setup_gitea.nu git@gitea.example.com:user/tui-slider.git
#   nu scripts/setup_gitea.nu https://gitea.example.com/user/tui-slider.git

def main [
    gitea_url: string,   # Gitea repository URL (SSH or HTTPS)
] {
    let green  = (ansi green)
    let red    = (ansi red)
    let yellow = (ansi yellow)
    let blue   = (ansi blue)
    let cyan   = (ansi cyan)
    let reset  = (ansi reset)
    let bold   = (ansi attr_bold)

    def success [msg: string] { print $"($green)✅ ($msg)($reset)" }
    def info    [msg: string] { print $"($blue)ℹ️  ($msg)($reset)" }
    def warning [msg: string] { print $"($yellow)⚠️  ($msg)($reset)" }
    def fail    [msg: string] { print $"($red)❌ ($msg)($reset)"; exit 1 }

    # ── Header ──────────────────────────────────────────────────
    print ""
    print $"($cyan)($bold)╔══════════════════════════════════════════════════╗($reset)"
    print $"($cyan)($bold)║     🎚️  tui-slider — Gitea Remote Setup         ║($reset)"
    print $"($cyan)($bold)╚══════════════════════════════════════════════════╝($reset)"
    print ""

    # ── Preflight checks ───────────────────────────────────────
    print $"($cyan)── Preflight ──($reset)"

    print -n "  checking git installed ... "
    let git_check = (do { git --version } | complete)
    if $git_check.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        fail "git is not installed. Please install git first."
    }

    print -n "  checking git repository ... "
    let repo_check = (do { git rev-parse --git-dir } | complete)
    if $repo_check.exit_code == 0 {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        fail "Not a git repository. Please run this script from the tui-slider directory."
    }

    print -n "  checking gitea URL format ... "
    if ($gitea_url | str ends-with ".git") or ($gitea_url | str starts-with "git@") or ($gitea_url | str starts-with "https://") or ($gitea_url | str starts-with "http://") {
        print $"($green)✓($reset)"
    } else {
        print $"($red)✗($reset)"
        warning $"URL doesn't look like a typical git remote: ($gitea_url)"
        info "Continuing anyway — git will validate the URL on first push."
    }

    print ""

    # ── Add or update gitea remote ─────────────────────────────
    print $"($cyan)── Remote Configuration ──($reset)"

    let existing_remotes = (do { git remote } | complete).stdout | str trim
    let has_gitea = ($existing_remotes | lines | any {|r| $r == "gitea" })

    if $has_gitea {
        print -n "  gitea remote exists, updating URL ... "
        let set_url = (do { git remote set-url gitea $gitea_url } | complete)
        if $set_url.exit_code == 0 {
            print $"($green)✓($reset)"
            success "Gitea remote URL updated"
        } else {
            print $"($red)✗($reset)"
            fail $"Failed to update gitea remote: ($set_url.stderr)"
        }
    } else {
        print -n "  adding gitea remote ... "
        let add_remote = (do { git remote add gitea $gitea_url } | complete)
        if $add_remote.exit_code == 0 {
            print $"($green)✓($reset)"
            success "Gitea remote added"
        } else {
            print $"($red)✗($reset)"
            fail $"Failed to add gitea remote: ($add_remote.stderr)"
        }
    }

    print ""

    # ── Show current remotes ───────────────────────────────────
    print $"($cyan)── Current Remotes ──($reset)"

    let remotes_output = (do { git remote -v } | complete).stdout | str trim
    let remote_lines = ($remotes_output | lines | where {|line| ($line | str starts-with "origin") or ($line | str starts-with "gitea") })
    for line in $remote_lines {
        print $"  ($line)"
    }

    print ""

    # ── Test connection ────────────────────────────────────────
    print $"($cyan)── Connection Test ──($reset)"

    print -n "  testing Gitea repository connection ... "
    let ls_remote = (do { git ls-remote gitea } | complete)
    if $ls_remote.exit_code == 0 {
        print $"($green)✓($reset)"
        success "Successfully connected to Gitea repository!"
    } else {
        print $"($yellow)⚠($reset)"
        warning "Could not connect to Gitea repository."
        print ""
        info "This is normal if the repository doesn't exist yet."
        print ""
        info "To create the repository on Gitea:"
        print "    1. Log in to your Gitea instance"
        print "    2. Click '+' → New Repository"
        print "    3. Repository name: tui-slider"
        print "    4. Do NOT initialize with README"
        print "    5. Click 'Create Repository'"
        print $"    6. Then run: ($cyan)just push-gitea($reset)"
    }

    print ""

    # ── Optional push ──────────────────────────────────────────
    print $"($cyan)── Initial Push ──($reset)"

    let answer = (input $"  Push all branches and tags to Gitea now? \(y/N\) ")
    if ($answer | str trim | str downcase) == "y" {
        print ""
        info "Pushing branches to Gitea..."

        let push_branches = (do { git push gitea --all } | complete)
        if $push_branches.exit_code == 0 {
            success "All branches pushed to Gitea"
        } else {
            warning "Failed to push branches. The repository might not exist yet."
            if ($push_branches.stderr | str length) > 0 {
                print $"  ($yellow)($push_branches.stderr | str trim)($reset)"
            }
        }

        info "Pushing tags to Gitea..."

        let push_tags = (do { git push gitea --tags } | complete)
        if $push_tags.exit_code == 0 {
            success "All tags pushed to Gitea"
        } else {
            warning "Failed to push tags."
            if ($push_tags.stderr | str length) > 0 {
                print $"  ($yellow)($push_tags.stderr | str trim)($reset)"
            }
        }
    } else {
        info "Skipped — you can push later with: just push-gitea"
    }

    print ""

    # ── Summary ─────────────────────────────────────────────────
    print $"($cyan)($bold)══════════════════════════════════════════════════($reset)"
    success "Gitea dual-hosting setup complete!"
    print $"($cyan)($bold)══════════════════════════════════════════════════($reset)"
    print ""

    info "Quick commands:"
    print $"  ($cyan)just push-gitea($reset)       Push main to Gitea"
    print $"  ($cyan)just push-all($reset)         Push main to both remotes"
    print $"  ($cyan)just push-tags-all($reset)    Push tags to both remotes"
    print $"  ($cyan)just sync-gitea($reset)       Force-sync Gitea with GitHub"
    print $"  ($cyan)just release-all <v>($reset)  Release to both remotes"
    print $"  ($cyan)just remotes($reset)          Show configured remotes"
    print ""

    info "Documentation:"
    print "  • Justfile commands: just --list"
    print "  • Scripts README:   scripts/README.md"
    print ""

    success "Happy coding! 🚀"
}
