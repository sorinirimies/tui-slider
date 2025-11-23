# 🎨 Examples Gallery

A visual showcase of all `tui-slider` examples with animated demos.

## 🎯 Quick Navigation

- [Horizontal Sliders](#horizontal-sliders)
- [Vertical Sliders](#vertical-sliders)
- [Styling & Customization](#styling--customization)
- [Interactive Features](#interactive-features)
- [Layout & Positioning](#layout--positioning)
- [Progress & Display](#progress--display)

---

## Horizontal Sliders

### Basic Horizontal Styles
Multiple horizontal slider styles with different symbols and visual themes.

![Horizontal Demo](vhs/output/horizontal.gif)

🚀 `cargo run --example horizontal`

---

### Horizontal Style Variations
Extended collection of horizontal slider visual styles.

![Horizontal Styles Demo](vhs/output/horizontal_styles.gif)

🚀 `cargo run --example horizontal_styles`

---

## Vertical Sliders

### Basic Vertical Styles
Vertical slider orientation with various visual styles, perfect for mixer interfaces.

![Vertical Demo](vhs/output/vertical.gif)

🚀 `cargo run --example vertical`

---

### Vertical Style Variations
Additional vertical slider styles optimized for different use cases.

![Vertical Styles Demo](vhs/output/vertical_styles.gif)

🚀 `cargo run --example vertical_styles`

---

### Vertical Positioning
Layout strategies and positioning techniques for vertical sliders.

![Vertical Positioning Demo](vhs/output/vertical_positioning.gif)

🚀 `cargo run --example vertical_positioning`

---

## Styling & Customization

### Custom Styles
Create your own slider styles with custom RGB colors and symbol combinations.

![Custom Styles Demo](vhs/output/custom.gif)

🚀 `cargo run --example custom`

---

### Border Colors
Colorful border styling with themed color palettes.

![Border Colors Demo](vhs/output/border_colors.gif)

🚀 `cargo run --example border_colors`

---

### Border Styles
Different border types and styles for slider containers.

![Border Styles Demo](vhs/output/border_styles.gif)

🚀 `cargo run --example border_styles`

---

## Interactive Features

### Handle Toggle
Show or hide slider handles for different UI contexts.

![Handle Toggle Demo](vhs/output/handle_toggle.gif)

🚀 `cargo run --example handle_toggle`

---

### Thumb Toggle
Alternative demonstration of thumb/handle visibility toggling.

![Thumb Toggle Demo](vhs/output/thumb_toggle.gif)

🚀 `cargo run --example thumb_toggle`

---

### Step Sizes
Configurable step intervals for fine or coarse value adjustments.

![Step Sizes Demo](vhs/output/step_sizes.gif)

🚀 `cargo run --example step_sizes`

---

## Layout & Positioning

### Title Alignment
Position slider titles left, center, or right.

![Title Alignment Demo](vhs/output/title_alignment.gif)

🚀 `cargo run --example title_alignment`

---

### Value Alignment
Control value display positioning for optimal layout.

![Value Alignment Demo](vhs/output/value_alignment.gif)

🚀 `cargo run --example value_alignment`

---

### Width Testing
Responsive behavior testing at various width constraints.

![Width Test Demo](vhs/output/width_test.gif)

🚀 `cargo run --example width_test`

---

## Progress & Display

### Progress Bars
Progress bar styles for loading indicators and completion displays.

![Progress Bars Demo](vhs/output/progress_bars.gif)

🚀 `cargo run --example progress_bars`

---

## 📋 Complete Example Index

| Example | Category | Description |
|---------|----------|-------------|
| `horizontal.rs` | Horizontal | Basic horizontal slider styles |
| `horizontal_styles.rs` | Horizontal | Extended horizontal variations |
| `vertical.rs` | Vertical | Basic vertical slider styles |
| `vertical_styles.rs` | Vertical | Extended vertical variations |
| `vertical_positioning.rs` | Vertical | Vertical layout strategies |
| `custom.rs` | Styling | Custom RGB colors and symbols |
| `border_colors.rs` | Styling | Colored border themes |
| `border_styles.rs` | Styling | Border type variations |
| `handle_toggle.rs` | Interactive | Toggle handle visibility |
| `thumb_toggle.rs` | Interactive | Alternative handle toggle |
| `step_sizes.rs` | Interactive | Configurable step intervals |
| `title_alignment.rs` | Layout | Title positioning options |
| `value_alignment.rs` | Layout | Value display positioning |
| `width_test.rs` | Layout | Width responsiveness testing |
| `progress_bars.rs` | Progress | Progress indicator styles |

---

## 🚀 Getting Started

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/tui-slider.git
   cd tui-slider
   ```

2. **Run any example**
   ```bash
   cargo run --example horizontal
   ```

3. **Browse the gallery**
   View all example demos and descriptions in this gallery.

---

## 🎮 Common Controls

Most examples share similar keyboard controls:

- **↑/↓** or **j/k** - Navigate between sliders
- **←/→** or **h/l** - Adjust slider values
- **q** or **ESC** - Quit the example

Check each example's guide for specific controls and features.

---

## 📚 Learn More

- **[Main README](../README.md)** - Library documentation and API reference
- **[Examples README](README.md)** - Detailed examples documentation
- **[Contributing Guide](../CONTRIBUTING.md)** - How to contribute

---

## 🎬 Regenerate Demos

All GIF demos are generated using [VHS](https://github.com/charmbracelet/vhs). To regenerate:

```bash
# Install VHS
go install github.com/charmbracelet/vhs@latest

# Generate all demos
just vhs-all

# Or generate specific demos
just vhs-horizontal
just vhs-vertical
# ... etc
```

See the [VHS README](vhs/README.md) for more details.

---

**Happy Sliding! 🎚️**