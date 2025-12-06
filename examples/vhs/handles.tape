# VHS tape for handles demo (side-by-side with/without handles comparison)

Output examples/vhs/output/handles.gif

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 900
Set PlaybackSpeed 1.5
Set Theme "Catppuccin Mocha"

Type "cargo run --example handles"
Enter
Sleep 2s

# Show the side-by-side interface
Sleep 1s

# Left section is active (with handles)
# Navigate through sliders to show handles in action
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust values to show handles moving
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Navigate to another slider
Type "k"
Sleep 500ms

# Adjust values
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show more sliders
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Switch to RIGHT section (WITHOUT handles)
Type "Tab"
Sleep 1s

# Navigate through progress bar style sliders
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Adjust values to show smooth progress without handles
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

# Navigate more
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms

# Adjust values
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms
Type "h"
Sleep 200ms

Sleep 1s

# Show more progress bars
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 1s

# Switch back to LEFT section (with handles)
Type "Tab"
Sleep 1s

# Final navigation to show handles again
Type "k"
Sleep 500ms
Type "k"
Sleep 500ms

Type "l"
Sleep 200ms
Type "l"
Sleep 200ms
Type "l"
Sleep 200ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
