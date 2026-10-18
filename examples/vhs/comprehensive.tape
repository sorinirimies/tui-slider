# VHS tape for the comprehensive studio-mixer demo

Output examples/vhs/output/comprehensive_demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1400
Set Height 900
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Hide
Type "clear && cargo run --release --features theme,crossterm --example comprehensive"
Enter
Sleep 6s
Show

Sleep 1s

# Channel faders: adjust Gain, then Bass
Up
Sleep 150ms
Up
Sleep 150ms
Up
Sleep 150ms
Down
Sleep 150ms
Tab
Sleep 400ms
Down
Sleep 150ms
Down
Sleep 150ms
Down
Sleep 150ms
Tab
Sleep 400ms
PageUp
Sleep 400ms
Tab
Sleep 400ms
Up
Sleep 150ms
Up
Sleep 150ms
Tab
Sleep 400ms
PageUp
Sleep 400ms
PageDown
Sleep 400ms
PageDown
Sleep 400ms
Tab
Sleep 400ms
Up
Sleep 150ms
Up
Sleep 150ms

# Horizontal controls: Pan (negative range)
Tab
Sleep 500ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms

# Tempo
Tab
Sleep 500ms
Right
Sleep 200ms
Right
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms

# Cutoff (segmented)
Tab
Sleep 500ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms

# Resonance
Tab
Sleep 500ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms

# Drive: 0.1 steps, no float drift
Tab
Sleep 500ms
Right
Sleep 300ms
Right
Sleep 300ms
Right
Sleep 300ms
Left
Sleep 300ms

# Mix
Tab
Sleep 500ms
Left
Sleep 200ms
Left
Sleep 200ms
Left
Sleep 200ms

# Attack (segmented)
Tab
Sleep 500ms
Right
Sleep 200ms
Right
Sleep 200ms
Right
Sleep 200ms
Sleep 1s

# Themes
Type "t"
Sleep 900ms
Type "t"
Sleep 900ms
Type "t"
Sleep 900ms
Type "T"
Sleep 600ms
Type "T"
Sleep 1s

Type "q"
Sleep 1s
