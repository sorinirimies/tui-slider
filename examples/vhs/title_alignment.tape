# VHS tape for title alignment demo

Output examples/vhs/target/title_alignment.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example title_alignment"
Enter
Sleep 2s

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

# Exit
Type "q"
Sleep 1s
