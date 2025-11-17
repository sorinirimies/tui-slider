//! Position enums for label and value placement in sliders

/// Position of the label in a vertical slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalLabelPosition {
    /// Label at the top of the slider
    #[default]
    Top,
    /// Label at the bottom of the slider
    Bottom,
}

/// Vertical position of the value display in a vertical slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalValuePosition {
    /// Value at the top
    Top,
    /// Value at the middle
    Middle,
    /// Value at the bottom
    #[default]
    Bottom,
}

/// Horizontal alignment of the value text in a vertical slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalValueAlignment {
    /// Value aligned to the left
    Left,
    /// Value aligned to the center
    #[default]
    Center,
    /// Value aligned to the right
    Right,
}

impl VerticalValueAlignment {
    /// Convert to ratatui's Alignment
    pub fn to_ratatui_alignment(&self) -> ratatui::layout::Alignment {
        match self {
            VerticalValueAlignment::Left => ratatui::layout::Alignment::Left,
            VerticalValueAlignment::Center => ratatui::layout::Alignment::Center,
            VerticalValueAlignment::Right => ratatui::layout::Alignment::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_label_position_default() {
        assert_eq!(VerticalLabelPosition::default(), VerticalLabelPosition::Top);
    }

    #[test]
    fn test_vertical_value_position_default() {
        assert_eq!(
            VerticalValuePosition::default(),
            VerticalValuePosition::Bottom
        );
    }

    #[test]
    fn test_vertical_value_alignment_default() {
        assert_eq!(
            VerticalValueAlignment::default(),
            VerticalValueAlignment::Center
        );
    }

    #[test]
    fn test_vertical_value_alignment_conversion() {
        use ratatui::layout::Alignment;

        assert_eq!(
            VerticalValueAlignment::Left.to_ratatui_alignment(),
            Alignment::Left
        );
        assert_eq!(
            VerticalValueAlignment::Center.to_ratatui_alignment(),
            Alignment::Center
        );
        assert_eq!(
            VerticalValueAlignment::Right.to_ratatui_alignment(),
            Alignment::Right
        );
    }
}
