//! Main slider widget module
//!
//! This module provides a simple slider widget that can be rendered in a ratatui application.
//!
//! # Overview
//!
//! The slider widget supports both horizontal and vertical orientations and can be customized
//! with different colors and symbols. It's designed to be simple and straightforward to use
//! while still providing enough flexibility for common use cases.
//!
//! # Features
//!
//! - Horizontal and vertical orientations
//! - Customizable colors for filled, empty, and handle
//! - Customizable symbols for bar and handle
//! - Optional label and value display
//! - Optional handle/thumb display
//! - State management with bounds checking

use crate::{orientation::SliderOrientation, state::SliderState};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};
use unicode_width::UnicodeWidthStr;

/// A simple slider widget for ratatui
///
/// This widget can be used to display and control values in a terminal UI.
/// It supports both horizontal and vertical orientations with basic styling.
///
/// # Examples
///
/// ## Basic Horizontal Slider
///
/// ```rust,no_run
/// use ratatui::prelude::*;
/// use ratatui::style::Color;
/// use tui_slider::{Slider, SliderState, SliderOrientation};
///
/// let state = SliderState::new(50.0, 0.0, 100.0);
/// let slider = Slider::from_state(&state)
///     .orientation(SliderOrientation::Horizontal)
///     .label("Volume")
///     .show_value(true)
///     .filled_color(Color::Cyan);
/// ```
///
/// ## Vertical Slider
///
/// ```rust,no_run
/// use ratatui::prelude::*;
/// use ratatui::style::Color;
/// use tui_slider::{Slider, SliderState, SliderOrientation};
///
/// let state = SliderState::new(75.0, 0.0, 100.0);
/// let slider = Slider::from_state(&state)
///     .orientation(SliderOrientation::Vertical)
///     .filled_symbol("│")
///     .empty_symbol("│")
///     .handle_symbol("━")
///     .filled_color(Color::Green);
/// ```
///
/// ## Custom Colors and Symbols
///
/// ```rust,no_run
/// use ratatui::prelude::*;
/// use ratatui::style::Color;
/// use tui_slider::{Slider, SliderState};
///
/// let state = SliderState::new(60.0, 0.0, 100.0);
/// let slider = Slider::from_state(&state)
///     .filled_symbol("█")
///     .empty_symbol("░")
///     .handle_symbol("▐")
///     .filled_color(Color::Yellow)
///     .empty_color(Color::DarkGray)
///     .handle_color(Color::White)
///     .show_handle(true);
/// ```
#[derive(Debug, Clone)]
pub struct Slider<'a> {
    /// Optional block for borders
    block: Option<Block<'a>>,
    /// Slider orientation
    orientation: SliderOrientation,
    /// Current value
    value: f64,
    /// Minimum value
    min: f64,
    /// Maximum value
    max: f64,
    /// Optional label
    label: Option<String>,
    /// Whether to show the value
    show_value: bool,
    /// Filled bar symbol
    filled_symbol: String,
    /// Empty bar symbol
    empty_symbol: String,
    /// Handle symbol
    handle_symbol: String,
    /// Filled bar color
    filled_color: Color,
    /// Empty bar color
    empty_color: Color,
    /// Handle color
    handle_color: Color,
    /// Whether to show handle
    show_handle: bool,
}

