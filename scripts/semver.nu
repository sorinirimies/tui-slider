# Semver precedence comparison (https://semver.org/#spec-item-11).
# Build metadata is not supported by the bump script's version regex.

# Compare two prerelease identifiers: numeric < alphanumeric, numerics by value.
def cmp-ident [a: string, b: string]: nothing -> int {
    let an = ($a =~ '^[0-9]+$')
    let bn = ($b =~ '^[0-9]+$')
    if $an and $bn {
        let x = ($a | into int)
        let y = ($b | into int)
        if $x < $y { -1 } else if $x > $y { 1 } else { 0 }
    } else if $an {
        -1
    } else if $bn {
        1
    } else if $a < $b { -1 } else if $a > $b { 1 } else { 0 }
}

# Returns -1 if a < b, 0 if equal, 1 if a > b.
export def semver-cmp [a: string, b: string]: nothing -> int {
    let pa = ($a | parse --regex '^(?P<core>[0-9]+\.[0-9]+\.[0-9]+)(?:-(?P<pre>.+))?$' | first)
    let pb = ($b | parse --regex '^(?P<core>[0-9]+\.[0-9]+\.[0-9]+)(?:-(?P<pre>.+))?$' | first)

    let ca = ($pa.core | split row "." | each {|x| $x | into int })
    let cb = ($pb.core | split row "." | each {|x| $x | into int })
    for i in 0..2 {
        let x = ($ca | get $i)
        let y = ($cb | get $i)
        if $x < $y { return (-1) }
        if $x > $y { return 1 }
    }

    # Same core: a version without prerelease outranks one with.
    let a_pre = ($pa.pre | is-empty)
    let b_pre = ($pb.pre | is-empty)
    if $a_pre and $b_pre { return 0 }
    if $a_pre { return 1 }
    if $b_pre { return (-1) }

    let ia = ($pa.pre | split row ".")
    let ib = ($pb.pre | split row ".")
    let n = ([($ia | length) ($ib | length)] | math min)
    for i in 0..<$n {
        let c = (cmp-ident ($ia | get $i) ($ib | get $i))
        if $c != 0 { return $c }
    }
    let la = ($ia | length)
    let lb = ($ib | length)
    if $la < $lb { -1 } else if $la > $lb { 1 } else { 0 }
}
