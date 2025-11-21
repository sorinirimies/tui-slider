# VHS tape for border colors demo

Output examples/vhs/target/border_colors.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example border_colors"
Enter
Sleep 2s

# Show the interface with different border colors
Sleep 1s

# Navigate through different colored borders
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms

# Adjust slider to show the colored borders better
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Show more color examples
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms

# Go back to show variety
Type "Left"
Sleep 800ms
Type "Left"
Sleep 800ms

# Adjust values
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Navigate through more examples
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
