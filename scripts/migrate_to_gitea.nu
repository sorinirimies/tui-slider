#!/usr/bin/env nu
# Migrate tui-slider to dual GitHub + Gitea hosting.
# Adds a Gitea remote, optionally pushes all history, and sets up .gitea/workflows.
#
# Usage:
#   nu scripts/migrate_to_gitea.nu
#   nu scripts/migrate_to_gitea.nu --project-dir /path/to/tui-slider
#   nu scripts/migrate_to_gitea.nu --gitea-url git@gitea.example.com:user/tui-slider.git
#   nu scripts/migrate_to_gitea.nu --project-dir . --gitea-url git@gitea.example.com:user/tui-slider.git

# ─── ANSI colour helpers ────────────────────────────────────────────────────

def success [msg: string] {
    print $"(ansi green)✅ ($msg)(ansi reset)"
}

def info [msg: string] {
    print $"(ansi blue)ℹ️  ($msg)(ansi reset)"
}

def warning [msg: string] {
    print $"(ansi yellow)⚠️  ($msg)(ansi reset)"
}

def heading [msg: string] {
    let bar = "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    print $"(ansi cyan)($bar)(ansi reset)"
    print $"(ansi cyan)  ($msg)(ansi reset)"
    print $"(ansi cyan)($bar)(ansi reset)"
}

def fail [msg: string] {
    print $"(ansi red)❌ Error: ($msg)(ansi reset)"
    exit 1
}

# ─── Main entry point ────────────────────────────────────────────────────────

