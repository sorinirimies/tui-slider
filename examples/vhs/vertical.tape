# VHS tape for vertical slider/equalizer demo

Output examples/vhs/target/vertical.gif

Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 900
Set PlaybackSpeed 1.0
Set Theme "Catppuccin Mocha"

Type "cargo run --example vertical"
Enter
Sleep 2s

# Show the interface
Sleep 1s

# Navigate through bands
Type "l"
Sleep 400ms
Type "l"
Sleep 400ms
Type "l"
Sleep 400ms

# Adjust some bands up
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 800ms

# Move to next band
Type "l"
Sleep 400ms

# Adjust down
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 800ms

# Navigate and adjust creating EQ curve
Type "l"
Sleep 400ms
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

Sleep 800ms

Type "l"
Sleep 400ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms
Type "k"
Sleep 200ms

Sleep 800ms

Type "l"
Sleep 400ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 800ms

# Move through remaining bands
Type "l"
Sleep 400ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 800ms

Type "l"
Sleep 400ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms
Type "j"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
