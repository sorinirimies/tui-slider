# Examples

This directory contains examples demonstrating various features of the `tui-slider` library.

> 📖 **[View Full Examples Documentation →](../README.md#-examples)** - Complete guide with animated GIF demos!

## Running Examples

```bash
# Run any example
cargo run --example horizontal
cargo run --example vertical
cargo run --example custom
# ... etc
```

## Available Examples

All examples are documented in the [main README](../README.md#-examples) with:
- 🎬 Animated GIF demonstrations
- 📝 Code snippets and usage
- 🎯 Organized by category
- 🎮 Keyboard controls

### Quick Reference

| Example | Description |
|---------|-------------|
| `horizontal.rs` | Comprehensive horizontal styles (2 pages: Standard & Specialty) |
| `vertical.rs` | Various vertical slider styles |
| `vertical_positioning.rs` | Vertical layout strategies |
| `custom_symbols.rs` | Custom RGB colors and symbols |
| `borders.rs` | Border types, styles, and colors |
| `handles.rs` | Handle visibility comparison (with/without) |
| `step_sizes.rs` | Configurable step intervals |
| `title_alignment.rs` | Title positioning options |
| `value_alignment.rs` | Value display positioning |

## Common Controls

Most examples use similar keyboard controls:

- **↑/↓** or **j/k** - Navigate between sliders
- **←/→** or **h/l** - Adjust slider values
- **q** or **ESC** - Quit

## Regenerating Demos

All GIF demos are generated using [VHS](https://github.com/charmbracelet/vhs):

```bash
# Generate all demos automatically (discovers all .tape files)
just vhs-all

# Or run the script directly
nu scripts/generate_all_tapes.nu

# Or generate specific demos manually
just vhs-horizontal
just vhs-vertical
just vhs-borders
# ... etc
```

The `vhs-all` command automatically discovers all `.tape` files in `examples/vhs/` and generates their GIFs, so you don't need to manually list each tape when adding new examples.

See [`scripts/README.md`](../scripts/README.md#generate_all_tapesnu) for more details.

## License

All examples are released under the same MIT license as the main library.