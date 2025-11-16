# VHS tape for vertical slider demo

Output examples/vhs/target/vertical.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 600
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example vertical"
Enter
Sleep 2s

# Show the interface
Sleep 1s

# Navigate through sliders
Type "Right"
Sleep 500ms
Type "Right"
Sleep 500ms
Type "Right"
Sleep 500ms

# Adjust some values
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

# Move to another slider
Type "Left"
Sleep 500ms
Type "Left"
Sleep 500ms

# Adjust values
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 1s

# Show different vertical sliders
Type "Right"
Sleep 500ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 1s

Type "Right"
Sleep 500ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
