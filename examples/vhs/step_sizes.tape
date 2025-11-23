# VHS tape for step sizes demo

Output examples/vhs/output/step_sizes.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example step_sizes"
Enter
Sleep 2s

# Show the interface with different step sizes
Sleep 1s

# Navigate through sliders
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust slider with step size 1 (fine control)
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms

Sleep 1s

# Move to slider with step size 5
Type "j"
Sleep 800ms

# Adjust with larger steps
Type "h"
Sleep 300ms
Type "h"
Sleep 300ms
Type "h"
Sleep 300ms

Sleep 1s

# Show step size 10
Type "j"
Sleep 800ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms

Sleep 1s

# Navigate to step size 25
Type "j"
Sleep 800ms
Type "h"
Sleep 300ms
Type "h"
Sleep 300ms

Sleep 1s

# Show step size 50 (large jumps)
Type "j"
Sleep 800ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms

Sleep 1s

# Go back to show the differences
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms
Type "l"
Sleep 300ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
