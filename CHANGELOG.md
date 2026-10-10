# Changelog
All notable changes to this project will be documented in this file.

## [0.3.5] - 2026-10-10

### Bug Fixes
- Gate nightly-update has_changes on Cargo.lock only
- Drop deprecated str downcase; exit non-zero when bump is aborted

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

