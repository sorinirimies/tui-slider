# tui-slider

[![Crates.io](https://img.shields.io/crates/v/tui-slider)](https://crates.io/crates/tui-slider)
[![Downloads](https://img.shields.io/crates/d/tui-slider)](https://crates.io/crates/tui-slider)
[![Documentation](https://docs.rs/tui-slider/badge.svg)](https://docs.rs/tui-slider)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Release](https://github.com/sorinirimies/tui-slider/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/tui-slider/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/tui-slider/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/tui-slider/actions/workflows/ci.yml)

A simple TUI slider component library for [ratatui](https://github.com/ratatui-org/ratatui).

Perfect for building music players, audio controls, equalizers, and any terminal UI that needs simple, clean sliders.

## ✨ Features

- 🎚️ **Horizontal and Vertical sliders** - Support for both orientations
- 🎨 **Simple customization** - Easily customize colors and symbols
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

### Horizontal Slider

```rust
use ratatui::style::Color;
use tui_slider::{Slider, SliderState, SliderOrientation};

let state = SliderState::new(75.0, 0.0, 100.0);

let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Horizontal)
    .label("Volume")
    .show_value(true)
    .filled_symbol("━")
    .empty_symbol("─")
    .handle_symbol("●")
    .filled_color(Color::Cyan)
    .empty_color(Color::DarkGray)
    .handle_color(Color::White);
```

### Vertical Slider

```rust
use ratatui::style::Color;
use tui_slider::{Slider, SliderState, SliderOrientation};

let state = SliderState::new(60.0, 0.0, 100.0);

let slider = Slider::from_state(&state)
    .orientation(SliderOrientation::Vertical)
    .label("Bass")
    .show_value(true)
    .filled_symbol("│")
    .empty_symbol("│")
    .handle_symbol("━")
    .filled_color(Color::Green)
    .empty_color(Color::DarkGray)
    .handle_color(Color::White);
```

### Custom Symbols

```rust
let slider = Slider::from_state(&state)
    .filled_symbol("█")
    .empty_symbol("░")
    .handle_symbol("▐")
    .filled_color(Color::Yellow)
    .show_handle(true);
```

### Toggle Thumb/Handle Visibility

```rust
// Show the thumb indicator (default)
let slider = Slider::from_state(&state)
    .show_thumb(true);

// Hide the thumb for a progress bar style
let slider = Slider::from_state(&state)
    .show_thumb(false);

// show_handle() is an alias for show_thumb()
let slider = Slider::from_state(&state)
    .show_handle(false);
```

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

## 🎨 Demos

Run the examples to see the sliders in action:

```bash
# Horizontal sliders with different styles
cargo run --example horizontal

# Vertical sliders (equalizer style)
cargo run --example vertical

# Custom slider configurations
cargo run --example custom

# Toggle thumb/handle visibility
cargo run --example thumb_toggle
```

## 🏗️ Architecture

The library consists of three main components:

- **Slider** - The widget that renders the slider
- **SliderState** - Manages value, bounds, and state
- **SliderOrientation** - Horizontal or Vertical orientation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Inspired by various TUI music players and audio applications

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes.