# VHS tape for custom slider styles demo

Output examples/vhs/output/custom.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 800
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Hide
Type "clear && cargo run --release --features theme --example custom"
Enter
Sleep 5s
Show

# Switch theme (t) to show shared theming
Sleep 1s
Type "t"
Sleep 1s

# Show the interface with all custom styles
Sleep 1s

# Navigate through different custom styles
Type "j"
Sleep 800ms
Type "j"
Sleep 800ms

# Adjust Sunset Gradient slider
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

# Move to Ocean Wave style
Type "j"
Sleep 800ms

# Adjust values
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show Forest style
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Navigate to Fire style
Type "j"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show Electric style
Type "j"
Sleep 800ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Final style - Ice
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "k"
Sleep 800ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
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
