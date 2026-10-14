# VHS tape for horizontal slider demo (merged styles with pagination)

Output examples/vhs/output/horizontal.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 1000
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Hide
Type "clear && cargo run --release --example horizontal"
Enter
Sleep 5s
Show

# Switch theme (t) to show shared theming
Sleep 1s
Type "t"
Sleep 1s

# Page 1: Standard Styles
Sleep 1s

# Navigate through standard styles
Type "j"
Sleep 500ms
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

# Show more standard styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Go to Page 2: Specialty Styles
Type "n"
Sleep 1s

# Navigate through specialty styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust segmented styles
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Show dots and squares styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show more specialty styles
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Go back to Page 1
Type "p"
Sleep 1s

# Final navigation
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Cycle themes
Type "t"
Sleep 700ms
Type "t"
Sleep 700ms
Type "T"
Sleep 700ms

# Exit
Type "q"
Sleep 1s
