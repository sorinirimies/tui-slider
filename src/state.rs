//! Slider state management module
//!
//! This module provides the state management for sliders, including value tracking
//! and bounds checking.
//!
//! # Overview
//!
//! The `SliderState` struct manages the value and bounds of a slider. It ensures
//! that values are always clamped within the specified min/max range.
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```
//! use tui_slider::SliderState;
//!
//! let mut state = SliderState::new(50.0, 0.0, 100.0);
//! assert_eq!(state.value(), 50.0);
//!
//! state.set_value(75.0);
//! assert_eq!(state.value(), 75.0);
//! ```
//!
//! ## Value Clamping
//!
//! ```
//! use tui_slider::SliderState;
//!
//! let mut state = SliderState::new(50.0, 0.0, 100.0);
//!
//! // Values are automatically clamped
//! state.set_value(150.0);
//! assert_eq!(state.value(), 100.0);
//!
//! state.set_value(-10.0);
//! assert_eq!(state.value(), 0.0);
//! ```
//!
//! ## Percentage Operations
//!
//! ```
//! use tui_slider::SliderState;
//!
//! let mut state = SliderState::new(50.0, 0.0, 100.0);
//!
//! // Get value as percentage (0.0 to 1.0)
//! assert_eq!(state.percentage(), 0.5);
//!
//! // Set value from percentage
//! state.set_percentage(0.75);
//! assert_eq!(state.value(), 75.0);
//! ```

/// State management for a slider widget
///
/// Manages the current value and min/max bounds. All values are automatically
/// clamped to stay within the specified range.
///
/// # Examples
///
/// ```
/// use tui_slider::SliderState;
///
/// let mut state = SliderState::new(50.0, 0.0, 100.0);
///
/// // Direct value manipulation
/// state.increase(10.0);
/// assert_eq!(state.value(), 60.0);
///
/// state.decrease(5.0);
/// assert_eq!(state.value(), 55.0);
/// ```
#[derive(Debug, Clone)]
pub struct SliderState {
    /// Current value of the slider
    value: f64,
    /// Minimum value
    min: f64,
    /// Maximum value
    max: f64,
}

