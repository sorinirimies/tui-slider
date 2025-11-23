# Examples

This directory contains examples demonstrating various features of the `tui-slider` library.

> 🎨 **[View Visual Gallery →](EXAMPLES_GALLERY.md)** - Browse all examples with animated GIF demos!

All examples have animated GIF demos available in the `vhs/output/` directory, generated using [VHS](https://github.com/charmbracelet/vhs).

## Available Examples

| Example | Description | Demo |
|---------|-------------|------|
| `horizontal.rs` | Horizontal slider styles with different symbols and colors | [GIF](vhs/output/horizontal.gif) |
| `horizontal_styles.rs` | Extended horizontal style variations | [GIF](vhs/output/horizontal_styles.gif) |
| `vertical.rs` | Vertical sliders (mixer/equalizer style) | [GIF](vhs/output/vertical.gif) |
| `vertical_styles.rs` | Extended vertical style variations | [GIF](vhs/output/vertical_styles.gif) |
| `vertical_positioning.rs` | Vertical slider layout and positioning | [GIF](vhs/output/vertical_positioning.gif) |
| `custom.rs` | Custom styles with RGB colors | [GIF](vhs/output/custom.gif) |
| `border_colors.rs` | Colored border themes | [GIF](vhs/output/border_colors.gif) |
| `border_styles.rs` | Different border types | [GIF](vhs/output/border_styles.gif) |
| `handle_toggle.rs` | Toggle handle visibility | [GIF](vhs/output/handle_toggle.gif) |
| `thumb_toggle.rs` | Alternative handle toggle demo | [GIF](vhs/output/thumb_toggle.gif) |
| `step_sizes.rs` | Configurable step intervals | [GIF](vhs/output/step_sizes.gif) |
| `title_alignment.rs` | Title positioning (left/center/right) | [GIF](vhs/output/title_alignment.gif) |
| `value_alignment.rs` | Value display positioning | [GIF](vhs/output/value_alignment.gif) |
| `progress_bars.rs` | Progress bar styles | [GIF](vhs/output/progress_bars.gif) |
| `width_test.rs` | Width responsiveness testing | [GIF](vhs/output/width_test.gif) |

## Running Examples

```bash
# Run any example
cargo run --example horizontal
cargo run --example vertical
cargo run --example custom
# ... etc
```

## Common Controls

Most examples use similar keyboard controls:

- **↑/↓** or **j/k** - Navigate between sliders
- **←/→** or **h/l** - Adjust slider values
- **q** or **ESC** - Quit

Check each example's source code for specific controls and features.

## Quick Start

### With Handle (Interactive Controls)

```rust
use tui_slider::{Slider, SliderState, SliderOrientation};
use ratatui::style::Color;

let state = SliderState::new(75.0, 0.0, 100.0);
let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Horizontal)
    .filled_symbol("━")
    .empty_symbol("─")
    .handle_symbol("●")
    .filled_color(Color::Cyan)
    .show_handle(true);
```

### Without Handle (Progress Bars)

```rust
let progress = SliderState::new(65.0, 0.0, 100.0);
let bar = Slider::from_state(&progress)
    .filled_symbol("█")
    .empty_symbol("░")
    .filled_color(Color::Green)
    .show_handle(false);
```

## Regenerating Demos

All GIF demos can be regenerated using VHS:

```bash
# Generate all demos
just vhs-all

# Or generate specific demos
just vhs-horizontal
just vhs-vertical
# ... etc
```

See the [VHS README](vhs/README.md) for more details.

## License

All examples are released under the same MIT license as the main library.