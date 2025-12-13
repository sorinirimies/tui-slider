# VHS tape for comprehensive slider demo
# Shows both horizontal and vertical sliders with various styles
# with and without handles, different title/value positioning, and smooth animations

Output examples/vhs/output/comprehensive_demo.gif

Set Shell "bash"
Set FontSize 14
Set Width 1700
Set Height 1100
Set PlaybackSpeed 1.5
Set Theme "Aardvark Blue"

Type "cargo run --example comprehensive_demo"
Enter
Sleep 2s

# Show the initial state with both horizontal and vertical sections
Sleep 1s

# ============================================================
# HORIZONTAL SLIDERS SECTION - Demonstrate animations
# ============================================================

# Start with first slider - show smooth value changes
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms

Sleep 800ms

# Go back down
Type "h"
Sleep 150ms
Type "h"
Sleep 150ms
Type "h"
Sleep 150ms

Sleep 1s

# Navigate to second slider and animate it
Type "j"
Sleep 400ms

# Animate value changes smoothly
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms

Sleep 800ms

# Navigate to "Blocks (Handle, Value Left)" - show handle animation
Type "j"
Sleep 400ms

Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms

Sleep 800ms

# Animate backwards
Type "h"
Sleep 150ms
Type "h"
Sleep 150ms
Type "h"
Sleep 150ms
Type "h"
Sleep 150ms

Sleep 1s

# Navigate to "Wave (No Handle)" - smooth progress animation
Type "j"
Sleep 400ms

Type "h"
Sleep 120ms
Type "h"
Sleep 120ms
Type "h"
Sleep 120ms
Type "h"
Sleep 120ms
Type "h"
Sleep 120ms

Sleep 800ms

# Show "Gradient (Handle, Value Center)" with animation
Type "j"
Sleep 400ms

Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms

Sleep 1s

# Navigate to "Retro (No Handle)" and animate
Type "j"
Sleep 400ms

Type "h"
Sleep 130ms
Type "h"
Sleep 130ms
Type "h"
Sleep 130ms
Type "h"
Sleep 130ms

Sleep 800ms

# Show "Dots (Handle, Value Right)" with smooth animation
Type "j"
Sleep 400ms

Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms

Sleep 1s

# Navigate to "Minimal (No Handle)" - quick animation
Type "j"
Sleep 400ms

Type "h"
Sleep 120ms
Type "h"
Sleep 120ms
Type "h"
Sleep 120ms

Sleep 800ms

# Show segmented blocks with animation
Type "j"
Sleep 400ms

Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms
Type "l"
Sleep 150ms

Sleep 800ms

# Last horizontal slider - segmented squares
Type "j"
Sleep 400ms

Type "h"
Sleep 130ms
Type "h"
Sleep 130ms
Type "h"
Sleep 130ms
Type "h"
Sleep 130ms
Type "h"
Sleep 130ms

Sleep 1s

# Quick showcase - go back and show variety
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms

Type "l"
Sleep 140ms
Type "l"
Sleep 140ms
Type "l"
Sleep 140ms

Sleep 1s

# ============================================================
# SWITCH TO VERTICAL SLIDERS SECTION
# ============================================================

Type "Tab"
Sleep 1s

# Navigate and animate vertical sliders with value positioning
Type "l"
Sleep 400ms

# Animate first vertical slider (Bottom-Left)
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms

Sleep 800ms

Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms

Sleep 1s

# Move to second slider (Bottom-Center) and animate
Type "l"
Sleep 400ms

Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms

Sleep 800ms

# Move to third slider (Bottom-Right) and animate
Type "l"
Sleep 400ms

Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms

Sleep 1s

# Show fourth slider (Top-Left) with animation
Type "l"
Sleep 400ms

Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms

Sleep 800ms

# Show fifth slider (Top-Center) with animation
Type "l"
Sleep 400ms

Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms

Sleep 1s

# Show sixth slider (Top-Right) with animation
Type "l"
Sleep 400ms

Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms

Sleep 800ms

# Show dots slider (Bottom-Left) with animation
Type "l"
Sleep 400ms

Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms

Sleep 1s

# Show minimal slider (Top-Center) with animation
Type "l"
Sleep 400ms

Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms

Sleep 800ms

# Show segmented blocks (Bottom-Right) with animation
Type "l"
Sleep 400ms

Type "j"
Sleep 140ms
Type "j"
Sleep 140ms
Type "j"
Sleep 140ms

Sleep 800ms

# Show segmented squares (Top-Left) with animation
Type "l"
Sleep 400ms

Type "k"
Sleep 140ms
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms
Type "k"
Sleep 140ms

Sleep 1s

# Go back to show some earlier sliders
Type "h"
Sleep 300ms
Type "h"
Sleep 300ms
Type "h"
Sleep 300ms

# Animate a bit more
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms
Type "k"
Sleep 130ms

Sleep 1s

# ============================================================
# SWITCH BACK TO HORIZONTAL SLIDERS - Final showcase
# ============================================================

Type "Tab"
Sleep 1s

# Navigate to middle sliders and create smooth animations
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

# Create a wave effect with rapid adjustments
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms
Type "l"
Sleep 120ms

Sleep 800ms

Type "h"
Sleep 120ms
Type "h"
Sleep 120ms
Type "h"
Sleep 120ms

Sleep 1s

# Move to another slider and animate
Type "k"
Sleep 300ms
Type "k"
Sleep 300ms

Type "l"
Sleep 130ms
Type "l"
Sleep 130ms
Type "l"
Sleep 130ms
Type "l"
Sleep 130ms

Sleep 1s

# Final navigation
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms

Type "h"
Sleep 140ms
Type "h"
Sleep 140ms
Type "h"
Sleep 140ms

Sleep 2s

# Exit
Type "q"
Sleep 1s
