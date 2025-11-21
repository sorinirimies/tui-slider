# VHS tape for status bars demo

Output examples/vhs/target/status_bars.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 900
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example status_bars"
Enter
Sleep 2s

# Show the interface with status bars
Sleep 1s

# Navigate through status bars
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust values to show progress
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to another status bar
Type "j"
Sleep 800ms

# Increase progress
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Navigate to show different status bar styles
Type "j"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show more status bars
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Navigate back to show variety
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
