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
| `vertical.rs` | Basic vertical slider styles |
| `vertical_styles.rs` | Extended vertical variations |
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
# Generate all demos
just vhs-all

# Or generate specific demos
just vhs-horizontal
just vhs-vertical
just vhs-borders
# ... etc
```

See the [VHS README](vhs/README.md) for more details.

## License

All examples are released under the same MIT license as the main library.