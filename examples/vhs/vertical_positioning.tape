# VHS tape for vertical slider positioning demo

Output examples/vhs/target/vertical_positioning.gif

Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 700
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example vertical_positioning"
Enter
Sleep 2s

# Show the interface with all positioning combinations
Sleep 1s

# Navigate through different positioning styles
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms
Type "l"
Sleep 500ms

# Adjust first slider (Volume - Label: Top, Value: Bottom/Center)
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

# Move to slider with different positioning (Treble - Label: Top, Value: Middle/Left)
Type "l"
Sleep 500ms

# Adjust values
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 1s

# Show another positioning style (Delay - Label: Top, Value: Middle/Right)
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

# Navigate to bottom label position (Gain - Label: Bottom, Value: Middle/Center)
Type "l"
Sleep 500ms
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

# Show final positioning style (Attack - Label: Top, Value: Bottom/Right)
Type "l"
Sleep 500ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
