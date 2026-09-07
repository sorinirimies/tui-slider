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

use crate::{
    orientation::SliderOrientation,
    position::{
        HorizontalBarAlignment, VerticalLabelPosition, VerticalValueAlignment,
        VerticalValuePosition,
    },
    state::SliderState,
    style::SliderStyle,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
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
    /// Alignment of the value display
    value_alignment: Alignment,
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
    /// Whether to leave a one-cell gap between bar symbols
    segmented: bool,
    /// Label position for vertical sliders
    vertical_label_position: VerticalLabelPosition,
    /// Value position for vertical sliders
    vertical_value_position: VerticalValuePosition,
    /// Value alignment for vertical sliders
    vertical_value_alignment: VerticalValueAlignment,
    /// Vertical alignment of the bar in horizontal sliders
    horizontal_bar_alignment: HorizontalBarAlignment,
}

impl<'a> Slider<'a> {
    fn assert_valid_bounds(min: f64, max: f64) {
        assert!(
            min.is_finite() && max.is_finite(),
            "min and max must be finite"
        );
        assert!(min <= max, "min must not exceed max");
    }

    fn assert_valid_value(value: f64) {
        assert!(!value.is_nan(), "value must not be NaN");
    }

    /// Creates a new slider with default settings
    ///
    /// **Note:** It's recommended to use [`Slider::from_state`] instead for better state management.
    ///
    /// # Arguments
    ///
    /// * `value` - Initial value (will be clamped to min..max)
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    ///
    /// # Panics
    ///
    /// Panics if `value` is NaN, either bound is non-finite, or `min > max`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderState};
    ///
    /// // Recommended approach using from_state
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// let slider = Slider::from_state(&state);
    ///
    /// // Or directly (less flexible)
    /// let slider = Slider::new(50.0, 0.0, 100.0);
    /// ```
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        Self::assert_valid_bounds(min, max);
        Self::assert_valid_value(value);
        Self {
            block: None,
            orientation: SliderOrientation::Horizontal,
            value: value.clamp(min, max),
            min,
            max,
            label: None,
            show_value: false,
            value_alignment: Alignment::Right,
            filled_symbol: "━".to_string(),
            empty_symbol: "─".to_string(),
            handle_symbol: "●".to_string(),
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            show_handle: true,
            segmented: false,
            vertical_label_position: VerticalLabelPosition::default(),
            vertical_value_position: VerticalValuePosition::default(),
            vertical_value_alignment: VerticalValueAlignment::default(),
            horizontal_bar_alignment: HorizontalBarAlignment::default(),
        }
    }

    /// Creates a slider from a state (recommended approach)
    ///
    /// This is the preferred way to create a slider as it works seamlessly with
    /// [`SliderState`] for managing value changes and keeping UI in sync.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderState, SliderOrientation};
    ///
    /// // Create a slider state
    /// let mut state = SliderState::new(75.0, 0.0, 100.0);
    ///
    /// // Create slider from state
    /// let slider = Slider::from_state(&state)
    ///     .orientation(SliderOrientation::Horizontal)
    ///     .label("Volume")
    ///     .show_value(true);
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
    /// # Panics
    ///
    /// Panics if `value` is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().value(75.0);
    /// ```
    pub fn value(mut self, value: f64) -> Self {
        Self::assert_valid_value(value);
        self.value = value.clamp(self.min, self.max);
        self
    }

    /// Sets the minimum value
    ///
    /// # Panics
    ///
    /// Panics if `min` is non-finite or exceeds the current maximum.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().min(0.0).max(100.0);
    /// ```
    pub fn min(mut self, min: f64) -> Self {
        Self::assert_valid_bounds(min, self.max);
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Sets the maximum value
    ///
    /// # Panics
    ///
    /// Panics if `max` is non-finite or is below the current minimum.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default().min(0.0).max(100.0);
    /// ```
    pub fn max(mut self, max: f64) -> Self {
        Self::assert_valid_bounds(self.min, max);
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    /// Sets the label text displayed above the slider
    ///
    /// The render area describes the bar itself. Reserve one buffer row above
    /// horizontal sliders when displaying a label or value.
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

    /// Sets the alignment for the value display
    ///
    /// # Examples
    ///
    /// ```
    /// use ratatui::layout::Alignment;
    /// use tui_slider::Slider;
    ///
    /// let slider = Slider::default()
    ///     .show_value(true)
    ///     .value_alignment(Alignment::Center);
    /// ```
    pub fn value_alignment(mut self, alignment: Alignment) -> Self {
        self.value_alignment = alignment;
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

    /// Applies a reusable [`SliderStyle`].
    pub fn style(mut self, style: &SliderStyle) -> Self {
        self.filled_symbol = style.filled_symbol.to_string();
        self.empty_symbol = style.empty_symbol.to_string();
        self.handle_symbol = style.handle_symbol.to_string();
        self.filled_color = style.filled_color;
        self.empty_color = style.empty_color;
        self.handle_color = style.handle_color;
        self.segmented = style.segmented;
        self
    }

    /// Enables or disables one-cell gaps between rendered bar symbols.
    pub fn segmented(mut self, segmented: bool) -> Self {
        self.segmented = segmented;
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

    /// Sets the label position for vertical sliders
    ///
    /// For vertical sliders, the label can be positioned at the top or bottom.
    /// This setting only affects vertical sliders; horizontal sliders ignore this.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderOrientation, VerticalLabelPosition};
    ///
    /// let slider = Slider::default()
    ///     .orientation(SliderOrientation::Vertical)
    ///     .label("Volume")
    ///     .vertical_label_position(VerticalLabelPosition::Bottom);
    /// ```
    pub fn vertical_label_position(mut self, position: VerticalLabelPosition) -> Self {
        self.vertical_label_position = position;
        self
    }

    /// Sets the value position for vertical sliders
    ///
    /// For vertical sliders, the numeric value can be positioned at the top, middle, or bottom.
    /// This setting only affects vertical sliders; horizontal sliders ignore this.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderOrientation, VerticalValuePosition};
    ///
    /// let slider = Slider::default()
    ///     .orientation(SliderOrientation::Vertical)
    ///     .show_value(true)
    ///     .vertical_value_position(VerticalValuePosition::Top);
    /// ```
    pub fn vertical_value_position(mut self, position: VerticalValuePosition) -> Self {
        self.vertical_value_position = position;
        self
    }

    /// Sets the value alignment for vertical sliders
    ///
    /// For vertical sliders, the numeric value can be aligned left, center, or right.
    /// This setting only affects vertical sliders; horizontal sliders use `value_alignment`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderOrientation, VerticalValueAlignment};
    ///
    /// let slider = Slider::default()
    ///     .orientation(SliderOrientation::Vertical)
    ///     .show_value(true)
    ///     .vertical_value_alignment(VerticalValueAlignment::Left);
    /// ```
    pub fn vertical_value_alignment(mut self, alignment: VerticalValueAlignment) -> Self {
        self.vertical_value_alignment = alignment;
        self
    }

    /// Sets the vertical alignment of the bar in horizontal sliders
    ///
    /// For horizontal sliders, the bar can be positioned at the top, center (middle),
    /// or bottom of the available area. This setting only affects horizontal sliders.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::{Slider, SliderOrientation, HorizontalBarAlignment};
    ///
    /// let slider = Slider::default()
    ///     .orientation(SliderOrientation::Horizontal)
    ///     .horizontal_bar_alignment(HorizontalBarAlignment::Top);
    /// ```
    pub fn horizontal_bar_alignment(mut self, alignment: HorizontalBarAlignment) -> Self {
        self.horizontal_bar_alignment = alignment;
        self
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

        // Calculate the Y position for the bar based on horizontal_bar_alignment
        let bar_y = match self.horizontal_bar_alignment {
            HorizontalBarAlignment::Top => area.y,
            HorizontalBarAlignment::Center => area.y + (area.height / 2),
            HorizontalBarAlignment::Bottom => area.y + area.height.saturating_sub(1),
        };

        // Render bar - track column position to ensure we fill exactly bar_width columns
        let mut current_x = area.x;
        let mut col = 0;
        let mut render_gap = false;

        while col < bar_width {
            if self.segmented && render_gap {
                buf.set_string(current_x, bar_y, " ", Style::default());
                current_x += 1;
                col += 1;
                render_gap = false;
                continue;
            }

            let remaining_cols = bar_width - col;
            let (symbol, color, symbol_width) = if col < filled_columns {
                (&self.filled_symbol, self.filled_color, filled_width)
            } else {
                (&self.empty_symbol, self.empty_color, empty_width)
            };

            if symbol_width > remaining_cols {
                for _ in 0..remaining_cols {
                    buf.set_string(current_x, bar_y, " ", Style::default());
                    current_x += 1;
                }
                break;
            }

            buf.set_string(current_x, bar_y, symbol, Style::default().fg(color));
            current_x += symbol_width as u16;
            col += symbol_width;
            render_gap = self.segmented;
        }

        if self.show_handle && bar_width > 0 {
            let handle_width = handle_width.min(bar_width);
            let travel = bar_width - handle_width;
            let handle_offset = (travel as f64 * percentage).round() as u16;
            let handle_x = area.x + handle_offset;

            buf.set_stringn(
                handle_x,
                bar_y,
                &self.handle_symbol,
                handle_width,
                Style::default().fg(self.handle_color),
            );
        }
    }

    /// Renders a vertical slider
    fn render_vertical(&self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let percentage = self.percentage();
        let bar_height = area.height as usize;
        let filled_rows = (bar_height as f64 * percentage) as usize;

        // Terminal symbols always consume one row. Unicode display width only
        // affects horizontal centering and clipping, never vertical progress.
        let max_symbol_width = self
            .filled_symbol
            .width()
            .max(self.empty_symbol.width())
            .max(self.handle_symbol.width())
            .max(1)
            .min(area.width as usize) as u16;
        let base_x = area.x + (area.width - max_symbol_width) / 2;
        let bottom_y = area.y + area.height - 1;

        for row in 0..bar_height {
            let y = bottom_y - row as u16;
            if self.segmented && row % 2 == 1 {
                for x in base_x..base_x + max_symbol_width {
                    buf.set_string(x, y, " ", Style::default());
                }
                continue;
            }

            let (symbol, color) = if row < filled_rows {
                (&self.filled_symbol, self.filled_color)
            } else {
                (&self.empty_symbol, self.empty_color)
            };
            let symbol_width = symbol.width().min(max_symbol_width as usize) as u16;
            let symbol_x = base_x + (max_symbol_width - symbol_width) / 2;

            buf.set_stringn(
                symbol_x,
                y,
                symbol,
                max_symbol_width as usize,
                Style::default().fg(color),
            );
        }

        if self.show_handle {
            // 0% maps to the bottom row; 100% maps to the top row.
            let handle_offset = filled_rows.min(bar_height - 1) as u16;
            let handle_y = bottom_y - handle_offset;
            let handle_width = self
                .handle_symbol
                .width()
                .max(1)
                .min(max_symbol_width as usize) as u16;
            let handle_x = base_x + (max_symbol_width - handle_width) / 2;

            buf.set_stringn(
                handle_x,
                handle_y,
                &self.handle_symbol,
                handle_width as usize,
                Style::default().fg(self.handle_color),
            );
        }
    }

    /// Renders label and value for horizontal sliders
    fn render_label_and_value(&self, area: Rect, buf: &mut Buffer) {
        // This is only used for horizontal sliders now
        let label_info = self.calculate_label_info(area);
        let value_info = self.calculate_value_info(area, true);
        let (label_x, value_x) = self.resolve_positions(area, true, &label_info, &value_info);
        self.render_label(buf, area, true, label_x);
        self.render_value(buf, area, true, value_x, value_info);
    }

    /// Renders label and value for vertical sliders with positioning options
    fn render_vertical_label_and_value(&self, area: Rect, buf: &mut Buffer) {
        // Check if label and value will be on the same side
        let label_at_top =
            self.label.is_some() && self.vertical_label_position == VerticalLabelPosition::Top;
        let label_at_bottom =
            self.label.is_some() && self.vertical_label_position == VerticalLabelPosition::Bottom;
        let value_at_top =
            self.show_value && self.vertical_value_position == VerticalValuePosition::Top;
        let value_at_bottom =
            self.show_value && self.vertical_value_position == VerticalValuePosition::Bottom;

        let both_at_top = label_at_top && value_at_top;
        let both_at_bottom = label_at_bottom && value_at_bottom;

        // Render label if present
        if let Some(ref label) = self.label {
            let label_y = match self.vertical_label_position {
                VerticalLabelPosition::Top => {
                    if both_at_top {
                        // Stack label above value at top
                        area.y.saturating_sub(2)
                    } else {
                        area.y.saturating_sub(1)
                    }
                }
                VerticalLabelPosition::Bottom => {
                    if both_at_bottom {
                        // Stack label below value at bottom
                        area.y + area.height + 1
                    } else {
                        area.y + area.height
                    }
                }
            };

            // Center the label horizontally
            let label_width = label.width() as u16;
            let label_x = area.x + (area.width.saturating_sub(label_width)) / 2;
            self.render_text_clipped(buf, area, label_x, label_y, label);
        }

        // Render value if enabled
        if self.show_value {
            let value_str = format!("{:.0}", self.value);
            let value_width = value_str.len() as u16;

            // Calculate Y position based on vertical position setting
            // When stacked with label, adjust position
            let value_y = match self.vertical_value_position {
                VerticalValuePosition::Top => {
                    if both_at_top {
                        // Value goes below label at top
                        area.y.saturating_sub(1)
                    } else {
                        area.y.saturating_sub(1)
                    }
                }
                VerticalValuePosition::Middle => area.y + area.height / 2,
                VerticalValuePosition::Bottom => {
                    if both_at_bottom {
                        // Value goes above label at bottom
                        area.y + area.height
                    } else {
                        area.y + area.height
                    }
                }
            };

            // Calculate X position based on alignment setting
            let value_x = match self.vertical_value_alignment {
                VerticalValueAlignment::Left => area.x,
                VerticalValueAlignment::Center => {
                    area.x + (area.width.saturating_sub(value_width)) / 2
                }
                VerticalValueAlignment::Right => area.x + area.width.saturating_sub(value_width),
            };
            self.render_text_clipped(buf, area, value_x, value_y, &value_str);
        }
    }

    fn calculate_label_info(&self, area: Rect) -> Option<(u16, u16)> {
        self.label.as_ref().map(|label| {
            let label_width = label.width() as u16;
            (area.x, label_width)
        })
    }

    fn calculate_value_info(&self, area: Rect, is_horizontal: bool) -> Option<(u16, u16, String)> {
        if !self.show_value {
            return None;
        }

        let value_str = format!("{:.1}", self.value);
        let value_width = value_str.len() as u16;

        let x_pos = if is_horizontal {
            // If we have a label and value alignment is Left, add spacing after the label
            if self.value_alignment == Alignment::Left && self.label.is_some() {
                let label_width = self.label.as_ref().map(|l| l.width() as u16).unwrap_or(0);
                let spacing = 2;
                area.x + label_width + spacing
            } else {
                self.calculate_horizontal_value_position(area, value_width)
            }
        } else {
            area.x + 2
        };

        Some((x_pos, value_width, value_str))
    }

    fn calculate_horizontal_value_position(&self, area: Rect, value_width: u16) -> u16 {
        match self.value_alignment {
            Alignment::Left => area.x,
            Alignment::Center => area.x + (area.width.saturating_sub(value_width)) / 2,
            Alignment::Right => area.x + area.width.saturating_sub(value_width),
        }
    }

    fn resolve_positions(
        &self,
        area: Rect,
        is_horizontal: bool,
        label_info: &Option<(u16, u16)>,
        value_info: &Option<(u16, u16, String)>,
    ) -> (u16, u16) {
        match (label_info, value_info) {
            (Some((label_x, label_w)), Some((value_x, value_w, _))) => {
                if is_horizontal && self.has_overlap(*label_x, *label_w, *value_x, *value_w) {
                    self.adjust_for_overlap(area, *label_x, *label_w, *value_w)
                } else {
                    (*label_x, *value_x)
                }
            }
            _ => (
                label_info.map(|(x, _)| x).unwrap_or(area.x),
                value_info.as_ref().map(|(x, _, _)| *x).unwrap_or(area.x),
            ),
        }
    }

    fn has_overlap(&self, label_x: u16, label_w: u16, value_x: u16, value_w: u16) -> bool {
        let label_end = label_x + label_w;
        let value_end = value_x + value_w;
        !(label_end <= value_x || value_end <= label_x)
    }

    fn adjust_for_overlap(
        &self,
        area: Rect,
        label_x: u16,
        label_w: u16,
        value_w: u16,
    ) -> (u16, u16) {
        match self.value_alignment {
            Alignment::Center | Alignment::Right => {
                // Keep label on left, move value to right
                (label_x, area.x + area.width.saturating_sub(value_w))
            }
            Alignment::Left => {
                // Try to add spacing between label and value
                let spacing = 2;
                let label_end = label_x + label_w;

                if label_end + spacing + value_w <= area.x + area.width {
                    (label_x, label_end + spacing)
                } else {
                    // Not enough space, put value on right edge
                    (label_x, area.x + area.width.saturating_sub(value_w))
                }
            }
        }
    }

    /// Renders the label text at the specified position
    fn render_label(&self, buf: &mut Buffer, area: Rect, is_horizontal: bool, label_x: u16) {
        if let Some(ref label) = self.label {
            let label_y = if is_horizontal {
                area.y.saturating_sub(1)
            } else {
                area.y
            };
            self.render_text_clipped(buf, area, label_x, label_y, label);
        }
    }

    /// Renders the value text at the specified position
    fn render_value(
        &self,
        buf: &mut Buffer,
        area: Rect,
        is_horizontal: bool,
        value_x: u16,
        value_info: Option<(u16, u16, String)>,
    ) {
        if let Some((_, _, value_str)) = value_info {
            let value_y = if is_horizontal {
                area.y.saturating_sub(1)
            } else {
                area.y.saturating_add(area.height)
            };
            self.render_text_clipped(buf, area, value_x, value_y, &value_str);
        }
    }

    fn render_text_clipped(&self, buf: &mut Buffer, area: Rect, x: u16, y: u16, text: &str) {
        if !self.is_within_buffer(buf, x, y) {
            return;
        }

        let area_right = area.x.saturating_add(area.width);
        let buffer_right = buf.area.x.saturating_add(buf.area.width);
        let max_width = area_right.min(buffer_right).saturating_sub(x) as usize;
        if max_width > 0 {
            buf.set_stringn(x, y, text, max_width, Style::default());
        }
    }

    fn is_within_buffer(&self, buf: &Buffer, x: u16, y: u16) -> bool {
        x >= buf.area.x
            && x < buf.area.x.saturating_add(buf.area.width)
            && y >= buf.area.y
            && y < buf.area.y.saturating_add(buf.area.height)
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

        // Render the bar first so labels and values remain legible when a
        // configured position intentionally overlaps the bar or block border.
        match self.orientation {
            SliderOrientation::Horizontal => self.render_horizontal(area, buf),
            SliderOrientation::Vertical => self.render_vertical(area, buf),
        }

        match self.orientation {
            SliderOrientation::Horizontal => self.render_label_and_value(area, buf),
            SliderOrientation::Vertical => self.render_vertical_label_and_value(area, buf),
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
        let slider = Slider::default().show_thumb(false);
        assert!(!slider.show_handle);

        let slider = Slider::default().show_thumb(true);
        assert!(slider.show_handle);
    }

    #[test]
    fn test_value_alignment() {
        use ratatui::layout::Alignment;

        let slider = Slider::default().value_alignment(Alignment::Left);
        assert_eq!(slider.value_alignment, Alignment::Left);

        let slider = Slider::default().value_alignment(Alignment::Center);
        assert_eq!(slider.value_alignment, Alignment::Center);

        let slider = Slider::default().value_alignment(Alignment::Right);
        assert_eq!(slider.value_alignment, Alignment::Right);
    }

    #[test]
    fn test_colors() {
        let slider = Slider::default()
            .filled_color(Color::Red)
            .empty_color(Color::Blue)
            .handle_color(Color::Green);

        assert_eq!(slider.filled_color, Color::Red);
        assert_eq!(slider.empty_color, Color::Blue);
        assert_eq!(slider.handle_color, Color::Green);
    }

    #[test]
    fn test_symbols() {
        let slider = Slider::default()
            .filled_symbol("█")
            .empty_symbol("░")
            .handle_symbol("▐");

        assert_eq!(slider.filled_symbol, "█");
        assert_eq!(slider.empty_symbol, "░");
        assert_eq!(slider.handle_symbol, "▐");
    }

    #[test]
    fn test_apply_style() {
        let style = SliderStyle::segmented_blocks();
        let slider = Slider::default().style(&style);

        assert_eq!(slider.filled_symbol, style.filled_symbol);
        assert_eq!(slider.empty_symbol, style.empty_symbol);
        assert_eq!(slider.handle_symbol, style.handle_symbol);
        assert_eq!(slider.filled_color, style.filled_color);
        assert_eq!(slider.empty_color, style.empty_color);
        assert_eq!(slider.handle_color, style.handle_color);
        assert!(slider.segmented);
    }

    #[test]
    fn test_render_segmented_style_leaves_gaps() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 5, 1);
        let mut buf = Buffer::empty(area);
        Slider::new(100.0, 0.0, 100.0)
            .style(&SliderStyle::segmented_blocks())
            .show_handle(false)
            .render(area, &mut buf);

        assert_eq!(buf[(0, 0)].symbol(), "█");
        assert_eq!(buf[(1, 0)].symbol(), " ");
        assert_eq!(buf[(2, 0)].symbol(), "█");
        assert_eq!(buf[(3, 0)].symbol(), " ");
        assert_eq!(buf[(4, 0)].symbol(), "█");
    }

    #[test]
    fn test_render_vertical_segmented_style_leaves_gaps() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 1, 5);
        let mut buf = Buffer::filled(area, ratatui::buffer::Cell::new("x"));
        Slider::new(100.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .style(&SliderStyle::segmented_blocks())
            .show_handle(false)
            .render(area, &mut buf);

        assert_eq!(buf[(0, 4)].symbol(), "█");
        assert_eq!(buf[(0, 3)].symbol(), " ");
        assert_eq!(buf[(0, 2)].symbol(), "█");
        assert_eq!(buf[(0, 1)].symbol(), " ");
        assert_eq!(buf[(0, 0)].symbol(), "█");
    }

    #[test]
    fn test_min_max_clamping() {
        let slider = Slider::default().min(10.0).max(90.0).value(100.0);
        assert_eq!(slider.value, 90.0);

        let slider = Slider::default().min(10.0).max(90.0).value(5.0);
        assert_eq!(slider.value, 10.0);

        let slider = Slider::default().min(10.0).max(90.0).value(50.0);
        assert_eq!(slider.value, 50.0);
    }

    #[test]
    fn test_label() {
        let slider = Slider::default().label("Volume");
        assert_eq!(slider.label, Some("Volume".to_string()));

        let slider = Slider::default();
        assert_eq!(slider.label, None);
    }

    #[test]
    fn test_show_value() {
        let slider = Slider::default().show_value(true);
        assert!(slider.show_value);

        let slider = Slider::default();
        assert!(!slider.show_value);
    }

    #[test]
    fn test_orientation() {
        let slider = Slider::default().orientation(SliderOrientation::Horizontal);
        assert_eq!(slider.orientation, SliderOrientation::Horizontal);

        let slider = Slider::default().orientation(SliderOrientation::Vertical);
        assert_eq!(slider.orientation, SliderOrientation::Vertical);
    }

    #[test]
    fn test_block() {
        use ratatui::widgets::{Block, Borders};

        let block = Block::default().borders(Borders::ALL);
        let slider = Slider::default().block(block);
        assert!(slider.block.is_some());

        let slider = Slider::default();
        assert!(slider.block.is_none());
    }

    #[test]
    fn test_percentage_calculation() {
        let slider = Slider::new(50.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.5);

        let slider = Slider::new(0.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.0);

        let slider = Slider::new(100.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 1.0);

        let slider = Slider::new(25.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.25);
    }

    #[test]
    fn test_default_values() {
        let slider = Slider::default();
        assert_eq!(slider.value, 0.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
        assert_eq!(slider.orientation, SliderOrientation::Horizontal);
        assert!(!slider.show_value);
        assert!(slider.show_handle);
        assert_eq!(slider.filled_symbol, "━");
        assert_eq!(slider.empty_symbol, "─");
        assert_eq!(slider.handle_symbol, "●");
    }

    #[test]
    fn test_chaining() {
        let slider = Slider::default()
            .value(75.0)
            .min(0.0)
            .max(100.0)
            .label("Test")
            .show_value(true)
            .value_alignment(ratatui::layout::Alignment::Center)
            .filled_symbol("█")
            .empty_symbol("░")
            .handle_symbol("▐")
            .filled_color(Color::Red)
            .empty_color(Color::Blue)
            .handle_color(Color::Green)
            .show_handle(true)
            .orientation(SliderOrientation::Vertical);

        assert_eq!(slider.value, 75.0);
        assert_eq!(slider.min, 0.0);
        assert_eq!(slider.max, 100.0);
        assert_eq!(slider.label, Some("Test".to_string()));
        assert!(slider.show_value);
        assert_eq!(slider.value_alignment, ratatui::layout::Alignment::Center);
        assert_eq!(slider.filled_symbol, "█");
        assert_eq!(slider.empty_symbol, "░");
        assert_eq!(slider.handle_symbol, "▐");
        assert_eq!(slider.filled_color, Color::Red);
        assert_eq!(slider.empty_color, Color::Blue);
        assert_eq!(slider.handle_color, Color::Green);
        assert!(slider.show_handle);
        assert_eq!(slider.orientation, SliderOrientation::Vertical);
    }

    #[test]
    fn test_vertical_positioning() {
        use crate::position::{
            VerticalLabelPosition, VerticalValueAlignment, VerticalValuePosition,
        };

        let slider = Slider::default()
            .orientation(SliderOrientation::Vertical)
            .vertical_label_position(VerticalLabelPosition::Bottom)
            .vertical_value_position(VerticalValuePosition::Top)
            .vertical_value_alignment(VerticalValueAlignment::Left);

        assert_eq!(
            slider.vertical_label_position,
            VerticalLabelPosition::Bottom
        );
        assert_eq!(slider.vertical_value_position, VerticalValuePosition::Top);
        assert_eq!(
            slider.vertical_value_alignment,
            VerticalValueAlignment::Left
        );
    }

    #[test]
    fn test_vertical_positioning_defaults() {
        use crate::position::{
            VerticalLabelPosition, VerticalValueAlignment, VerticalValuePosition,
        };

        let slider = Slider::default();

        assert_eq!(slider.vertical_label_position, VerticalLabelPosition::Top);
        assert_eq!(
            slider.vertical_value_position,
            VerticalValuePosition::Bottom
        );
        assert_eq!(
            slider.vertical_value_alignment,
            VerticalValueAlignment::Center
        );
    }

    #[test]
    fn test_from_state_preserves_values() {
        let state = SliderState::new(42.0, 10.0, 90.0);
        let slider = Slider::from_state(&state);
        assert_eq!(slider.value, 42.0);
        assert_eq!(slider.min, 10.0);
        assert_eq!(slider.max, 90.0);
    }

    #[test]
    fn test_vertical_rendering_consistency() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        // Create two sliders with different values but same configuration
        let state1 = SliderState::new(25.0, 0.0, 100.0);
        let state2 = SliderState::new(75.0, 0.0, 100.0);

        let slider1 = Slider::from_state(&state1)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("│")
            .empty_symbol("│")
            .handle_symbol("━");

        let slider2 = Slider::from_state(&state2)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("│")
            .empty_symbol("│")
            .handle_symbol("━");

        // Render both sliders in same-sized areas
        let area = Rect::new(0, 0, 5, 20);
        let mut buf1 = Buffer::empty(area);
        let mut buf2 = Buffer::empty(area);

        slider1.render(area, &mut buf1);
        slider2.render(area, &mut buf2);

        // Both should render in the full area height
        // Count non-empty cells to verify rendering happened
        let count1 = (0..area.height)
            .filter(|y| {
                let cell = &buf1[(area.x + area.width / 2, area.y + y)];
                !cell.symbol().trim().is_empty()
            })
            .count();

        let count2 = (0..area.height)
            .filter(|y| {
                let cell = &buf2[(area.x + area.width / 2, area.y + y)];
                !cell.symbol().trim().is_empty()
            })
            .count();

        // Both should have similar number of rendered symbols (within reasonable range)
        assert!(count1 > 0, "Slider 1 should render symbols");
        assert!(count2 > 0, "Slider 2 should render symbols");
        assert_eq!(
            count1 + count2,
            area.height as usize * 2,
            "Both sliders should fill the same height"
        );
    }

    // ── Rendering: horizontal slider ─────────────────────────────────────────

    #[test]
    fn test_render_horizontal_at_zero() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(0.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false);
        slider.render(area, &mut buf);

        // At 0% all cells should be empty symbol
        let cell = &buf[(0, 0)];
        assert_eq!(cell.symbol(), "─");
    }

    #[test]
    fn test_render_horizontal_at_full() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(100.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false);
        slider.render(area, &mut buf);

        // At 100% all cells should be filled symbol
        let cell = &buf[(0, 0)];
        assert_eq!(cell.symbol(), "━");
    }

    #[test]
    fn test_render_horizontal_at_half() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false);
        slider.render(area, &mut buf);

        // First cell filled, last cell empty
        assert_eq!(buf[(0, 0)].symbol(), "━");
        assert_eq!(buf[(19, 0)].symbol(), "─");
    }

    #[test]
    fn test_render_horizontal_with_handle() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 20, 1);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .handle_symbol("●")
            .show_handle(true);
        slider.render(area, &mut buf);

        // The buffer should contain a handle symbol somewhere
        let has_handle = (0..20u16).any(|x| buf[(x, 0)].symbol() == "●");
        assert!(has_handle, "handle symbol should be rendered");
    }

    #[test]
    fn test_render_horizontal_handle_at_max_stays_visible() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 4));
        let area = Rect::new(2, 2, 5, 1);
        Slider::new(100.0, 0.0, 100.0)
            .filled_symbol("-")
            .handle_symbol("H")
            .render(area, &mut buf);

        assert_eq!(buf[(6, 2)].symbol(), "H");
    }

    #[test]
    fn test_render_horizontal_zero_width_no_panic() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        // Zero-width area must not panic
        let area = Rect::new(0, 0, 0, 1);
        let mut buf = Buffer::empty(Rect::new(0, 0, 1, 1));
        let slider = Slider::new(50.0, 0.0, 100.0);
        slider.render(area, &mut buf);
    }

    #[test]
    fn test_render_horizontal_zero_height_no_panic() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 20, 0);
        let mut buf = Buffer::empty(Rect::new(0, 0, 20, 1));
        let slider = Slider::new(50.0, 0.0, 100.0);
        slider.render(area, &mut buf);
    }

    // ── Rendering: bar alignment ──────────────────────────────────────────────

    #[test]
    fn test_render_horizontal_bar_alignment_top() {
        use crate::position::HorizontalBarAlignment;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 10, 3);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(100.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false)
            .horizontal_bar_alignment(HorizontalBarAlignment::Top);
        slider.render(area, &mut buf);

        // Row 0 (top) should be filled
        assert_eq!(buf[(0, 0)].symbol(), "━");
        // Row 2 (bottom) should be empty
        assert_eq!(buf[(0, 2)].symbol(), " ");
    }

    #[test]
    fn test_render_horizontal_bar_alignment_bottom() {
        use crate::position::HorizontalBarAlignment;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 10, 3);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(100.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false)
            .horizontal_bar_alignment(HorizontalBarAlignment::Bottom);
        slider.render(area, &mut buf);

        // Row 2 (bottom) should be filled
        assert_eq!(buf[(0, 2)].symbol(), "━");
        // Row 0 (top) should be empty
        assert_eq!(buf[(0, 0)].symbol(), " ");
    }

    #[test]
    fn test_render_horizontal_bar_alignment_center() {
        use crate::position::HorizontalBarAlignment;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 10, 3);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(100.0, 0.0, 100.0)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false)
            .horizontal_bar_alignment(HorizontalBarAlignment::Center);
        slider.render(area, &mut buf);

        // Center row (row 1) should be filled
        assert_eq!(buf[(0, 1)].symbol(), "━");
    }

    // ── Rendering: vertical slider ────────────────────────────────────────────

    #[test]
    fn test_render_vertical_at_zero() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 3, 10);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(0.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("│")
            .empty_symbol("┊")
            .show_handle(false);
        slider.render(area, &mut buf);

        // At 0% the top row should be empty symbol
        let center_x = area.x + area.width / 2;
        assert_eq!(buf[(center_x, 0)].symbol(), "┊");
    }

    #[test]
    fn test_render_vertical_at_full() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 3, 10);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(100.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("│")
            .empty_symbol("┊")
            .show_handle(false);
        slider.render(area, &mut buf);

        // At 100% the bottom row should be filled symbol
        let center_x = area.x + area.width / 2;
        assert_eq!(buf[(center_x, 9)].symbol(), "│");
    }

    #[test]
    fn test_render_vertical_with_handle() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 3, 10);
        let mut buf = Buffer::empty(area);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("│")
            .empty_symbol("┊")
            .handle_symbol("━")
            .show_handle(true);
        slider.render(area, &mut buf);

        let center_x = area.x + area.width / 2;
        let has_handle = (0..10u16).any(|y| buf[(center_x, y)].symbol() == "━");
        assert!(has_handle, "vertical handle should be rendered");
    }

    #[test]
    fn test_render_vertical_handle_at_max_stays_visible_with_offset_area() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let area = Rect::new(2, 2, 3, 5);
        Slider::new(100.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("|")
            .handle_symbol("H")
            .render(area, &mut buf);

        assert_eq!(buf[(3, 2)].symbol(), "H");
    }

    #[test]
    fn test_render_vertical_wide_symbols_fill_every_row() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let mut buf = Buffer::empty(Rect::new(0, 0, 10, 10));
        let area = Rect::new(2, 2, 3, 4);
        Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol("界")
            .empty_symbol("界")
            .show_handle(false)
            .render(area, &mut buf);

        for y in area.y..area.y + area.height {
            assert_eq!(buf[(2, y)].symbol(), "界", "missing symbol on row {y}");
        }
    }

    #[test]
    fn test_render_vertical_zero_height_no_panic() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let area = Rect::new(0, 0, 3, 0);
        let mut buf = Buffer::empty(Rect::new(0, 0, 3, 1));
        let slider = Slider::new(50.0, 0.0, 100.0).orientation(SliderOrientation::Vertical);
        slider.render(area, &mut buf);
    }

    // ── Rendering: label and value ────────────────────────────────────────────

    #[test]
    fn test_render_horizontal_with_label() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        // Give extra height so label row (y-1 = row 0) sits inside the buffer
        let buf_area = Rect::new(0, 1, 20, 2);
        let mut buf = Buffer::empty(buf_area);
        let slider = Slider::new(50.0, 0.0, 100.0).label("Vol").show_value(false);
        let render_area = Rect::new(0, 2, 20, 1);
        slider.render(render_area, &mut buf);

        // Label "Vol" should appear on row 1 (one above render_area.y)
        let has_v = (0..20u16).any(|x| buf[(x, 1)].symbol() == "V");
        assert!(has_v, "label should be rendered above the slider");
    }

    #[test]
    fn test_render_horizontal_with_value() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 1, 20, 2);
        let mut buf = Buffer::empty(buf_area);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .show_value(true)
            .show_handle(false);
        let render_area = Rect::new(0, 2, 20, 1);
        slider.render(render_area, &mut buf);

        // The value "50.0" should appear somewhere on row 1
        let rendered: String = (0..20u16)
            .map(|x| buf[(x, 1)].symbol().to_string())
            .collect();
        assert!(
            rendered.contains("50") || rendered.contains('5'),
            "value should be rendered: got '{rendered}'"
        );
    }

    #[test]
    fn test_render_label_is_clipped_to_slider_width() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let mut buf = Buffer::empty(Rect::new(0, 0, 12, 4));
        let area = Rect::new(2, 2, 3, 1);
        Slider::default()
            .label("abcdef")
            .show_handle(false)
            .render(area, &mut buf);

        assert_eq!(buf[(2, 1)].symbol(), "a");
        assert_eq!(buf[(4, 1)].symbol(), "c");
        assert_eq!(buf[(5, 1)].symbol(), " ");
    }

    #[test]
    fn test_render_horizontal_with_label_and_value_left_aligned() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Alignment;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 1, 30, 2);
        let mut buf = Buffer::empty(buf_area);
        let slider = Slider::new(75.0, 0.0, 100.0)
            .label("Vol")
            .show_value(true)
            .value_alignment(Alignment::Left)
            .show_handle(false);
        let render_area = Rect::new(0, 2, 30, 1);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(0, 1)].symbol(), "V");
        assert_eq!(buf[(5, 1)].symbol(), "7");
    }

    #[test]
    fn test_render_horizontal_with_label_and_value_right_aligned() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Alignment;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 1, 30, 2);
        let mut buf = Buffer::empty(buf_area);
        let slider = Slider::new(75.0, 0.0, 100.0)
            .label("Vol")
            .show_value(true)
            .value_alignment(Alignment::Right)
            .show_handle(false);
        let render_area = Rect::new(0, 2, 30, 1);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(0, 1)].symbol(), "V");
        assert_eq!(buf[(26, 1)].symbol(), "7");
    }

    // ── Rendering: vertical label and value ───────────────────────────────────

    #[test]
    fn test_render_vertical_with_label_top() {
        use crate::position::VerticalLabelPosition;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .label("V")
            .vertical_label_position(VerticalLabelPosition::Top)
            .show_value(false);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(2, 1)].symbol(), "V");
    }

    #[test]
    fn test_render_vertical_with_label_bottom() {
        use crate::position::VerticalLabelPosition;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .label("V")
            .vertical_label_position(VerticalLabelPosition::Bottom)
            .show_value(false);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(2, 12)].symbol(), "V");
    }

    #[test]
    fn test_render_vertical_with_value_top() {
        use crate::position::VerticalValuePosition;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Top);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(1, 1)].symbol(), "5");
    }

    #[test]
    fn test_render_vertical_with_value_middle() {
        use crate::position::VerticalValuePosition;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Middle);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(1, 7)].symbol(), "5");
        assert_eq!(buf[(2, 7)].symbol(), "0");
    }

    #[test]
    fn test_render_vertical_with_value_bottom() {
        use crate::position::VerticalValuePosition;
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Bottom);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(1, 12)].symbol(), "5");
    }

    #[test]
    fn test_render_vertical_label_and_value_both_top() {
        use crate::position::{VerticalLabelPosition, VerticalValuePosition};
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        // Both label and value at top exercises the stacking branch
        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 3, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .label("V")
            .vertical_label_position(VerticalLabelPosition::Top)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Top);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(2, 1)].symbol(), "V");
        assert_eq!(buf[(1, 2)].symbol(), "5");
    }

    #[test]
    fn test_render_vertical_label_and_value_both_bottom() {
        use crate::position::{VerticalLabelPosition, VerticalValuePosition};
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .label("V")
            .vertical_label_position(VerticalLabelPosition::Bottom)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Bottom);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(1, 12)].symbol(), "5");
        assert_eq!(buf[(2, 13)].symbol(), "V");
    }

    #[test]
    fn test_render_vertical_value_alignment_left() {
        use crate::position::{VerticalValueAlignment, VerticalValuePosition};
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Middle)
            .vertical_value_alignment(VerticalValueAlignment::Left);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(0, 7)].symbol(), "5");
        assert_eq!(buf[(1, 7)].symbol(), "0");
    }

    #[test]
    fn test_render_vertical_value_alignment_right() {
        use crate::position::{VerticalValueAlignment, VerticalValuePosition};
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;

        let buf_area = Rect::new(0, 0, 5, 15);
        let mut buf = Buffer::empty(buf_area);
        let render_area = Rect::new(0, 2, 5, 10);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .orientation(SliderOrientation::Vertical)
            .show_value(true)
            .vertical_value_position(VerticalValuePosition::Middle)
            .vertical_value_alignment(VerticalValueAlignment::Right);
        slider.render(render_area, &mut buf);

        assert_eq!(buf[(3, 7)].symbol(), "5");
        assert_eq!(buf[(4, 7)].symbol(), "0");
    }

    // ── Rendering: block ──────────────────────────────────────────────────────

    #[test]
    fn test_render_with_block() {
        use ratatui::buffer::Buffer;
        use ratatui::layout::Rect;
        use ratatui::widgets::{Block, Borders};

        let area = Rect::new(0, 0, 20, 3);
        let mut buf = Buffer::empty(area);
        let block = Block::default().borders(Borders::ALL);
        let slider = Slider::new(50.0, 0.0, 100.0)
            .block(block)
            .filled_symbol("━")
            .empty_symbol("─")
            .show_handle(false);
        slider.render(area, &mut buf);

        // Border corners should be present
        assert_eq!(buf[(0, 0)].symbol(), "┌");
        assert_eq!(buf[(19, 0)].symbol(), "┐");
        assert_eq!(buf[(0, 2)].symbol(), "└");
        assert_eq!(buf[(19, 2)].symbol(), "┘");
    }

    // ── Percentage edge cases ─────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "value must not be NaN")]
    fn test_slider_rejects_nan_value() {
        Slider::new(f64::NAN, 0.0, 100.0);
    }

    #[test]
    #[should_panic(expected = "min and max must be finite")]
    fn test_slider_rejects_infinite_bounds() {
        Slider::new(0.0, f64::NEG_INFINITY, 100.0);
    }

    #[test]
    fn test_percentage_equal_min_max() {
        // When min == max the percentage guard returns 0.0 without dividing by zero
        let slider = Slider::new(50.0, 50.0, 50.0);
        assert_eq!(slider.percentage(), 0.0);
    }

    #[test]
    fn test_percentage_clamped_above_max() {
        let slider = Slider::new(100.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 1.0);
    }

    #[test]
    fn test_percentage_clamped_below_min() {
        let slider = Slider::new(0.0, 0.0, 100.0);
        assert_eq!(slider.percentage(), 0.0);
    }

    // ── horizontal_bar_alignment builder ─────────────────────────────────────

    #[test]
    fn test_horizontal_bar_alignment_builder() {
        use crate::position::HorizontalBarAlignment;

        let slider = Slider::default().horizontal_bar_alignment(HorizontalBarAlignment::Top);
        assert_eq!(slider.horizontal_bar_alignment, HorizontalBarAlignment::Top);

        let slider = Slider::default().horizontal_bar_alignment(HorizontalBarAlignment::Center);
        assert_eq!(
            slider.horizontal_bar_alignment,
            HorizontalBarAlignment::Center
        );

        let slider = Slider::default().horizontal_bar_alignment(HorizontalBarAlignment::Bottom);
        assert_eq!(
            slider.horizontal_bar_alignment,
            HorizontalBarAlignment::Bottom
        );
    }
}
