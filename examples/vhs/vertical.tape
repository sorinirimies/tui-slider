# VHS tape for vertical slider styles demo

Output examples/vhs/output/vertical.gif

Set Shell "bash"
Set FontSize 14
Set Width 1500
Set Height 900
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example vertical"
Enter
Sleep 2s

# Show the interface with all vertical slider styles
Sleep 1s

# Navigate through sliders - show different styles
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms

# Adjust Volume slider (first slider with default style)
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 1s

# Move to Bass slider (blocks style)
Type "h"
Sleep 500ms
Type "h"
Sleep 500ms

# Adjust Bass values
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 1s

# Show Delay slider (wave style)
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 1s

# Navigate to Chorus (progress style)
Type "l"
Sleep 500ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 1s

# Show Phaser slider (retro style)
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 1s

# Final adjustments on Attack slider (segmented blocks)
Type "l"
Sleep 500ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
