# tui-slider

[![Crates.io](https://img.shields.io/crates/v/tui-slider)](https://crates.io/crates/tui-slider)
[![Downloads](https://img.shields.io/crates/d/tui-slider)](https://crates.io/crates/tui-slider)
[![Documentation](https://docs.rs/tui-slider/badge.svg)](https://docs.rs/tui-slider)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-slider/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-slider/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-slider/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-slider/actions/workflows/ci.yml)

A highly customizable and configurable slider widget for [ratatui](https://github.com/ratatui-org/ratatui) that puts you in full control of every visual aspect.

Whether you're building music players, audio mixers, settings panels, or progress indicators, `tui-slider` adapts to your needs with extensive customization options. Configure colors, symbols, orientations, alignments, borders, and behavior—all through a clean, intuitive API. From minimalist progress bars to feature-rich interactive sliders, you decide exactly how your UI looks and feels.

## ✨ Features

- 🎚️ **[Horizontal](#horizontal-sliders) and [Vertical](#vertical-sliders) sliders** - Support for both orientations
- 🎨 **[Border styles](#border-styles)** - Multiple border style options with customizable symbols
- 🎯 **[Title alignment](#title-alignment)** - Left, center, and right title positioning
- 📊 **[Value alignment](#value-alignment)** - Flexible value display positioning
- 📍 **[Vertical positioning](#vertical-slider-positioning)** - Label and value positioning for vertical sliders
- 🎨 **[Progress bars](#progress-bars)** - Use as progress indicators without handles
- 🔧 **Easy to use** - Minimal configuration required
- 📊 **State management** - Built-in state for value tracking
- ⚡ **Lightweight** - No complex dependencies

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tui-slider = "0.1"
ratatui = "0.28"
```

## 🚀 Quick Start

```rust
use ratatui::prelude::*;
use tui_slider::{Slider, SliderState, SliderOrientation};

fn main() {
    // Create a slider state
    let mut state = SliderState::new(50.0, 0.0, 100.0);
    
    // Create and render a slider
    let slider = Slider::from_state(&state)
        .orientation(SliderOrientation::Horizontal)
        .label("Volume")
        .show_value(true);
    
    // In your render loop
    frame.render_widget(slider, area);
    
    // Update the value
    state.set_value(75.0);
}
```

## 📖 Examples

> 📚 **[Examples Source Code →](examples/)** - View all example implementations

### 🎯 Quick Navigation

- [Horizontal Sliders](#horizontal-sliders)
- [Vertical Sliders](#vertical-sliders)
- [Styling & Customization](#styling--customization)
- [Interactive Features](#interactive-features)
- [Layout & Positioning](#layout--positioning)

- [Complete Example Index](#-complete-example-index)

---

## Horizontal Sliders

### Horizontal Slider Styles
Comprehensive horizontal slider styles organized into two pages:
- **Page 1: Standard Styles** - Basic lines, blocks, gradients, progress bars (9 styles)
- **Page 2: Specialty Styles** - Segmented, dots, squares, stars (7 styles)

![Horizontal Demo](examples/vhs/output/horizontal.gif)

```rust
use tui_slider::{Slider, SliderState, SliderOrientation};

let state = SliderState::new(75.0, 0.0, 100.0);
let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Horizontal)
    .filled_symbol("━")
    .empty_symbol("─")
    .handle_symbol("●")
    .show_value(true);
```

🚀 `cargo run --example horizontal` | 📄 [View Source](examples/horizontal.rs)

_Use `n`/`PageDown` and `p`/`PageUp` to navigate between style pages._

---

## Vertical Sliders

### Basic Vertical Styles
Vertical slider orientation with various visual styles, perfect for mixer interfaces.

![Vertical Demo](examples/vhs/output/vertical.gif)

```rust
let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Vertical)
    .filled_symbol("█")
    .empty_symbol("░")
    .handle_symbol("▬");
```

🚀 `cargo run --example vertical` | 📄 [View Source](examples/vertical.rs)

---

### Vertical Style Variations
Various vertical slider styles optimized for different use cases.

![Vertical Styles Demo](examples/vhs/output/vertical.gif)

🚀 `cargo run --example vertical` | 📄 [View Source](examples/vertical.rs)

---

### Vertical Positioning
Layout strategies and positioning techniques for vertical sliders.

![Vertical Positioning Demo](examples/vhs/output/vertical_positioning.gif)

🚀 `cargo run --example vertical_positioning` | 📄 [View Source](examples/vertical_positioning.rs)

---

## Styling & Customization

### Custom Styles
Create your own slider styles with custom RGB colors and symbol combinations.

![Custom Styles Demo](examples/vhs/output/custom.gif)

```rust
use tui_slider::style::SliderStyle;

let style = SliderStyle::horizontal_thick();
let slider = Slider::from_state(&state)
    .filled_symbol(style.filled_symbol)
    .filled_color(style.filled_color);
```

🚀 `cargo run --example custom` | 📄 [View Source](examples/custom_symbols.rs)

---

### Borders
Border types, styles, and color theming - including Plain, Rounded, Double, Thick borders with full, segmented, and sides-only variations.

![Borders Demo](examples/vhs/output/borders.gif)

🚀 `cargo run --example borders` | 📄 [View Source](examples/borders.rs)

---

## Interactive Features

### Handle/Thumb Visibility
Side-by-side comparison of sliders WITH handles (interactive controls) vs WITHOUT handles (progress bars).
Shows the same sliders in both modes to clearly demonstrate the difference.

![Handles Demo](examples/vhs/output/handles.gif)

🚀 `cargo run --example handles` | 📄 [View Source](examples/handles.rs)

_Use `Tab` to switch between left (with handles) and right (without handles) sections._

---

### Step Sizes
Configurable step intervals for fine or coarse value adjustments.

![Step Sizes Demo](examples/vhs/output/step_sizes.gif)

🚀 `cargo run --example step_sizes` | 📄 [View Source](examples/step_sizes.rs)

---

## Layout & Positioning

### Title Alignment
Position slider titles left, center, or right.

![Title Alignment Demo](examples/vhs/output/title_alignment.gif)

🚀 `cargo run --example title_alignment` | 📄 [View Source](examples/title_alignment.rs)

---

### Value Alignment
Control value display positioning for optimal layout.

![Value Alignment Demo](examples/vhs/output/value_alignment.gif)

🚀 `cargo run --example value_alignment` | 📄 [View Source](examples/value_alignment.rs)

---

### Horizontal Bar Alignment
Position horizontal slider bars at the top, center, or bottom of the container.

![Horizontal Bar Alignment Demo](examples/vhs/output/horizontal_bar_alignment.gif)

🚀 `cargo run --example horizontal_bar_alignment` | 📄 [View Source](examples/horizontal_bar_alignment.rs)

---


## 📋 Complete Example Index

| Example | Category | Description | Demo | Source |
|---------|----------|-------------|------|--------|
| `horizontal.rs` | Horizontal | Comprehensive horizontal styles (2 pages: Standard & Specialty) | [GIF](examples/vhs/output/horizontal.gif) | [Code](examples/horizontal.rs) |
| `vertical.rs` | Vertical | Various vertical slider styles | [GIF](examples/vhs/output/vertical.gif) | [Code](examples/vertical.rs) |
| `vertical_positioning.rs` | Vertical | Vertical layout strategies | [GIF](examples/vhs/output/vertical_positioning.gif) | [Code](examples/vertical_positioning.rs) |
| `custom_symbols.rs` | Styling | Custom RGB colors and symbols | [GIF](examples/vhs/output/custom.gif) | [Code](examples/custom_symbols.rs) |
| `borders.rs` | Styling | Border types, styles, and colors | [GIF](examples/vhs/output/borders.gif) | [Code](examples/borders.rs) |
| `handles.rs` | Interactive | Handle visibility comparison (with/without) | [GIF](examples/vhs/output/handles.gif) | [Code](examples/handles.rs) |
| `step_sizes.rs` | Interactive | Configurable step intervals | [GIF](examples/vhs/output/step_sizes.gif) | [Code](examples/step_sizes.rs) |
| `title_alignment.rs` | Layout | Title positioning options | [GIF](examples/vhs/output/title_alignment.gif) | [Code](examples/title_alignment.rs) |
| `value_alignment.rs` | Layout | Value display positioning | [GIF](examples/vhs/output/value_alignment.gif) | [Code](examples/value_alignment.rs) |
| `horizontal_bar_alignment.rs` | Layout | Horizontal bar vertical positioning (top/center/bottom) | [GIF](examples/vhs/output/horizontal_bar_alignment.gif) | [Code](examples/horizontal_bar_alignment.rs) |


---

## 🎮 Common Controls

Most examples share similar keyboard controls:

- **↑/↓** or **j/k** - Navigate between sliders
- **←/→** or **h/l** - Adjust slider values
- **q** or **ESC** - Quit the example

Check each example's source code for specific controls and features.

## 🎮 Interactive Usage

```rust
use tui_slider::SliderState;

let mut state = SliderState::new(50.0, 0.0, 100.0);

// Increase/decrease by a step
state.increase(5.0);
state.decrease(5.0);

// Set exact value
state.set_value(75.0);

// Get current value
let current = state.value();

// Get as percentage (0.0 to 1.0)
let percentage = state.percentage();
```

## 🎯 API Overview

### Slider Widget

- `new(value, min, max)` - Create a new slider
- `from_state(&state)` - Create from a state
- `orientation(orientation)` - Set horizontal or vertical
- `label(text)` - Set label text
- `show_value(bool)` - Show/hide value display
- `show_handle(bool)` - Show/hide handle
- `filled_symbol(symbol)` - Set filled bar symbol
- `empty_symbol(symbol)` - Set empty bar symbol
- `handle_symbol(symbol)` - Set handle symbol
- `filled_color(color)` - Set filled bar color
- `empty_color(color)` - Set empty bar color
- `handle_color(color)` - Set handle color
- `show_handle(bool)` - Show/hide thumb indicator
- `show_thumb(bool)` - Alias for show_handle
- `vertical_label_position(position)` - Set label position for vertical sliders
- `vertical_value_position(position)` - Set value position for vertical sliders
- `vertical_value_alignment(alignment)` - Set value alignment for vertical sliders
- `block(block)` - Add border block

### SliderState

- `new(value, min, max)` - Create new state
- `value()` - Get current value
- `set_value(value)` - Set value
- `increase(step)` - Increase by step
- `decrease(step)` - Decrease by step
- `min()` / `max()` - Get bounds
- `set_min(min)` / `set_max(max)` - Set bounds
- `percentage()` - Get value as percentage (0.0-1.0)
- `set_percentage(percentage)` - Set from percentage



## 🏗️ Architecture

The library consists of three main components:

- **Slider** - The widget that renders the slider
- **SliderState** - Manages value, bounds, and state
- **SliderOrientation** - Horizontal or Vertical orientation

## 🛠️ Development

This project uses [just](https://github.com/casey/just) as a command runner for common development tasks.

### Quick Setup

Run the interactive setup script to install `just` and configure your environment:

```bash
./scripts/setup-just.sh
```

This script will:
- Install `just` command runner (if not already installed)
- Create a new justfile if one doesn't exist (with common commands)
- Enhance existing justfile with missing commands (optional)
- Install optional tools like `git-cliff` for changelog generation
- Set up shell completion
- Create backups before modifying files

### Manual Setup

If you prefer manual installation:

```bash
# Install just
cargo install just

# Install git-cliff (optional, for changelogs)
cargo install git-cliff

# View available commands
just --list
```

### Common Commands

```bash
just build              # Build the project
just test               # Run tests
just check-all          # Run all checks (fmt, clippy, test)
just run                # Run horizontal slider example
just examples           # Run all examples
just bump <version>     # Bump version (runs checks first)
```

For a complete list of available commands, run `just --list` or see the [justfile](justfile).

### Justfile Patterns

This project follows the "fail early" pattern for version bumps and releases:
- **`just bump <version>`** runs all checks (fmt, clippy, test) before bumping
- **`just release <version>`** depends on `bump`, ensuring quality before release
- All destructive operations have quality gates

See [Justfile Best Practices & Patterns](docs/JUSTFILE_PATTERNS.md) for detailed documentation.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Inspired by various TUI music players and audio applications

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.
