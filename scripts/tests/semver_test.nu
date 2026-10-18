#!/usr/bin/env nu
# Tests for scripts/semver.nu
use ../semver.nu [semver-cmp]

def main [] {
    let cases = [
        ["0.4.1" "0.4.2" -1]
        ["0.4.2" "0.4.1" 1]
        ["0.4.1" "0.4.1" 0]
        ["0.4.9" "0.4.10" -1]
        ["0.9.0" "0.10.0" -1]
        ["1.0.0" "0.99.99" 1]
        ["1.0.0-alpha" "1.0.0" -1]
        ["1.0.0" "1.0.0-rc.1" 1]
        ["1.0.0-alpha" "1.0.0-alpha.1" -1]
        ["1.0.0-alpha.1" "1.0.0-alpha.beta" -1]
        ["1.0.0-beta.2" "1.0.0-beta.11" -1]
        ["1.0.0-beta" "1.0.0-alpha" 1]
        ["1.0.0-rc.1" "1.0.0-rc.1" 0]
    ]
    mut failed = []
    for c in $cases {
        let got = (semver-cmp $c.0 $c.1)
        if $got != $c.2 { $failed = ($failed | append $"semver-cmp ($c.0) ($c.1): expected ($c.2) got ($got)") }
    }
    if ($failed | length) > 0 { error make { msg: ($failed | str join "\n") } }
    print $"ok: ($cases | length) semver cases"
}
