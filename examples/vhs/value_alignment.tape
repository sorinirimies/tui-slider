# VHS tape for value alignment demo

Output examples/vhs/output/value_alignment.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example value_alignment"
Enter
Sleep 2s

# Show the interface with different value alignments
Sleep 1s

# Navigate through sliders to show value positions
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust values to show the alignment
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

# Adjust this one
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show progress bar examples
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
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