impl<'a> Slider<'a> {
    /// Creates a new slider with default settings
    ///
    /// # Arguments
    ///
    /// * `value` - Initial value (will be clamped to min..max)
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::new(50.0, 0.0, 100.0);
    /// ```
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        Self {
            block: None,
            orientation: SliderOrientation::Horizontal,
            value: value.clamp(min, max),
            min,
            max,
            label: None,
            show_value: false,
            filled_symbol: "━".to_string(),
            empty_symbol: "─".to_string(),
            handle_symbol: "●".to_string(),
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            show_handle: true,
        }
    }

    /// Creates a slider from a state
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderState};
    ///
    /// let state = SliderState::new(75.0, 0.0, 100.0);
    /// let slider = Slider::from_state(&state);
    /// ```
    pub fn from_state(state: &SliderState) -> Self {
        Self::new(state.value(), state.min(), state.max())
    }

    /// Sets the block for borders
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::widgets::{Block, Borders};
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default()
    ///     .block(Block::default().borders(Borders::ALL).title("Volume"));
    /// ```
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets the orientation (horizontal or vertical)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderOrientation};
    ///
    /// let slider = Slider::default()
    ///     .orientation(SliderOrientation::Vertical);
    /// ```
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the value (will be clamped to min..max range)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().value(75.0);
    /// ```
    pub fn value(mut self, value: f64) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Sets the minimum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().min(0.0).max(100.0);
    /// ```
    pub fn min(mut self, min: f64) -> Self {
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Sets the maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().min(0.0).max(100.0);
    /// ```
    pub fn max(mut self, max: f64) -> Self {
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Sets the label text displayed above the slider
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().label("Volume");
    /// ```
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets whether to show the numeric value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().show_value(true);
    /// ```
    pub fn show_value(mut self, show: bool) -> Self {
        self.show_value = show;
        self
    }

    /// Sets the symbol used for the filled portion of the bar
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().filled_symbol("█");
    /// ```
    pub fn filled_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.filled_symbol = symbol.into();
        self
    }

    /// Sets the symbol used for the empty portion of the bar
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().empty_symbol("░");
    /// ```
    pub fn empty_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.empty_symbol = symbol.into();
        self
    }

    /// Sets the symbol used for the slider handle
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().handle_symbol("●");
    /// ```
    pub fn handle_symbol<S: Into<String>>(mut self, symbol: S) -> Self {
        self.handle_symbol = symbol.into();
        self
    }

    /// Sets the color of the filled portion of the bar
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().filled_color(Color::Cyan);
    /// ```
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_color = color;
        self
    }

    /// Sets the color of the empty portion of the bar
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().empty_color(Color::DarkGray);
    /// ```
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Sets the color of the slider handle
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::style::Color;
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().handle_color(Color::White);
    /// ```
    pub fn handle_color(mut self, color: Color) -> Self {
        self.handle_color = color;
        self
    }

    /// Sets whether to show the handle (thumb indicator) on the slider
    ///
    /// The handle is the visual indicator that shows the current position
    /// on the slider bar. You can hide it for a cleaner, progress-bar style look.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// // Show the handle (default)
    /// let slider = Slider::default().show_handle(true);
    ///
    /// // Hide the handle for a progress bar style
    /// let slider = Slider::default().show_handle(false);
    /// ```
    pub fn show_handle(mut self, show: bool) -> Self {
        self.show_handle = show;
        self
    }

    /// Sets whether to show the thumb indicator on the slider
    ///
    /// This is an alias for `show_handle()`. The thumb is the visual indicator
    /// that shows the current position on the slider bar.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// // Show the thumb (default)
    /// let slider = Slider::default().show_thumb(true);
    ///
    /// // Hide the thumb for a progress bar style
    /// let slider = Slider::default().show_thumb(false);
    /// ```
    pub fn show_thumb(self, show: bool) -> Self {
        self.show_handle(show)
    }

    /// Calculates the percentage (0.0 to 1.0) of the current value
    fn percentage(&self) -> f64 {
        if (self.max - self.min).abs() < f64::EPSILON {
            return 0.0;
        }
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    /// Renders a horizontal slider
    ///
    /// This method ensures that all sliders have consistent visual length by:
    /// - Measuring the display width of each symbol (some Unicode chars take 2+ columns)
    /// - Tracking column positions rather than character counts
    /// - Always filling exactly `area.width` columns
    fn render_horizontal(&self, area: Rect, buf: &mut Buffer) {
        if area.width < 1 {
            return;
        }

        let percentage = self.percentage();
        let bar_width = area.width as usize;

        // Get display widths of symbols using unicode-width
        // Most symbols are 1 column wide, but some (like emojis) can be 2 or more
        let filled_width = self.filled_symbol.width().max(1);
        let empty_width = self.empty_symbol.width().max(1);
        let handle_width = self.handle_symbol.width().max(1);

        // Calculate how many columns should be filled based on percentage
        let filled_columns = (bar_width as f64 * percentage) as usize;

        // Render bar - track column position to ensure we fill exactly bar_width columns
        let mut current_x = area.x;
        let mut col = 0;

        while col < bar_width {
            let remaining_cols = bar_width - col;

            // Determine which symbol to use based on current position
            let (symbol, color, symbol_width) = if col < filled_columns {
                (&self.filled_symbol, self.filled_color, filled_width)
            } else {
                (&self.empty_symbol, self.empty_color, empty_width)
            };

            // If this symbol would exceed the bar width, fill remaining space
            if symbol_width > remaining_cols {
                for _ in 0..remaining_cols {
                    buf.set_string(current_x, area.y, " ", Style::default());
                    current_x += 1;
                }
                break;
            }

            // Render the symbol
            buf.set_string(current_x, area.y, symbol, Style::default().fg(color));
            current_x += symbol_width as u16;
            col += symbol_width;
        }

        // Render handle if enabled
        if self.show_handle && bar_width > 0 {
            // Calculate the x position where the handle should be placed
            // This represents the transition point between filled and empty
            let mut handle_x = area.x;
            let mut accumulated_cols = 0;

            // Walk through to find where filled_columns falls
            while accumulated_cols < filled_columns && accumulated_cols < bar_width {
                let symbol_width = if accumulated_cols < filled_columns {
                    filled_width
                } else {
                    empty_width
                };

                // Stop if adding this symbol would overshoot the target
                if accumulated_cols + symbol_width > filled_columns {
                    break;
                }

                handle_x += symbol_width as u16;
                accumulated_cols += symbol_width;
            }

            // Only render handle if it fits within the area
            if handle_x >= area.x && handle_x + handle_width as u16 <= area.x + area.width {
                buf.set_string(
                    handle_x,
                    area.y,
                    &self.handle_symbol,
                    Style::default().fg(self.handle_color),
                );
            }
        }
    }

    /// Renders a vertical slider
    fn render_vertical(&self, area: Rect, buf: &mut Buffer) {
        if area.height < 1 {
            return;
        }

        let percentage = self.percentage();
        let bar_height = area.height as usize;
        let filled_height = (bar_height as f64 * percentage) as usize;

        // For vertical, we don't need to worry as much about width since each row is independent
        // But we still render consistently
        for i in 0..bar_height {
            let y = area.y + area.height - 1 - i as u16;
            if y < area.y {
                break;
            }

            let (symbol, color) = if i < filled_height {
                (&self.filled_symbol, self.filled_color)
            } else {
                (&self.empty_symbol, self.empty_color)
            };

            buf.set_string(area.x, y, symbol, Style::default().fg(color));
        }

        // Render handle if enabled
        if self.show_handle && bar_height > 0 {
            let handle_y = area.y + area.height - 1 - (bar_height as f64 * percentage) as u16;
            if handle_y >= area.y && handle_y < area.y + area.height {
                buf.set_string(
                    area.x,
                    handle_y,
                    &self.handle_symbol,
                    Style::default().fg(self.handle_color),
                );
            }
        }
    }

    /// Renders label and value
    fn render_label_and_value(&self, area: Rect, buf: &mut Buffer) {
        if let Some(ref label) = self.label {
            let y = if self.orientation.is_horizontal() {
                area.y.saturating_sub(1)
            } else {
                area.y
            };

            if y >= buf.area.y && y < buf.area.y + buf.area.height {
                buf.set_string(area.x, y, label, Style::default());
            }
        }

        if self.show_value {
            let value_str = format!("{:.1}", self.value);

            let (x, y) = if self.orientation.is_horizontal() {
                (
                    area.x + area.width.saturating_sub(value_str.len() as u16),
                    area.y.saturating_sub(1),
                )
            } else {
                (area.x + 2, area.y + area.height)
            };

            if x >= buf.area.x
                && x < buf.area.x + buf.area.width
                && y >= buf.area.y
                && y < buf.area.y + buf.area.height
            {
                buf.set_string(x, y, &value_str, Style::default());
            }
        }
    }
}

