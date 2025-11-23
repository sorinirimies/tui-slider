# VHS tape for progress bars demo

Output examples/vhs/output/progress_bars.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 900
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example progress_bars"
Enter
Sleep 2s

# Show the interface with progress bars
Sleep 1s

# Navigate through progress bars
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

# Move to another progress bar
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

# Navigate to show different styles
Type "k"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

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
