# VHS tape for handle toggle demo

Output examples/vhs/target/handle_toggle.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example handle_toggle"
Enter
Sleep 2s

# Show the interface with handle toggle examples
Sleep 1s

# Navigate through sliders
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust slider with handle visible
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to slider without handle
Type "j"
Sleep 800ms

# Adjust slider without handle
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Navigate to show more examples
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Go back to show variety
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show final examples
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
