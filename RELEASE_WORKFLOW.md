# Release Workflow Diagram

Visual guide to the tui-slider release process.

## 🚀 Quick Release (Recommended)

```
┌─────────────────────────────────────────────────────────────┐
│                    just release X.Y.Z                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  1. Run Pre-flight Checks             │
          │     ✓ cargo fmt --check               │
          │     ✓ cargo clippy                    │
          │     ✓ cargo test --all-features       │
          │     ✓ cargo build --release           │
          └───────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  2. Bump Version                      │
          │     • Update Cargo.toml               │
          │     • Update Cargo.lock               │
          └───────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  3. Generate Changelog                │
          │     • Run git-cliff                   │
          │     • Update CHANGELOG.md             │
          │     • Group by commit type            │
          └───────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  4. Create Git Commit & Tag           │
          │     • Commit changes                  │
          │     • Create annotated tag vX.Y.Z     │
          └───────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  5. Push to Remote                    │
          │     • Push main branch                │
          │     • Push version tag                │
          └───────────────────────────────────────┘
                              │
                              ▼
                      ✅ RELEASE READY!
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  6. Publish (Manual)                  │
          │     $ just publish                    │
          └───────────────────────────────────────┘
```

## 🔄 Complete Automated Release

```
┌─────────────────────────────────────────────────────────────┐
│                  just release-full X.Y.Z                    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  Runs: just release X.Y.Z             │
          │  (All steps above)                    │
          └───────────────────────────────────────┘
                              │
                              ▼
          ┌───────────────────────────────────────┐
          │  Auto-publish to crates.io            │
          │     • cargo publish                   │
          └───────────────────────────────────────┘
                              │
                              ▼
                  🎉 FULLY RELEASED!
```

## 📊 Release Timeline

```
Development                Release                 Published
    │                         │                         │
    │  Write code            │                         │
    │  Write tests           │                         │
    │  Update docs           │                         │
    │                        │                         │
    ▼                        │                         │
┌────────┐                  │                         │
│ Commit │                  │                         │
└────────┘                  │                         │
    │                        │                         │
    │  More commits...       │                         │
    │                        │                         │
    ▼                        │                         │
┌────────────────┐           │                         │
│ Ready to       │           │                         │
│ release?       │           │                         │
└────────────────┘           │                         │
    │                        │                         │
    │ YES                    │                         │
    ▼                        ▼                         │
┌──────────────────────────────────┐                  │
│  just release X.Y.Z              │                  │
│  (runs all checks, bumps,        │                  │
│   generates changelog, pushes)   │                  │
└──────────────────────────────────┘                  │
                               │                       │
                               ▼                       │
                      ┌────────────────┐               │
                      │ Review on      │               │
                      │ GitHub         │               │
                      └────────────────┘               │
                               │                       │
                               │ Looks good?           │
                               ▼                       ▼
                      ┌─────────────────────────────────┐
                      │  just publish                   │
                      │  (publish to crates.io)         │
                      └─────────────────────────────────┘
                                       │
                                       ▼
                              ✅ Live on crates.io!
```

## 🛠️ Command Reference

### Main Commands

| Command | Description | Pushes? | Publishes? |
|---------|-------------|---------|------------|
| `just release X.Y.Z` | Full release prep + push | ✅ Yes | ❌ No |
| `just release-full X.Y.Z` | Complete automation | ✅ Yes | ✅ Yes |
| `just publish` | Publish to crates.io | ❌ No | ✅ Yes |
| `just release-check` | Pre-flight checks only | ❌ No | ❌ No |

### Helper Commands

| Command | Description |
|---------|-------------|
| `just check-all` | Run fmt, clippy, tests |
| `just changelog-preview-unreleased` | Preview unreleased changes |
| `just push-release` | Push commits and tags (redundant, built into release) |
| `just version` | Show current version |
| `just ci` | Run full CI checks locally |

## 📝 Conventional Commits

The changelog is generated from commit messages. Use these prefixes:

```
feat: ✨      New features
fix: 🐛       Bug fixes
docs: 📚      Documentation
style: 💄     Code style (formatting)
refactor: ♻️  Code refactoring
perf: ⚡      Performance improvements
test: 🧪      Tests
chore: 🔧     Maintenance
ci: 👷        CI/CD changes
```

### Example Commits

```bash
git commit -m "feat: add show_thumb() method"
git commit -m "fix: correct slider rendering at boundaries"
git commit -m "docs: update README with new examples"
git commit -m "refactor: simplify state management"
git commit -m "test: add unit tests for SliderState"
git commit -m "chore(release): bump version to 0.2.0"
```

## 🔍 What Gets Generated

### Version Bump
```toml
# Cargo.toml
[package]
name = "tui-slider"
version = "0.2.0"  # ← Updated
```

### Changelog
```markdown
# CHANGELOG.md

## [0.2.0] - 2025-11-12

### Features
- Add show_thumb() method
- Add vertical slider support

### Bug Fixes
- Fix slider rendering at boundaries

### Documentation
- Update README with new examples
```

### Git Operations
```bash
# Commit created
chore(release): bump version to 0.2.0

# Tag created
v0.2.0

# Pushed to remote
origin/main
origin/tags/v0.2.0
```

## 🎯 Decision Tree

```
                    Ready to release?
                          │
                    ┌─────┴─────┐
                   YES          NO
                    │            │
                    │            └──► Continue development
                    │
                    ▼
            Need to review first?
                    │
            ┌───────┴────────┐
           YES              NO
            │                │
            │                └──► just release-full X.Y.Z
            │                         (fully automated)
            ▼
     just release X.Y.Z
     (auto-pushes)
            │
            ▼
     Review on GitHub
            │
      ┌─────┴──────┐
     OK          Issues?
      │              │
      │              └──► Fix issues, repeat
      ▼
   just publish
```

## 📋 Pre-Release Checklist

Before running `just release`:

- [ ] All features complete
- [ ] Tests passing locally (`just test`)
- [ ] Code formatted (`just fmt`)
- [ ] No clippy warnings (`just clippy`)
- [ ] Examples work (`just examples`)
- [ ] Documentation updated (README, docs)
- [ ] Changelog preview looks good (`just changelog-preview-unreleased`)
- [ ] Version number is correct (semver)

## 🚨 Emergency Rollback

If something goes wrong:

```bash
# 1. Delete the tag locally
git tag -d v0.2.0

# 2. Delete the tag from remote
git push origin :refs/tags/v0.2.0

# 3. Reset to previous commit
git reset --hard HEAD~1

# 4. Force push main (be careful!)
git push origin main --force

# 5. If already published to crates.io
# You can yank the version (makes it unavailable for new users)
cargo yank --vers 0.2.0
```

## 💡 Tips

1. **Always preview changelog first**
   ```bash
   just changelog-preview-unreleased
   ```

2. **Test publish before actual publish**
   ```bash
   just publish-dry
   ```

3. **Run CI checks locally before releasing**
   ```bash
   just ci
   ```

4. **Use conventional commits** for better changelogs

5. **Follow semantic versioning** (MAJOR.MINOR.PATCH)

## 🔗 Quick Links

- [RELEASE.md](./RELEASE.md) - Detailed documentation
- [RELEASE_QUICK_START.md](./RELEASE_QUICK_START.md) - Quick reference
- [justfile](./justfile) - All commands
- [cliff.toml](./cliff.toml) - Changelog configuration