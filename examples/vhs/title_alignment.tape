# VHS tape for title alignment demo

Output examples/vhs/output/title_alignment.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 600
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Hide
Type "clear && cargo run --release --example title_alignment"
Enter
Sleep 5s
Show

# Switch theme (t) to show shared theming
Sleep 1s
Type "t"
Sleep 1s

# Show the interface with different title alignments
Sleep 1s

# Navigate through sliders to show title positions
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust some values to show the alignment
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Move back up
Type "k"
Sleep 800ms

# Adjust this one
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Navigate to show all examples
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust
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