def main [
    --project-dir: string = ""   # Path to the tui-slider project (default: current directory)
    --gitea-url: string   = ""   # Gitea repository URL (SSH or HTTPS). Prompted if omitted.
] {
    # ── Resolve project directory ────────────────────────────────────────────
    let project_dir = if ($project_dir | is-empty) {
        $env.PWD
    } else {
        $project_dir | path expand
    }

    let project_name = ($project_dir | path basename)

    # ── Preflight checks ────────────────────────────────────────────────────
    heading "Preflight Checks"

    # git installed?
    let git_check = (do { run-external "git" "--version" } | complete)
    if $git_check.exit_code != 0 {
        fail "git is not installed. Please install git first."
    }
    success $"git found: ($git_check.stdout | str trim)"

    # Inside a git repo?
    let repo_check = (do { run-external "git" "-C" $project_dir "rev-parse" "--git-dir" } | complete)
    if $repo_check.exit_code != 0 {
        fail $"Not a git repository: ($project_dir)"
    }
    success $"Git repository confirmed: ($project_dir)"

    # ── Resolve Gitea URL ────────────────────────────────────────────────────
    heading "Gitea Repository URL"

    let gitea_url = if ($gitea_url | is-empty) {
        print ""
        info "No --gitea-url provided. Please enter the Gitea repository URL."
        info $"Example \(SSH\):   git@gitea.example.com:username/($project_name).git"
        info $"Example \(HTTPS\): https://gitea.example.com/username/($project_name).git"
        print ""
        let url = (input "Gitea URL: " | str trim)
        if ($url | is-empty) {
            fail "Gitea URL is required."
        }
        $url
    } else {
        $gitea_url
    }

    # Determine SSH vs HTTPS
    let use_ssh = ($gitea_url | str starts-with "git@")

    let gitea_host = if $use_ssh {
        # git@host:user/repo.git → extract host
        $gitea_url | str replace --regex 'git@([^:]+):.*' '$1'
    } else {
        # https://host/user/repo.git → extract host
        $gitea_url | url parse | get host? | default ""
    }

    if (not $use_ssh) {
        warning "HTTPS URL detected. SSH is strongly recommended for passwordless pushes!"
    }

    print ""
    info $"Project:    ($project_name)"
    info $"Directory:  ($project_dir)"
    info $"Gitea URL:  ($gitea_url)"
    info $"Gitea Host: ($gitea_host)"
    info $"Protocol:   (if $use_ssh { 'SSH' } else { 'HTTPS' })"
    print ""

    # ── SSH key check (SSH only) ─────────────────────────────────────────────
    if $use_ssh {
        heading "Checking SSH Configuration"

        let key_paths = ["~/.ssh/id_ed25519" "~/.ssh/id_rsa" "~/.ssh/id_ecdsa"]
        let has_key = ($key_paths | any {|p| ($p | path expand | path exists) })

        if $has_key {
            success "SSH key found"

            info $"Testing SSH connection to ($gitea_host)..."
            let ssh_test = (do { run-external "ssh" "-o" "ConnectTimeout=5" "-T" $"git@($gitea_host)" } | complete)
            let ssh_output = $"($ssh_test.stdout)($ssh_test.stderr)"

            if ($ssh_output | str contains "successfully authenticated") or ($ssh_output | str contains "Hi there") {
                success "SSH connection successful!"
            } else {
                warning "Could not verify SSH connection."
                info "Make sure your SSH key is added to Gitea:"
                print "  1. Copy:  cat ~/.ssh/id_ed25519.pub"
                print "  2. Paste: Gitea → Settings → SSH/GPG Keys"
                print ""
            }
        } else {
            warning "No SSH keys found!"
            print ""
            info "Generate one with:"
            print "  ssh-keygen -t ed25519 -C \"your_email@example.com\""
            print ""
            let cont = (input "Continue anyway? (y/N) " | str trim | str downcase)
            if $cont != "y" {
                fail "SSH keys required. Please set up SSH first."
            }
        }
    }

    # ── Add or update gitea remote ───────────────────────────────────────────
    heading "Configuring Gitea Remote"

    let remotes = (do { run-external "git" "-C" $project_dir "remote" } | complete)
    let remote_list = ($remotes.stdout | lines)

    if ($remote_list | any {|r| $r == "gitea" }) {
        warning "Gitea remote already exists — updating URL..."
        let set_url = (do { run-external "git" "-C" $project_dir "remote" "set-url" "gitea" $gitea_url } | complete)
        if $set_url.exit_code != 0 {
            fail $"Failed to update gitea remote: ($set_url.stderr | str trim)"
        }
        success "Gitea remote URL updated"
    } else {
        let add_remote = (do { run-external "git" "-C" $project_dir "remote" "add" "gitea" $gitea_url } | complete)
        if $add_remote.exit_code != 0 {
            fail $"Failed to add gitea remote: ($add_remote.stderr | str trim)"
        }
        success "Gitea remote added"
    }

    # ── Show current remotes ─────────────────────────────────────────────────
    print ""
    info "Configured remotes:"
    let rv = (do { run-external "git" "-C" $project_dir "remote" "-v" } | complete)
    let relevant = ($rv.stdout | lines | where {|l| ($l | str starts-with "origin") or ($l | str starts-with "gitea") })
    for line in $relevant {
        print $"  ($line)"
    }
    print ""

    # ── Test Gitea connection ────────────────────────────────────────────────
    heading "Testing Gitea Connection"

    info "Testing connection to Gitea repository..."
    let ls_remote = (do { run-external "git" "-C" $project_dir "ls-remote" "gitea" } | complete)

    if $ls_remote.exit_code == 0 {
        success "Successfully connected to Gitea repository!"
    } else {
        warning "Could not connect to Gitea repository."
        print ""
        info "This is normal if the repository doesn't exist yet."
        print ""
        info "To create the repository on Gitea:"
        print "  1. Log in to your Gitea instance"
        print "  2. Click '+' → New Repository"
        print $"  3. Repository name: ($project_name)"
        print "  4. Do NOT initialize with README"
        print "  5. Click 'Create Repository'"
        print $"  6. Then run: git push gitea --all"
        print ""
    }

    # ── Optional: push all branches + tags ───────────────────────────────────
    heading "Push to Gitea"

    let push_answer = (input "Push all branches and tags to Gitea now? (y/N) " | str trim | str downcase)

    if $push_answer == "y" {
        info "Pushing all branches to Gitea..."
        let push_branches = (do { run-external "git" "-C" $project_dir "push" "gitea" "--all" } | complete)
        if $push_branches.exit_code == 0 {
            success "All branches pushed to Gitea"
        } else {
            warning $"Failed to push branches: ($push_branches.stderr | str trim)"
            info "The repository might not exist yet on the Gitea server."
        }

        info "Pushing all tags to Gitea..."
        let push_tags = (do { run-external "git" "-C" $project_dir "push" "gitea" "--tags" } | complete)
        if $push_tags.exit_code == 0 {
            success "All tags pushed to Gitea"
        } else {
            warning $"Failed to push tags: ($push_tags.stderr | str trim)"
        }
    } else {
        info "Skipping push. You can push later with:"
        print "  git push gitea --all && git push gitea --tags"
    }
    print ""

    # ── Optional: set up .gitea/workflows ────────────────────────────────────
    heading "Gitea Actions (CI/CD)"

    let gitea_dir = ($project_dir | path join ".gitea")
    let gitea_wf  = ($gitea_dir  | path join "workflows")
    let github_wf = ($project_dir | path join ".github" "workflows")

    if ($gitea_wf | path exists) {
        success ".gitea/workflows already exists — skipping."
        let wf_files = (ls $gitea_wf | get name)
        if ($wf_files | length) > 0 {
            info "Existing workflow files:"
            for f in $wf_files {
                print $"  • ($f | path basename)"
            }
        }
    } else {
        let setup_answer = (input "Set up .gitea/workflows from .github/workflows? (y/N) " | str trim | str downcase)

        if $setup_answer == "y" {
            mkdir $gitea_wf
            success "Created .gitea/workflows/"

            if ($github_wf | path exists) {
                let src_files = (ls $github_wf | where type == "file" | get name)
                if ($src_files | length) > 0 {
                    info "Copying workflows from .github/workflows → .gitea/workflows..."
                    for src in $src_files {
                        let basename = ($src | path basename)
                        let dest = ($gitea_wf | path join $basename)
                        cp $src $dest
                        success $"Copied ($basename)"
                    }
                    print ""
                    warning "Review the copied workflows — you may need to adjust runner labels,"
                    warning "action references, or secrets for your Gitea instance."
                } else {
                    info "No workflow files found in .github/workflows."
                    info "Add your Gitea Actions YAML files to .gitea/workflows/ manually."
                }
            } else {
                info "No .github/workflows directory found."
                info "Add your Gitea Actions YAML files to .gitea/workflows/ manually."
            }
        } else {
            info "Skipping .gitea/workflows setup."
        }
    }
    print ""

    # ── Summary ──────────────────────────────────────────────────────────────
    heading "Migration Complete! 🎉"
    print ""
    success $"tui-slider migrated to dual GitHub + Gitea hosting!"
    print ""

    info "What was done:"
    print $"  ✓ Gitea remote configured: ($gitea_url)"
    if ($gitea_wf | path exists) {
        print "  ✓ .gitea/workflows set up for Gitea Actions"
    }
    print ""

    info "Quick commands:"
    print "  git push origin main          # Push to GitHub only"
    print "  git push gitea main           # Push to Gitea only"
    print "  git push origin main && git push gitea main   # Push to both"
    print "  git push gitea --tags         # Push tags to Gitea"
    print ""

    info "Justfile commands (if configured):"
    print "  just push              # Push to GitHub"
    print "  just push-gitea        # Push to Gitea"
    print "  just push-all          # Push to both GitHub and Gitea"
    print "  just push-tags-all     # Push tags to both remotes"
    print "  just sync-gitea        # Force-sync Gitea with GitHub"
    print "  just remotes           # Show all configured remotes"
    print ""

    info "Release commands:"
    print "  just release 0.4.0          # Release to GitHub only"
    print "  just release-gitea 0.4.0    # Release to Gitea only"
    print "  just release-all 0.4.0      # Release to both remotes"
    print ""

    if $use_ssh {
        success "SSH configured — no passwords needed! 🔑"
    } else {
        warning "Using HTTPS — you will be prompted for credentials on each push."
        info "Switch to SSH for passwordless access:"
        print $"  git remote set-url gitea git@($gitea_host):username/tui-slider.git"
        print ""
    }

    print ""
    success "Happy dual-hosting! 🚀"
}