impl<'a> Default for Slider<'a> {
    fn default() -> Self {
        Self::new(0.0, 0.0, 100.0)
    }
}

impl<'a> Widget for Slider<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref block) => {
                let inner = block.inner(area);
                block.clone().render(area, buf);
                inner
            }
            None => area,
        };

        if area.width == 0 || area.height == 0 {
            return;
        }

        // Render label and value if needed
        self.render_label_and_value(area, buf);

        // Render the slider based on orientation
        match self.orientation {
            SliderOrientation::Horizontal => self.render_horizontal(area, buf),
            SliderOrientation::Vertical => self.render_vertical(area, buf),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_new() {
        let slider = Slider::new(50.0, 0.0, 100.0);
        assert_eq!(slider.value, 50.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
    }

    #[test]
    fn test_slider_clamping() {
        let slider = Slider::new(150.0, 0.0, 100.0);
        assert_eq!(slider.value, 100.0);

        let slider = Slider::new(-50.0, 0.0, 100.0);
        assert_eq!(slider.value, 0.0);
    }

    #[test]
    fn test_slider_percentage() {
        let slider = Slider::new(50.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.5);

        let slider = Slider::new(25.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.25);

        let slider = Slider::new(0.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.0);

        let slider = Slider::new(100.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 1.0);
    }

    #[test]
    fn test_slider_builder() {
        let slider = Slider::default()
            .value(75.0)
            .min(0.0)
            .max(100.0)
            .label("Test")
            .show_value(true)
            .orientation(SliderOrientation::Vertical);

        assert_eq!(slider.value, 75.0);
        assert_eq!(slider.label, Some("Test".to_string()));
        assert!(slider.show_value);
        assert_eq!(slider.orientation, SliderOrientation::Vertical);
    }

    #[test]
    fn test_slider_from_state() {
        let state = SliderState::new(60.0, 0.0, 100.0);
        let slider = Slider::from_state(&state);
        assert_eq!(slider.value, 60.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
    }

    #[test]
    fn test_show_handle() {
        let slider = Slider::default().show_handle(true);
        assert!(slider.show_handle);

        let slider = Slider::default().show_handle(false);
        assert!(!slider.show_handle);
    }

    #[test]
    fn test_show_thumb_alias() {
        let slider = Slider::default().show_thumb(true);
        assert!(slider.show_handle);

        let slider = Slider::default().show_thumb(false);
        assert!(!slider.show_handle);
    }
}
