# VHS tape for border styles demo

Output examples/vhs/target/border_styles.gif

Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 600
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example border_styles"
Enter
Sleep 2s

# Show the interface
Sleep 1s

# Navigate through different border styles
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms

# Go back to show variety
Type "Left"
Sleep 800ms
Type "Left"
Sleep 800ms

# Adjust values to show the borders better
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Show more styles
Type "Right"
Sleep 800ms
Type "Right"
Sleep 800ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
