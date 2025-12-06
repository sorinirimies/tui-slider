# VHS tape for borders demo (combined styles and colors)

Output examples/vhs/output/borders.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 900
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example borders"
Enter
Sleep 2s

# Show the interface with different border colors
Sleep 1s

# Navigate through colored borders on page 1
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust slider values
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 800ms

# Navigate more
Type "k"
Sleep 500ms
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

# Go to next page to show border styles
Type "n"
Sleep 1s

# Navigate through border styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust values to show different styles
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Show more styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Go to next page
Type "n"
Sleep 1s

# Navigate through more border variations
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust values
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Go back to previous page
Type "p"
Sleep 1s

# Navigate
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms

Sleep 1s

# Go back to first page
Type "p"
Sleep 1s

# Final adjustments
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
