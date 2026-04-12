#!/usr/bin/env nu
# Generate all VHS demo tapes for tui-slider.
# Automatically discovers all .tape files in examples/vhs/ and generates GIFs.
# Usage: nu scripts/generate_all_tapes.nu

def main [] {
    let green = (ansi green)
    let red = (ansi red)
    let blue = (ansi blue)
    let cyan = (ansi cyan)
    let reset = (ansi reset)

    print $"($blue)🎬 Generating all VHS demo tapes...($reset)"
    print ""

    # Check that vhs is installed
    if (which vhs | length) == 0 {
        error make { msg: $"($red)❌ vhs is not installed($reset)\nInstall it with: brew install vhs" }
    }

    let tape_dir = "examples/vhs"
    let output_dir = "examples/vhs/output"

    # Verify the tape directory exists
    if not ($tape_dir | path exists) {
        error make { msg: $"($red)❌ ($tape_dir) directory not found($reset)\nRun this script from the project root." }
    }

    # Create output directory if it doesn't exist
    mkdir $output_dir

    # Discover all .tape files, sorted alphabetically
    let tapes = (glob $"($tape_dir)/*.tape" | sort)
    let total = ($tapes | length)

    if $total == 0 {
        error make { msg: $"($red)❌ No .tape files found in ($tape_dir)($reset)" }
    }

    print $"($blue)📼 Found ($total) tape\(s\) to generate($reset)"
    print ""

    mut succeeded = 0
    mut failed = 0
    mut failed_names = []

    for tape in ($tapes | enumerate) {
        let idx = ($tape.index + 1)
        let tape_path = $tape.item
        let tape_name = ($tape_path | path basename | str replace '.tape' '')

        print $"($blue)[($idx)/($total)]($reset) Generating ($cyan)($tape_name)($reset)..."

        let result = (do { run-external "vhs" ($tape_path | into string) } | complete)
        if $result.exit_code == 0 {
            $succeeded = $succeeded + 1
            print $"  ($green)✅ Successfully generated ($tape_name).gif($reset)"
        } else {
            $failed = $failed + 1
            $failed_names = ($failed_names | append $tape_name)
            print $"  ($red)❌ Failed to generate ($tape_name)($reset)"
            if ($result.stderr | str trim | is-not-empty) {
                print $"  ($red)   Error: ($result.stderr | str trim)($reset)"
            }
        }
        print ""
    }

    # Summary
    print $"($blue)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"
    print $"($blue)📊 Summary:($reset)"
    print $"   Total tapes: ($total)"
    print $"   ($green)Succeeded: ($succeeded)($reset)"
    if $failed > 0 {
        print $"   ($red)Failed: ($failed)($reset)"
        for name in $failed_names {
            print $"   ($red)  • ($name)($reset)"
        }
    }
    print $"($blue)━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━($reset)"

    if $failed == 0 {
        print ""
        print $"($green)🎉 All demo GIFs generated successfully!($reset)"
        print $"($blue)📁 Output directory: ($output_dir)($reset)"
    } else {
        print ""
        print $"($red)⚠️ Some tapes failed to generate. Review the errors above.($reset)"
        exit 1
    }
}
