# VHS tape for horizontal slider demo

Output examples/vhs/target/horizontal.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 900
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example horizontal"
Enter
Sleep 2s

# Show the interface
Sleep 1s

# Navigate through sliders
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust some values
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to another slider
Type "k"
Sleep 500ms

# Adjust values
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Move through all sliders
Type "j"
Sleep 500ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

Type "j"
Sleep 500ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
