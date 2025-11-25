# VHS tape for step sizes demo

Output examples/vhs/output/step_sizes.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example step_sizes"
Enter
Sleep 2s

# Show the interface with 5 different step sizes
Sleep 1s

# Start with Fine Control (step 0.5)
# Make small adjustments to show precision
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms

Sleep 1s

# Move to Standard (step 1.0)
Type "j"
Sleep 600ms

# Adjust with standard steps
Type "h"
Sleep 250ms
Type "h"
Sleep 250ms
Type "h"
Sleep 250ms

Sleep 1s

# Move to Coarse (step 5.0)
Type "j"
Sleep 600ms

# Show larger jumps
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms

Sleep 1s

# Move to Large Jumps (step 10.0)
Type "j"
Sleep 600ms

# Demonstrate big increments
Type "h"
Sleep 250ms
Type "h"
Sleep 250ms

Sleep 1s

# Move to Percentage (step 0.01, range 0-1)
Type "j"
Sleep 600ms

# Show percentage-based adjustments
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms

Sleep 1s

# Go back to show contrast between step sizes
Type "k"
Sleep 600ms
Type "k"
Sleep 600ms
Type "k"
Sleep 600ms

# Adjust Fine Control again to show difference
Type "l"
Sleep 250ms
Type "l"
Sleep 250ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