impl SliderState {
    /// Creates a new slider state with the given value and bounds
    ///
    /// # Arguments
    ///
    /// * `value` - Initial value
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    ///
    /// # Panics
    ///
    /// Panics if min >= max
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.value(), 50.0);
    /// assert_eq!(state.min(), 0.0);
    /// assert_eq!(state.max(), 100.0);
    /// ```
    ///
    /// ```should_panic
    /// use tui_slider::SliderState;
    ///
    /// // This will panic because min >= max
    /// let state = SliderState::new(50.0, 100.0, 0.0);
    /// ```
    pub fn new(value: f64, min: f64, max: f64) -> Self {
        assert!(min < max, "min must be less than max");
        let clamped_value = value.clamp(min, max);
        Self {
            value: clamped_value,
            min,
            max,
        }
    }

    /// Gets the current value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(75.0, 0.0, 100.0);
    /// assert_eq!(state.value(), 75.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Sets the value (automatically clamped to min..max range)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_value(75.0);
    /// assert_eq!(state.value(), 75.0);
    ///
    /// // Values are clamped to the valid range
    /// state.set_value(150.0);
    /// assert_eq!(state.value(), 100.0);
    /// ```
    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Gets the minimum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.min(), 0.0);
    /// ```
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Gets the maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.max(), 100.0);
    /// ```
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Sets the minimum value
    ///
    /// # Panics
    ///
    /// Panics if the new min is >= max
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_min(-10.0);
    /// assert_eq!(state.min(), -10.0);
    /// ```
    pub fn set_min(&mut self, min: f64) {
        assert!(min < self.max, "min must be less than max");
        self.min = min;
        self.value = self.value.clamp(self.min, self.max);
    }

    /// Sets the maximum value
    ///
    /// # Panics
    ///
    /// Panics if the new max is <= min
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_max(200.0);
    /// assert_eq!(state.max(), 200.0);
    /// ```
    pub fn set_max(&mut self, max: f64) {
        assert!(max > self.min, "max must be greater than min");
        self.max = max;
        self.value = self.value.clamp(self.min, self.max);
    }

    /// Gets the value as a percentage (0.0 to 1.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.percentage(), 0.5);
    ///
    /// let state = SliderState::new(25.0, 0.0, 100.0);
    /// assert_eq!(state.percentage(), 0.25);
    /// ```
    pub fn percentage(&self) -> f64 {
        if (self.max - self.min).abs() < f64::EPSILON {
            return 0.0;
        }
        (self.value - self.min) / (self.max - self.min)
    }

    /// Sets the value from a percentage (0.0 to 1.0)
    ///
    /// The percentage is automatically clamped to the 0.0-1.0 range.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(0.0, 0.0, 100.0);
    /// state.set_percentage(0.75);
    /// assert_eq!(state.value(), 75.0);
    ///
    /// state.set_percentage(0.5);
    /// assert_eq!(state.value(), 50.0);
    /// ```
    pub fn set_percentage(&mut self, percentage: f64) {
        let clamped_percentage = percentage.clamp(0.0, 1.0);
        self.set_value(self.min + (self.max - self.min) * clamped_percentage);
    }

    /// Increases the value by a step
    ///
    /// The result is automatically clamped to the maximum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.increase(10.0);
    /// assert_eq!(state.value(), 60.0);
    ///
    /// // Won't exceed max
    /// state.increase(100.0);
    /// assert_eq!(state.value(), 100.0);
    /// ```
    pub fn increase(&mut self, step: f64) {
        self.set_value(self.value + step);
    }

    /// Decreases the value by a step
    ///
    /// The result is automatically clamped to the minimum value.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.decrease(10.0);
    /// assert_eq!(state.value(), 40.0);
    ///
    /// // Won't go below min
    /// state.decrease(100.0);
    /// assert_eq!(state.value(), 0.0);
    /// ```
    pub fn decrease(&mut self, step: f64) {
        self.set_value(self.value - step);
    }

    /// Sets the value from a position within a given length
    ///
    /// # Arguments
    ///
    /// * `position` - Position in the slider (0 to length)
    /// * `length` - Total length of the slider
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(0.0, 0.0, 100.0);
    /// state.set_from_position(50, 100);
    /// assert_eq!(state.value(), 50.0);
    /// ```
    pub fn set_from_position(&mut self, position: u16, length: u16) {
        if length == 0 {
            return;
        }
        let percentage = position as f64 / length as f64;
        self.set_percentage(percentage);
    }

    /// Gets the position within a given length
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.position(100), 50);
    ///
    /// let state = SliderState::new(25.0, 0.0, 100.0);
    /// assert_eq!(state.position(100), 25);
    /// ```
    pub fn position(&self, length: u16) -> u16 {
        (self.percentage() * length as f64).round() as u16
    }
}

impl Default for SliderState {
    fn default() -> Self {
        Self::new(0.0, 0.0, 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_state() {
        let state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.value(), 50.0);
        assert_eq!(state.min(), 0.0);
        assert_eq!(state.max(), 100.0);
    }

    #[test]
    fn test_clamping() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);

        state.set_value(150.0);
        assert_eq!(state.value(), 100.0);

        state.set_value(-50.0);
        assert_eq!(state.value(), 0.0);
    }

    #[test]
    fn test_percentage() {
        let state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.5);

        let state = SliderState::new(25.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.25);

        let state = SliderState::new(0.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 0.0);

        let state = SliderState::new(100.0, 0.0, 100.0);
        assert_eq!(state.percentage(), 1.0);
    }

    #[test]
    fn test_set_percentage() {
        let mut state = SliderState::new(0.0, 0.0, 100.0);
        state.set_percentage(0.5);
        assert_eq!(state.value(), 50.0);

        state.set_percentage(0.25);
        assert_eq!(state.value(), 25.0);
    }

    #[test]
    fn test_increase_decrease() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);

        state.increase(10.0);
        assert_eq!(state.value(), 60.0);

        state.set_value(50.0); // Reset
        state.decrease(10.0);
        assert_eq!(state.value(), 40.0);
    }

    #[test]
    fn test_position() {
        let state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.position(100), 50);

        let state = SliderState::new(25.0, 0.0, 100.0);
        assert_eq!(state.position(100), 25);
    }

    #[test]
    fn test_set_from_position() {
        let mut state = SliderState::new(0.0, 0.0, 100.0);
        state.set_from_position(50, 100);
        assert_eq!(state.value(), 50.0);
    }

    #[test]
    #[should_panic(expected = "min must be less than max")]
    fn test_invalid_bounds() {
        SliderState::new(50.0, 100.0, 0.0);
    }
}
