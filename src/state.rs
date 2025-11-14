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
    /// Step size for increment/decrement operations
    step: f64,
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
            step: 1.0, // Default step size
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

    /// Increases the value by the configured step size
    ///
    /// This is a convenience method that uses the step size set via `set_step()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_step(5.0);
    /// state.step_up();
    /// assert_eq!(state.value(), 55.0);
    /// ```
    pub fn step_up(&mut self) {
        self.increase(self.step);
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

    /// Decreases the value by the configured step size
    ///
    /// This is a convenience method that uses the step size set via `set_step()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_step(5.0);
    /// state.step_down();
    /// assert_eq!(state.value(), 45.0);
    /// ```
    pub fn step_down(&mut self) {
        self.decrease(self.step);
    }

    /// Gets the current step size
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.step(), 1.0);
    /// ```
    pub fn step(&self) -> f64 {
        self.step
    }

    /// Sets the step size for increment/decrement operations
    ///
    /// The step size determines how much the value changes when using
    /// `step_up()` and `step_down()` methods.
    ///
    /// # Arguments
    ///
    /// * `step` - The step size (must be positive)
    ///
    /// # Panics
    ///
    /// Panics if step is not positive (step <= 0.0)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_step(5.0);
    /// state.step_up();
    /// assert_eq!(state.value(), 55.0);
    ///
    /// state.set_step(10.0);
    /// state.step_up();
    /// assert_eq!(state.value(), 65.0);
    /// ```
    ///
    /// ```should_panic
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::new(50.0, 0.0, 100.0);
    /// state.set_step(-1.0); // Panics!
    /// ```
    pub fn set_step(&mut self, step: f64) {
        assert!(step > 0.0, "step must be positive");
        self.step = step;
    }

    /// Creates a new slider state with a custom step size
    ///
    /// # Arguments
    ///
    /// * `value` - Initial value
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    /// * `step` - Step size for increment/decrement operations
    ///
    /// # Panics
    ///
    /// Panics if min >= max or if step <= 0.0
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let mut state = SliderState::with_step(50.0, 0.0, 100.0, 5.0);
    /// assert_eq!(state.step(), 5.0);
    /// state.step_up();
    /// assert_eq!(state.value(), 55.0);
    /// ```
    pub fn with_step(value: f64, min: f64, max: f64, step: f64) -> Self {
        assert!(min < max, "min must be less than max");
        assert!(step > 0.0, "step must be positive");
        let clamped_value = value.clamp(min, max);
        Self {
            value: clamped_value,
            min,
            max,
            step,
        }
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

    /// Returns the range (max - min)
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.range(), 100.0);
    ///
    /// let state = SliderState::new(50.0, 25.0, 75.0);
    /// assert_eq!(state.range(), 50.0);
    /// ```
    pub fn range(&self) -> f64 {
        self.max - self.min
    }

    /// Returns true if the slider is at its minimum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(0.0, 0.0, 100.0);
    /// assert!(state.is_at_min());
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert!(!state.is_at_min());
    /// ```
    pub fn is_at_min(&self) -> bool {
        (self.value - self.min).abs() < f64::EPSILON
    }

    /// Returns true if the slider is at its maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(100.0, 0.0, 100.0);
    /// assert!(state.is_at_max());
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert!(!state.is_at_max());
    /// ```
    pub fn is_at_max(&self) -> bool {
        (self.value - self.max).abs() < f64::EPSILON
    }

    /// Returns true if the slider is at or near the middle of its range
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert!(state.is_at_middle());
    ///
    /// let state = SliderState::new(0.0, 0.0, 100.0);
    /// assert!(!state.is_at_middle());
    /// ```
    pub fn is_at_middle(&self) -> bool {
        let middle = (self.min + self.max) / 2.0;
        (self.value - middle).abs() < self.range() * 0.1
    }

    /// Returns true if the slider value is in the lower third of its range
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(20.0, 0.0, 100.0);
    /// assert!(state.is_low());
    ///
    /// let state = SliderState::new(80.0, 0.0, 100.0);
    /// assert!(!state.is_low());
    /// ```
    pub fn is_low(&self) -> bool {
        self.percentage() < 0.33
    }

    /// Returns true if the slider value is in the middle third of its range
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert!(state.is_medium());
    ///
    /// let state = SliderState::new(10.0, 0.0, 100.0);
    /// assert!(!state.is_medium());
    /// ```
    pub fn is_medium(&self) -> bool {
        let pct = self.percentage();
        (0.33..0.67).contains(&pct)
    }

    /// Returns true if the slider value is in the upper third of its range
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(80.0, 0.0, 100.0);
    /// assert!(state.is_high());
    ///
    /// let state = SliderState::new(20.0, 0.0, 100.0);
    /// assert!(!state.is_high());
    /// ```
    pub fn is_high(&self) -> bool {
        self.percentage() >= 0.67
    }

    /// Returns the distance from the minimum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(75.0, 0.0, 100.0);
    /// assert_eq!(state.distance_from_min(), 75.0);
    ///
    /// let state = SliderState::new(75.0, 25.0, 100.0);
    /// assert_eq!(state.distance_from_min(), 50.0);
    /// ```
    pub fn distance_from_min(&self) -> f64 {
        self.value - self.min
    }

    /// Returns the distance from the maximum value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(75.0, 0.0, 100.0);
    /// assert_eq!(state.distance_from_max(), 25.0);
    ///
    /// let state = SliderState::new(75.0, 25.0, 100.0);
    /// assert_eq!(state.distance_from_max(), 25.0);
    /// ```
    pub fn distance_from_max(&self) -> f64 {
        self.max - self.value
    }

    /// Returns a formatted string representation of the current value
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(75.5, 0.0, 100.0);
    /// assert_eq!(state.value_string(1), "75.5");
    /// assert_eq!(state.value_string(0), "76");
    /// ```
    pub fn value_string(&self, decimals: usize) -> String {
        format!("{:.decimals$}", self.value, decimals = decimals)
    }

    /// Returns a formatted percentage string (e.g., "75%")
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_slider::SliderState;
    ///
    /// let state = SliderState::new(75.0, 0.0, 100.0);
    /// assert_eq!(state.percentage_string(), "75%");
    ///
    /// let state = SliderState::new(50.0, 0.0, 100.0);
    /// assert_eq!(state.percentage_string(), "50%");
    /// ```
    pub fn percentage_string(&self) -> String {
        format!("{:.0}%", self.percentage() * 100.0)
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

    #[test]
    fn test_default_step() {
        let state = SliderState::new(50.0, 0.0, 100.0);
        assert_eq!(state.step(), 1.0);
    }

    #[test]
    fn test_set_step() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        state.set_step(5.0);
        assert_eq!(state.step(), 5.0);

        state.set_step(10.0);
        assert_eq!(state.step(), 10.0);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn test_invalid_step_negative() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        state.set_step(-1.0);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn test_invalid_step_zero() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        state.set_step(0.0);
    }

    #[test]
    fn test_step_up() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        state.set_step(5.0);

        state.step_up();
        assert_eq!(state.value(), 55.0);

        state.step_up();
        assert_eq!(state.value(), 60.0);
    }

    #[test]
    fn test_step_down() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);
        state.set_step(5.0);

        state.step_down();
        assert_eq!(state.value(), 45.0);

        state.step_down();
        assert_eq!(state.value(), 40.0);
    }

    #[test]
    fn test_step_up_clamping() {
        let mut state = SliderState::new(95.0, 0.0, 100.0);
        state.set_step(10.0);

        state.step_up();
        assert_eq!(state.value(), 100.0); // Clamped to max
    }

    #[test]
    fn test_step_down_clamping() {
        let mut state = SliderState::new(5.0, 0.0, 100.0);
        state.set_step(10.0);

        state.step_down();
        assert_eq!(state.value(), 0.0); // Clamped to min
    }

    #[test]
    fn test_with_step() {
        let state = SliderState::with_step(50.0, 0.0, 100.0, 5.0);
        assert_eq!(state.value(), 50.0);
        assert_eq!(state.min(), 0.0);
        assert_eq!(state.max(), 100.0);
        assert_eq!(state.step(), 5.0);
    }

    #[test]
    fn test_with_step_operations() {
        let mut state = SliderState::with_step(50.0, 0.0, 100.0, 2.5);

        state.step_up();
        assert_eq!(state.value(), 52.5);

        state.step_down();
        assert_eq!(state.value(), 50.0);
    }

    #[test]
    #[should_panic(expected = "step must be positive")]
    fn test_with_step_invalid() {
        SliderState::with_step(50.0, 0.0, 100.0, -1.0);
    }

    #[test]
    fn test_different_step_sizes() {
        let mut state = SliderState::new(50.0, 0.0, 100.0);

        // Step by 0.1
        state.set_step(0.1);
        state.step_up();
        assert!((state.value() - 50.1).abs() < 0.0001);

        // Step by 25
        state.set_value(50.0);
        state.set_step(25.0);
        state.step_up();
        assert_eq!(state.value(), 75.0);
    }
}
