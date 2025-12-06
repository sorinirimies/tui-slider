# Scripts Directory

This directory contains automation scripts for the tui-slider project.

## Available Scripts

### `generate_all_tapes.sh`

Automated VHS tape generation script that discovers and generates all demo GIFs.

#### Usage

```bash
# Using just (recommended)
just vhs-all

# Direct script call
./scripts/generate_all_tapes.sh
```

#### What It Does

The script automatically:

1. **Discovers all .tape files** - Finds all `*.tape` files in `examples/vhs/`
2. **Validates VHS installation** - Checks if VHS is installed
3. **Creates output directory** - Ensures `examples/vhs/output/` exists
4. **Generates GIFs** - Runs `vhs` on each tape file
5. **Progress reporting** - Shows real-time progress with counters
6. **Summary statistics** - Reports success/failure counts

#### Features

- ✅ Automatic discovery - No need to manually list tapes
- ✅ Color-coded output for better readability
- ✅ Progress counter (e.g., [3/11])
- ✅ Success/failure tracking
- ✅ Alphabetically sorted processing
- ✅ Detailed error reporting
- ✅ Summary statistics at completion

#### Requirements

- **vhs** - VHS tape-to-GIF generator
  ```bash
  # macOS
  brew install vhs
  
  # Other platforms
  # See: https://github.com/charmbracelet/vhs
  ```

#### Output

All generated GIFs are saved to `examples/vhs/output/`:
- `horizontal.gif`
- `vertical.gif`
- `borders.gif`
- `custom_symbols.gif`
- And more...

#### Exit Codes

- `0` - All tapes generated successfully
- `1` - VHS not installed, no tapes found, or generation failures

#### Adding New Tapes

To add a new demo:

1. Create a new `.tape` file in `examples/vhs/`:
   ```bash
   touch examples/vhs/my_new_demo.tape
   ```

2. Define the tape content (see existing tapes for examples)

3. Run the generator:
   ```bash
   just vhs-all
   ```

The script will automatically discover and generate your new tape!

#### Example Output

```
🎬 Generating all VHS demo tapes...

📼 Found 11 tape(s) to generate

[1/11] Generating borders...
✅ Successfully generated borders.gif

[2/11] Generating custom_symbols...
✅ Successfully generated custom_symbols.gif

...

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 Summary:
   Total tapes: 11
   Succeeded: 11
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

🎉 All demo GIFs generated successfully!
📁 Output directory: examples/vhs/output
```

---

### `bump_version.sh`

Automated version bump script that handles the entire release preparation process.

#### Usage

```bash
# Using just (recommended)
just bump-version 0.2.0

# Direct script call
./scripts/bump_version.sh 0.2.0
```

#### What It Does

The script performs the following steps automatically:

1. **Validates version format** - Ensures semantic versioning (X.Y.Z or X.Y.Z-suffix)
2. **Updates Cargo.toml** - Changes version field
3. **Updates README.md** - Updates version badges
4. **Updates Cargo.lock** - Runs `cargo update`
5. **Formats code** - Runs `cargo fmt`
6. **Runs clippy** - Checks for linting issues
7. **Runs tests** - Ensures all tests pass
8. **Generates CHANGELOG.md** - Uses git-cliff if available
9. **Creates git commit** - Commits all changes
10. **Creates git tag** - Tags the release

#### Version Format

The script accepts semantic versioning formats:

- **Standard**: `0.2.0`, `1.0.0`, `2.1.3`
- **Pre-release**: `0.2.0-beta.1`, `1.0.0-rc.2`, `0.1.0-alpha`

#### Exit Codes

- `0` - Success
- `1` - Error (invalid version, tests failed, clippy failed, etc.)

#### Examples

```bash
# Bump to version 0.2.0
just bump-version 0.2.0

# Bump to beta version
just bump-version 0.3.0-beta.1

# Bump to release candidate
just bump-version 1.0.0-rc.1
```

#### After Running

Once the script completes successfully, you need to:

1. **Review changes**:
   ```bash
   git show
   git log -1
   ```

2. **Push to remote**:
   ```bash
   git push origin main
   git push origin v0.2.0
   ```

3. **Publish to crates.io**:
   ```bash
   cargo publish
   ```

4. **Create GitHub release**:
   - Go to GitHub releases page
   - Create new release from the tag
   - Copy changelog entries
   - Publish release

#### Requirements

- **git** - For version control operations
- **cargo** - For Rust project management
- **sed** - For text replacement (usually pre-installed)
- **git-cliff** (optional) - For changelog generation
  ```bash
  cargo install git-cliff
  ```

#### Troubleshooting

**Script fails on clippy**:
```bash
cargo clippy --fix
./scripts/bump_version.sh 0.2.0
```

**Script fails on tests**:
```bash
cargo test
# Fix failing tests
./scripts/bump_version.sh 0.2.0
```

**Tag already exists**:
```bash
# Delete existing tag
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0

# Run script again
./scripts/bump_version.sh 0.2.0
```

**Changelog not generated**:
```bash
# Install git-cliff
cargo install git-cliff

# Run script again
./scripts/bump_version.sh 0.2.0
```

#### Features

- ✅ Color-coded output for better readability
- ✅ Interactive confirmation before proceeding
- ✅ Comprehensive validation
- ✅ Automatic rollback on test failures
- ✅ Detailed progress reporting
- ✅ Post-completion instructions

#### Safety

The script includes several safety features:

- **Confirmation prompt** - Asks before making changes
- **Test validation** - Won't commit if tests fail
- **Clippy validation** - Won't commit if linting fails
- **Format check** - Ensures code is formatted
- **Version validation** - Prevents invalid versions

#### Integration with CI/CD

The script works seamlessly with the GitHub Actions workflows:

1. Run locally: `just bump-version 0.2.0`
2. Push: `git push origin main && git push origin v0.2.0`
3. CI automatically:
   - Runs tests on multiple platforms
   - Checks formatting and linting
   - Builds documentation
   - Creates GitHub release
   - Publishes to crates.io (on tag push)

## Future Scripts

Potential scripts that could be added:

- `generate_docs.sh` - Generate and preview documentation
- `run_benchmarks.sh` - Run performance benchmarks
- `update_deps.sh` - Update dependencies with testing
- `check_examples.sh` - Verify all examples compile and run

## Contributing

When adding new scripts:

1. Make them executable: `chmod +x scripts/your_script.sh`
2. Add shebang: `#!/bin/bash`
3. Include usage documentation
4. Add to this README
5. Add just command in justfile if appropriate
6. Follow the existing code style
7. Include error handling and validation

## Best Practices

Scripts in this directory should:

- ✅ Use `set -e` to exit on error
- ✅ Include color-coded output
- ✅ Provide clear error messages
- ✅ Validate all inputs
- ✅ Include usage examples
- ✅ Handle edge cases gracefully
- ✅ Be idempotent when possible
- ✅ Document all requirements

## Questions?

For issues with these scripts:

1. Check the troubleshooting section above
2. Review the script's source code
3. Open an issue on GitHub
4. Refer to the main project documentation

---

**Quick Reference**:
- Version bump: `just bump-version X.Y.Z`
- Generate all demos: `just vhs-all`
- Help: `./scripts/bump_version.sh` (shows usage)
- Justfile: See `justfile` for all available commands