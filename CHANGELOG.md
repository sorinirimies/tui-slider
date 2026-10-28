# Changelog
All notable changes to this project will be documented in this file.

## [0.4.7] - 2026-10-28

### Miscellaneous
- Bump tui-nuances to 0.1.10

## [0.4.6] - 2026-10-26

### Miscellaneous
- Bump version to 0.4.6

## [0.4.5] - 2026-10-26

### Bug Fixes
- Avoid NaN in set_percentage for overflowing ranges (fuzz crash)

### Documentation
- Comprehensive tape cycles themes with theme-driven focus border

### Features
- Re-export ThemeSwitcher; use it and terminal downgrade in comprehensive example
- Focus-aware theming via Slider::focused; theme-driven focus border in example
- SliderStyle::theme keeps explicitly set colors in either call order
- Enable the tui-nuances theme feature by default

### Miscellaneous
- Bump tui-nuances to 0.1.7
- Update fuzz lockfile dependencies
- Drop unused import
- Bump version to 0.4.5

### Ci
- Nightly update pushes to main directly like tui-spinner (no manual-merge PR)
- Stop calling api.github.com (runner can't resolve it); pin/derive Nushell via github.com
- Run clippy directly (rs-clippy-check needs api.github.com)
- Automerge green PRs (workflow_run) + dependabot for actions
- Automerge major dependabot bumps too
- Nightly workflow-action bumps
- Bump clechasseur/rs-clippy-check from 4 to 5 (#5)
- Bump softprops/action-gh-release from 2 to 3 (#6)
- Bump actions/checkout from 4 to 7 (#3)

## [0.4.4] - 2026-10-18

### Miscellaneous
- Bump version to 0.4.4

### Ci
- Close stale nightly update PR when nothing to update

## [0.4.3] - 2026-10-18

### Bug Fixes
- Bump_version.nu rejects versions lower than current
- Center label/value on the bar, not the area

### Documentation
- Regenerate demo GIFs, refresh README
- Rewrite comprehensive demo as a studio mixer

### Features
- Vertical cell mapping, fix step float drift; chore(deps): update Cargo.lock
- Crossterm event helpers; fix percentage for tiny/overflowing ranges

### Miscellaneous
- Bump version to 0.4.3

### Ci
- Install nushell from release tarball (setup-nu needs Node 20)

## [0.4.2] - 2026-10-18

### Documentation
- Document set_from_cell/cell for mouse input

### Features
- Add set_from_cell/cell matching rendered handle position

### Miscellaneous
- Track Cargo.lock and ignore fuzz build/corpus output
- Bump version to 0.4.2

## [0.4.1] - 2026-10-18

### Bug Fixes
- Align horizontal bar fill with handle position (fixes #1)

### Miscellaneous
- Skip unconfigured remotes in *-all recipes
- Push/pull current branch instead of hardcoded main
- Bump version to 0.4.1

## [0.4.0] - 2026-10-14

### Documentation
- Re-record demos showing results only; reference tui-nuances version

### Features
- Shared theming via tui-nuances
- Make theming an optional "theme" feature

### Miscellaneous
- Add force-push recipes (push-all-force, per-remote, tags, sync-all-gitea)
- Depend on tui-nuances 0.1.5
- Depend on tui-nuances 0.1.6
- Depend on tui-nuances 0.1.6 from crates.io
- Bump version to 0.4.0

### Ci
- Add cargo-audit workflow and just audit recipe
- Prebuilt Nushell, cargo-deny + cargo-audit security workflow; add cargo-fuzz targets

### Deps
- Tui-nuances 0.1.4 (git)

## [0.3.5] - 2026-10-10

### Bug Fixes
- Gate nightly-update has_changes on Cargo.lock only
- Drop deprecated str downcase; exit non-zero when bump is aborted

### Miscellaneous
- Bump version to 0.3.5

### Ci
- Release only on real Cargo.toml dependency changes; lock-only updates merge without release

## [0.3.4] - 2026-09-07

### Bug Fixes
- Sync Cargo.lock tui-slider entry to 0.3.3
- Sync Cargo.lock inside release_prepare.nu
- Secrets context not allowed in step-level if: on auto-release.yml
- Harden slider rendering and validation

### Miscellaneous
- Bump version to 0.3.4

### Ci
- Automate nightly dependency patch releases

### Justfile
- Add gitea-nexus-lab remote recipes (push/pull/sync/release)
- Rename gitea -> gitea-microlab; add full gitea-microlab/-starscream/-nexus-lab parity

## [0.3.3] - 2026-08-05

### Bug Fixes
- Correct nu regex bugs breaking version bump and release prep

### Features
- Auto-release when dependency-update PR merges

### Miscellaneous
- Bump version to 0.3.3

## [0.3.2] - 2026-03-11

### Miscellaneous
- Add development rules and gitignore
- Bump version to 0.3.2

## [0.3.0] - 2026-02-20

### Miscellaneous
- Bump version to 0.3.0

## [0.2.8] - 2026-02-20

### Miscellaneous
- Bump version to 0.2.8

## [0.2.7] - 2026-02-20

### Miscellaneous
- Bump version to 0.2.6
- Bump version to 0.2.7

