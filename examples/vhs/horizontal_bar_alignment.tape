# VHS tape for horizontal bar alignment demo

Output examples/vhs/output/horizontal_bar_alignment.gif

Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 900
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example horizontal_bar_alignment"
Enter
Sleep 2s

# Show the interface with all alignment options
Sleep 1s

# Navigate through different alignments
Type "j"
Sleep 500ms

# Adjust Top Aligned slider
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to Center Aligned
Type "j"
Sleep 500ms

# Adjust Center Aligned slider
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Move to Bottom Aligned
Type "j"
Sleep 500ms

# Adjust Bottom Aligned slider
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to Progress Top
Type "j"
Sleep 500ms

# Adjust Progress Top
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Move to Progress Center
Type "j"
Sleep 500ms

# Adjust Progress Center
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move to Progress Bottom
Type "j"
Sleep 500ms

# Adjust Progress Bottom
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 2s

# Go back to top to show all alignments
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
