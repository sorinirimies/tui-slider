# Examples

This directory contains examples demonstrating various features of the `tui-slider` library.

## 🎬 Demo Gallery

All examples have animated GIF demos available in the `vhs/target/` directory. The demos are generated using [VHS](https://github.com/charmbracelet/vhs).

## Available Examples

### Interactive Examples with Thumb Indicators

#### 1. Horizontal Sliders (`horizontal.rs`)
Demonstrates various horizontal slider styles with different symbols and colors.

![Horizontal Slider Demo](vhs/target/horizontal.gif)

```bash
cargo run --example horizontal
```

**Features:**
- 21 different slider styles
- Volume, Bass, Treble, and other audio-style controls
- Segmented slider styles
- Interactive value adjustment

**Controls:**
- `↑/↓` or `j/k`: Select slider
- `←/→` or `h/l`: Adjust value
- `q` or `ESC`: Quit

**Additional Demos:**
- [Horizontal Styles](vhs/target/horizontal_styles.gif) - Various horizontal slider styles
- [Border Styles](vhs/target/border_styles.gif) - Different border styles
- [Border Colors](vhs/target/border_colors.gif) - Colored borders
- [Title Alignment](vhs/target/title_alignment.gif) - Title positioning options
- [Value Alignment](vhs/target/value_alignment.gif) - Value positioning options

---

#### 2. Vertical Sliders (`vertical.rs`)
Shows vertical orientation with various styles.

![Vertical Slider Demo](vhs/target/vertical.gif)

```bash
cargo run --example vertical
```

**Features:**
- Vertical slider orientation
- Multiple visual styles
- Interactive controls

**Controls:**
- `←/→` or `h/l`: Select slider
- `↑/↓` or `j/k`: Adjust value
- `q` or `ESC`: Quit

**Additional Demos:**
- [Vertical Styles](vhs/target/vertical_styles.gif) - Various vertical slider styles
- [Vertical Positioning](vhs/target/vertical_positioning.gif) - Label and value positioning

---

#### 3. Custom Styles (`custom.rs`)
Showcases custom slider styles with RGB colors and creative symbol combinations.

![Custom Styles Demo](vhs/target/custom.gif)

```bash
cargo run --example custom
```

**Features:**
- Custom color combinations
- Creative symbol usage
- Segmented and non-segmented styles
- Named themes (Sunset, Ocean, Forest, etc.)

**Controls:**
- `↑/↓` or `j/k`: Select slider
- `←/→` or `h/l`: Adjust value
- `q` or `ESC`: Quit

---

#### 4. Handle Toggle (`handle_toggle.rs`)
Interactive demonstration of toggling the handle/thumb visibility.

![Handle Toggle Demo](vhs/target/handle_toggle.gif)

```bash
cargo run --example handle_toggle
```

**Features:**
- Toggle handle visibility on/off
- Compare sliders with and without handles
- Real-time switching

**Controls:**
- `↑/↓` or `j/k`: Select slider
- `←/→` or `h/l`: Adjust value
- `SHIFT+H` or `SPACE`: Toggle handle visibility
- `q` or `ESC`: Quit

---

#### 5. Thumb Toggle (`thumb_toggle.rs`)
Alternative demo for thumb/handle toggling functionality.

![Thumb Toggle Demo](vhs/target/thumb_toggle.gif)

```bash
cargo run --example thumb_toggle
```

---

#### 6. Step Sizes (`step_sizes.rs`)
Demonstrates configurable step intervals for increment/decrement operations.

![Step Sizes Demo](vhs/target/step_sizes.gif)

```bash
cargo run --example step_sizes
```

**Features:**
- 9 sliders with different step sizes (0.5 to 25.0)
- Real-time step size adjustment
- Dynamic step modification
- Fine control (0.1) to coarse control (25.0+)
- Special ranges (percentages, dB, temperature)

**Controls:**
- `↑/↓` or `j/k`: Select slider
- `←/→` or `h/l`: Adjust value (uses configured step)
- `1`: Set step to 1.0
- `2`: Set step to 2.0
- `5`: Set step to 5.0
- `+`: Double current step size
- `-`: Halve current step size
- `q` or `ESC`: Quit

**Use Cases:**
- Fine audio control (0.5 steps)
- Standard UI controls (1.0 steps)
- Quick adjustments (5.0 steps)
- Temperature controls (0.1 steps)
- Percentage sliders (0.01 steps)

---

### Examples WITHOUT Thumb Indicators

#### 7. Progress Bars (`progress_bars.rs`)
Various progress bar styles perfect for loading indicators and status displays.

![Progress Bars Demo](vhs/target/progress_bars.gif)

```bash
cargo run --example progress_bars
```

**Features:**
- 10 different progress bar styles
- Auto-progress animation mode
- Color coding based on percentage
- Download, Upload, Health, Mana, Experience bars
- Battery, Loading, Installation indicators

**Controls:**
- `SPACE`: Toggle auto-progress ON/OFF
- `q` or `ESC`: Quit

**Use Cases:**
- Loading screens
- Download/upload progress
- Game status bars
- Battery indicators

---

#### 8. Status Bars (`status_bars.rs`)
Game-style character stats and system monitoring without thumb indicators.

![Status Bars Demo](vhs/target/status_bars.gif)

```bash
cargo run --example status_bars
```

**Features:**
- RPG-style character status (Health, Mana, Stamina, XP)
- Multiple characters
- System resource monitoring (CPU, Memory, Disk, Network)
- Auto-regeneration mode
- Interactive stat manipulation
- Color-coded based on values

**Controls:**
- `↑/↓` or `j/k`: Select character
- `h`: Damage selected character
- `m`: Use mana
- `s`: Use stamina
- `SPACE`: Toggle auto-update
- `q` or `ESC`: Quit

**Use Cases:**
- Game interfaces
- RPG character sheets
- System monitoring dashboards
- Resource usage displays

---

### Testing Examples

#### 9. Width Test (`width_test.rs`)
Simple test to verify all sliders render with consistent width.

```bash
cargo run --example width_test
```

**Features:**
- Multiple symbol combinations at 50% value
- Verifies unicode width handling
- All sliders should have identical visual length

**Controls:**
- `q` or `ESC`: Quit

---

## Quick Start Guide

### With Thumb Indicator (Interactive Controls)

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
    .show_handle(true);  // Show the thumb/handle
```

### Without Thumb Indicator (Progress Bars)

```rust
use tui_slider::{Slider, SliderState};
use ratatui::style::Color;

let progress = SliderState::new(65.0, 0.0, 100.0);
let bar = Slider::from_state(&progress)
    .filled_symbol("█")
    .empty_symbol("░")
    .filled_color(Color::Green)
    .show_handle(false);  // Hide the thumb/handle
```

## Feature Highlights

### Handle/Thumb Visibility Control
All sliders support toggling the handle indicator:
- `.show_handle(true)` - Show the indicator (default)
- `.show_handle(false)` - Hide for progress bar style
- `.show_thumb(bool)` - Alias for show_handle

### Consistent Width
All sliders are rendered with consistent visual length regardless of which Unicode symbols are used, thanks to proper unicode width handling.

### Customization Options
- Orientation: Horizontal or Vertical
- Symbols: Filled, Empty, Handle
- Colors: Filled, Empty, Handle
- Display: Show/hide value, show/hide handle
- Blocks: Optional border blocks with titles

## Symbol Library

The library includes predefined symbol sets in `tui_slider::symbols`:

```rust
use tui_slider::symbols;

// Filled symbols
symbols::FILLED_THICK_LINE  // "━"
symbols::FILLED_BLOCK       // "█"
symbols::FILLED_PROGRESS    // "▰"
symbols::FILLED_WAVE        // "≈"

// Empty symbols
symbols::EMPTY_THIN_LINE    // "─"
symbols::EMPTY_LIGHT_SHADE  // "░"
symbols::EMPTY_PROGRESS     // "▱"
symbols::EMPTY_WAVE         // "˜"

// Handle symbols
symbols::HANDLE_CIRCLE      // "●"
symbols::HANDLE_DIAMOND     // "◆"
symbols::HANDLE_SQUARE      // "■"
symbols::HANDLE_SPARKLE     // "✨"
```

## Running All Examples

```bash
# Examples with thumb indicators
cargo run --example horizontal
cargo run --example vertical
cargo run --example custom
cargo run --example handle_toggle
cargo run --example thumb_toggle
cargo run --example step_sizes

# Examples without thumb indicators
cargo run --example progress_bars
cargo run --example status_bars

# Testing examples
cargo run --example width_test
```

## When to Use Which Style

### With Thumb/Handle (`.show_handle(true)`)
✅ Interactive volume/brightness controls  
✅ User-adjustable sliders  
✅ When precise position feedback matters  
✅ Settings and configuration UIs  

### Without Thumb/Handle (`.show_handle(false)`)
✅ Progress bars (loading, downloading)  
✅ Status indicators (health, mana, battery)  
✅ System resource monitors  
✅ Read-only displays  
✅ Game UI elements  

## Documentation

For more detailed information, see:
- `../README.md` - Main project documentation
- `../docs/HANDLE_VISIBILITY.md` - Handle visibility feature guide
- `../docs/NO_THUMB_EXAMPLES.md` - Detailed guide for no-thumb examples
- `../docs/STEP_SIZES.md` - Step size configuration guide
- `../docs/UNICODE_WIDTH_FIX.md` - Technical details on width consistency

## Generating Demo GIFs

All demo GIFs can be regenerated using the justfile commands:

```bash
# Generate individual demos
just vhs-horizontal
just vhs-vertical
just vhs-custom
just vhs-thumb
just vhs-handle-toggle
just vhs-step-sizes
just vhs-progress-bars
just vhs-status-bars
just vhs-border-colors
just vhs-border-styles
just vhs-horizontal-styles
just vhs-vertical-styles
just vhs-vertical-positioning
just vhs-title-alignment
just vhs-value-alignment

# Generate all demos at once
just vhs-all
```

**Requirements:**
- [VHS](https://github.com/charmbracelet/vhs) - Install with `go install github.com/charmbracelet/vhs@latest`

## Contributing

When adding new examples:
1. Create the example file in `examples/`
2. Add it to `Cargo.toml` under `[[example]]`
3. Create a VHS tape in `examples/vhs/your_example.tape`
4. Add a justfile command in the root `justfile`
5. Generate the GIF: `just vhs-your-example`
6. Update this README with description, usage, and GIF reference
7. Test that it builds: `cargo build --example your_example`
8. Test that it runs: `cargo run --example your_example`

## License

All examples are released under the same MIT license as the main library.