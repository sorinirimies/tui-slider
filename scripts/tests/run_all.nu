#!/usr/bin/env nu
# Assert 'cargo run' only appears between Hide and Show in every tape.
def main [] {
    mut bad = []
    for t in (glob examples/vhs/*.tape) {
        mut hidden = false
        for l in (open $t | lines) {
            let s = ($l | str trim)
            if $s == "Hide" { $hidden = true } else if $s == "Show" { $hidden = false } else if ($s | str contains "cargo run") and (not $hidden) { $bad = ($bad | append $t) }
        }
    }
    if ($bad | length) > 0 { error make { msg: $"visible compile in: ($bad | uniq)" } }
    print "ok: no visible compile in tapes"
}
