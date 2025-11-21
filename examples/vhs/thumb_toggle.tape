# VHS tape for thumb toggle demo

Output examples/vhs/target/thumb_toggle.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example thumb_toggle"
Enter
Sleep 2s

# Show the interface
Sleep 1s

# Navigate through sliders
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust slider with thumb visible
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to next slider
Type "j"
Sleep 800ms

# Adjust slider without thumb
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Navigate to show different combinations
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Show more examples
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Final adjustments
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
