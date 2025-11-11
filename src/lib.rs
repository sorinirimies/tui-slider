//! # tui-slider
//!
//! A simple TUI slider component library for ratatui.
//!
//! This library provides horizontal and vertical sliders for terminal user interfaces.
//!
//! ## Features
//!
//! - **Horizontal and Vertical sliders** - Support for both orientations
//! - **Simple styling** - Customizable colors and symbols
//! - **State management** - Built-in state for value tracking
//! - **Easy to use** - Minimal configuration required
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ratatui::prelude::*;
//! use tui_slider::{Slider, SliderState, SliderOrientation};
//!
//! // Create a slider state
//! let mut state = SliderState::new(50.0, 0.0, 100.0);
//!
//! // Create a slider widget
//! let slider = Slider::from_state(&state)
//!     .orientation(SliderOrientation::Horizontal)
//!     .label("Volume")
//!     .show_value(true);
//!
//! // Render it (in your terminal UI loop)
//! // frame.render_widget(slider, area);
//! ```
//!
//! ## Customization
//!
//! Customize the appearance with colors and symbols:
//!
//! ```rust
//! use ratatui::style::Color;
//! use tui_slider::{Slider, SliderState};
//!
//! let state = SliderState::new(50.0, 0.0, 100.0);
//!
//! let slider = Slider::from_state(&state)
//!     .filled_symbol("━")
//!     .empty_symbol("─")
//!     .handle_symbol("●")
//!     .filled_color(Color::Cyan)
//!     .empty_color(Color::DarkGray)
//!     .handle_color(Color::White);
//! ```
//!
//! ## Vertical Sliders
//!
//! Create vertical sliders for equalizers and level meters:
//!
//! ```rust
//! use tui_slider::{Slider, SliderState, SliderOrientation};
//!
//! let state = SliderState::new(60.0, 0.0, 100.0);
//!
//! let vertical_slider = Slider::from_state(&state)
//!     .orientation(SliderOrientation::Vertical)
//!     .label("Bass");
//! ```

pub mod orientation;
pub mod slider;
pub mod state;

// Re-export main types
pub use orientation::SliderOrientation;
pub use slider::Slider;
pub use state::SliderState;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::orientation::SliderOrientation;
    pub use crate::slider::Slider;
    pub use crate::state::SliderState;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_state() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.value(), 50.0);

        state.set_value(75.0);
        assert_eq!(state.value(), 75.0);

        state.set_value(150.0); // Should clamp to max
        assert_eq!(state.value(), 100.0);

        state.set_value(-10.0); // Should clamp to min
        assert_eq!(state.value(), 0.0);
    }

    #[test]
    fn test_slider_percentage() {
        let state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.5);

        let state = SliderState::new(25.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.25);
    }
}
