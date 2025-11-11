//! Slider orientation module
//!
//! This module defines the orientation options for sliders.

/// Orientation of the slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SliderOrientation {
    /// Horizontal slider (left to right)
    #[default]
    Horizontal,
    /// Vertical slider (bottom to top)
    Vertical,
}

impl SliderOrientation {
    /// Returns true if the orientation is horizontal
    pub fn is_horizontal(&self) -> bool {
        matches!(self, Self::Horizontal)
    }

    /// Returns true if the orientation is vertical
    pub fn is_vertical(&self) -> bool {
        matches!(self, Self::Vertical)
    }

    /// Toggles the orientation
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        };
    }

    /// Returns the opposite orientation
    pub fn opposite(&self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_horizontal() {
        assert!(SliderOrientation::Horizontal.is_horizontal());
        assert!(!SliderOrientation::Vertical.is_horizontal());
    }

    #[test]
    fn test_is_vertical() {
        assert!(SliderOrientation::Vertical.is_vertical());
        assert!(!SliderOrientation::Horizontal.is_vertical());
    }

    #[test]
    fn test_toggle() {
        let mut orientation = SliderOrientation::Horizontal;
        orientation.toggle();
        assert_eq!(orientation, SliderOrientation::Vertical);
        orientation.toggle();
        assert_eq!(orientation, SliderOrientation::Horizontal);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(
            SliderOrientation::Horizontal.opposite(),
            SliderOrientation::Vertical
        );
        assert_eq!(
            SliderOrientation::Vertical.opposite(),
            SliderOrientation::Horizontal
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(SliderOrientation::default(), SliderOrientation::Horizontal);
    }
}
