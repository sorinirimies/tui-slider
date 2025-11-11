# VHS Tapes for Demo GIFs

This directory contains [VHS](https://github.com/charmbracelet/vhs) tape files for generating demo GIFs of the tui-slider library examples.

## Prerequisites

Install VHS:

```bash
# macOS
brew install vhs

# Linux
# Download from https://github.com/charmbracelet/vhs/releases

# Or with Go
go install github.com/charmbracelet/vhs@latest
```

## Generating Demo GIFs

### Generate Individual Demos

```bash
# Horizontal sliders demo
just vhs-horizontal

# Vertical sliders demo
just vhs-vertical
```

### Generate All Demos

```bash
just vhs-all
```

### Clean Generated Files

```bash
just vhs-clean
```

## Manual Generation

You can also run VHS directly:

```bash
vhs examples/vhs/horizontal.tape
vhs examples/vhs/vertical.tape
```

## Output

Generated GIFs will be saved in the `examples/vhs/target/` directory:
- `horizontal.gif`
- `vertical.gif`

## Customization

You can edit the `.tape` files to:
- Change output format (GIF, MP4, WebM)
- Adjust timing and playback speed
- Modify terminal theme
- Change window size
- Add custom interactions

## VHS Tape Format

Each tape file follows this structure:

```tape
Output examples/vhs/target/output.gif
Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 900
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example name"
Enter
Sleep 2s
# ... interactions ...
Type "q"
```

## Available Themes

VHS supports many themes. Popular options:
- Catppuccin Mocha (default)
- Dracula
- Nord
- Solarized Dark
- Tokyo Night
- Gruvbox

To change theme, edit the `Set Theme` line in any tape file.

## Tips

- Adjust `PlaybackSpeed` to make demos faster or slower
- Use `Sleep` commands to control timing between actions
- Test tapes with shorter sleep times for faster iteration
- Increase terminal size for better visibility

## Troubleshooting

If GIF generation fails:
1. Ensure VHS is installed: `vhs --version`
2. Check that the example builds: `cargo build --example <name>`
3. Verify terminal dimensions are reasonable
4. Try running the example manually first

## Documentation

- VHS Documentation: https://github.com/charmbracelet/vhs
- VHS Examples: https://github.com/charmbracelet/vhs/tree/main/examples