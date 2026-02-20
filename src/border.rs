//! Border styling module
//!
//! This module provides border style configurations for sliders, including
//! solid and segmented border variants, as well as title positioning utilities.
//!
//! # Examples
//!
//! ```rust
//! use tui_slider::border::{BorderStyle, BorderSet, TitleAlignment, TitlePosition};
//! use ratatui::widgets::{Block, Borders};
//!
//! // Get a border set for plain borders
//! let plain = BorderStyle::Plain.border_set();
//!
//! // Get a border set for rounded segmented borders
//! let rounded_segmented = BorderStyle::RoundedSegmented.border_set();
//!
//! // Create a centered title
//! let alignment = TitleAlignment::Center;
//! ```

/// Title alignment options for block borders
///
/// Specifies where the title should be positioned horizontally on the border.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleAlignment {
    /// Left-aligned title
    Left,
    /// Center-aligned title
    #[default]
    Center,
    /// Right-aligned title
    Right,
}

impl TitleAlignment {
    /// Convert to ratatui's Alignment
    pub fn to_ratatui_alignment(self) -> ratatui::layout::Alignment {
        match self {
            TitleAlignment::Left => ratatui::layout::Alignment::Left,
            TitleAlignment::Center => ratatui::layout::Alignment::Center,
            TitleAlignment::Right => ratatui::layout::Alignment::Right,
        }
    }
}

/// Title position options for block borders
///
/// Specifies where the title should be positioned on the border (top or bottom).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitlePosition {
    /// Title on the top border
    #[default]
    Top,
    /// Title on the bottom border
    Bottom,
}

/// Border style variants
///
/// This enum defines the available border styles, including both solid
/// and segmented (dashed/gapped) variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderStyle {
    /// Plain borders with straight lines (┌─┐)
    #[default]
    Plain,
    /// Rounded borders with smooth corners (╭─╮)
    Rounded,
    /// Double-line borders (╔═╗)
    Double,
    /// Thick borders with bold lines (┏━┓)
    Thick,
    /// Plain borders with gaps/segments (┌─ ─┐)
    PlainSegmented,
    /// Rounded borders with gaps/segments (╭─ ─╮)
    RoundedSegmented,
    /// Double-line borders with gaps/segments (╔═ ═╗)
    DoubleSegmented,
    /// Thick borders with gaps/segments (┏━ ━┓)
    ThickSegmented,
    /// Plain borders on sides only (left/right)
    PlainSidesOnly,
    /// Rounded borders on sides only (left/right)
    RoundedSidesOnly,
    /// Double-line borders on sides only (left/right)
    DoubleSidesOnly,
    /// Thick borders on sides only (left/right)
    ThickSidesOnly,
}

/// Border character set
///
/// Contains the characters used to render borders
#[derive(Debug, Clone, Copy)]
pub struct BorderSet {
    /// Top-left corner character
    pub top_left: char,
    /// Top-right corner character
    pub top_right: char,
    /// Bottom-left corner character
    pub bottom_left: char,
    /// Bottom-right corner character
    pub bottom_right: char,
    /// Vertical line character
    pub vertical: char,
    /// Horizontal line character
    pub horizontal: char,
    /// Whether this border style uses segments/gaps
    pub segmented: bool,
    /// Whether this border style only shows sides (left/right borders)
    pub sides_only: bool,
}

impl BorderStyle {
    /// Get the border character set for this style
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// let plain = BorderStyle::Plain.border_set();
    /// assert_eq!(plain.top_left, '┌');
    /// assert_eq!(plain.horizontal, '─');
    /// assert!(!plain.segmented);
    ///
    /// let plain_seg = BorderStyle::PlainSegmented.border_set();
    /// assert!(plain_seg.segmented);
    /// ```
    pub fn border_set(self) -> BorderSet {
        match self {
            BorderStyle::Plain | BorderStyle::PlainSegmented | BorderStyle::PlainSidesOnly => {
                BorderSet {
                    top_left: '┌',
                    top_right: '┐',
                    bottom_left: '└',
                    bottom_right: '┘',
                    vertical: '│',
                    horizontal: '─',
                    segmented: matches!(self, BorderStyle::PlainSegmented),
                    sides_only: matches!(self, BorderStyle::PlainSidesOnly),
                }
            }
            BorderStyle::Rounded
            | BorderStyle::RoundedSegmented
            | BorderStyle::RoundedSidesOnly => BorderSet {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                vertical: '│',
                horizontal: '─',
                segmented: matches!(self, BorderStyle::RoundedSegmented),
                sides_only: matches!(self, BorderStyle::RoundedSidesOnly),
            },
            BorderStyle::Double | BorderStyle::DoubleSegmented | BorderStyle::DoubleSidesOnly => {
                BorderSet {
                    top_left: '╔',
                    top_right: '╗',
                    bottom_left: '╚',
                    bottom_right: '╝',
                    vertical: '║',
                    horizontal: '═',
                    segmented: matches!(self, BorderStyle::DoubleSegmented),
                    sides_only: matches!(self, BorderStyle::DoubleSidesOnly),
                }
            }
            BorderStyle::Thick | BorderStyle::ThickSegmented | BorderStyle::ThickSidesOnly => {
                BorderSet {
                    top_left: '┏',
                    top_right: '┓',
                    bottom_left: '┗',
                    bottom_right: '┛',
                    vertical: '┃',
                    horizontal: '━',
                    segmented: matches!(self, BorderStyle::ThickSegmented),
                    sides_only: matches!(self, BorderStyle::ThickSidesOnly),
                }
            }
        }
    }

    /// Get the display name for this border style
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// assert_eq!(BorderStyle::Plain.name(), "Plain");
    /// assert_eq!(BorderStyle::RoundedSegmented.name(), "Rounded (Segmented)");
    /// ```
    pub fn name(self) -> &'static str {
        match self {
            BorderStyle::Plain => "Plain",
            BorderStyle::Rounded => "Rounded",
            BorderStyle::Double => "Double",
            BorderStyle::Thick => "Thick",
            BorderStyle::PlainSegmented => "Plain (Segmented)",
            BorderStyle::RoundedSegmented => "Rounded (Segmented)",
            BorderStyle::DoubleSegmented => "Double (Segmented)",
            BorderStyle::ThickSegmented => "Thick (Segmented)",
            BorderStyle::PlainSidesOnly => "Plain (Sides Only)",
            BorderStyle::RoundedSidesOnly => "Rounded (Sides Only)",
            BorderStyle::DoubleSidesOnly => "Double (Sides Only)",
            BorderStyle::ThickSidesOnly => "Thick (Sides Only)",
        }
    }

    /// Get a description for this border style
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// assert_eq!(BorderStyle::Plain.description(), "Basic straight lines");
    /// assert_eq!(BorderStyle::Thick.description(), "Bold thick borders");
    /// ```
    pub fn description(self) -> &'static str {
        match self {
            BorderStyle::Plain => "Basic straight lines",
            BorderStyle::Rounded => "Smooth rounded corners",
            BorderStyle::Double => "Elegant double lines",
            BorderStyle::Thick => "Bold thick borders",
            BorderStyle::PlainSegmented => "Dashed lines with gaps",
            BorderStyle::RoundedSegmented => "Rounded with gaps",
            BorderStyle::DoubleSegmented => "Double lines with gaps",
            BorderStyle::ThickSegmented => "Thick with gaps",
            BorderStyle::PlainSidesOnly => "Left and right borders only",
            BorderStyle::RoundedSidesOnly => "Rounded sides only",
            BorderStyle::DoubleSidesOnly => "Double sides only",
            BorderStyle::ThickSidesOnly => "Thick sides only",
        }
    }

    /// Check if this border style is segmented
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// assert!(!BorderStyle::Plain.is_segmented());
    /// assert!(BorderStyle::PlainSegmented.is_segmented());
    /// ```
    pub fn is_segmented(self) -> bool {
        matches!(
            self,
            BorderStyle::PlainSegmented
                | BorderStyle::RoundedSegmented
                | BorderStyle::DoubleSegmented
                | BorderStyle::ThickSegmented
        )
    }

    /// Check if this border style only shows sides
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// assert!(!BorderStyle::Plain.is_sides_only());
    /// assert!(BorderStyle::PlainSidesOnly.is_sides_only());
    /// ```
    pub fn is_sides_only(self) -> bool {
        matches!(
            self,
            BorderStyle::PlainSidesOnly
                | BorderStyle::RoundedSidesOnly
                | BorderStyle::DoubleSidesOnly
                | BorderStyle::ThickSidesOnly
        )
    }

    /// Get all border styles as a list
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::border::BorderStyle;
    ///
    /// let styles = BorderStyle::all();
    /// assert_eq!(styles.len(), 12);
    /// ```
    pub fn all() -> &'static [BorderStyle] {
        &[
            BorderStyle::Plain,
            BorderStyle::PlainSegmented,
            BorderStyle::PlainSidesOnly,
            BorderStyle::Rounded,
            BorderStyle::RoundedSegmented,
            BorderStyle::RoundedSidesOnly,
            BorderStyle::Double,
            BorderStyle::DoubleSegmented,
            BorderStyle::DoubleSidesOnly,
            BorderStyle::Thick,
            BorderStyle::ThickSegmented,
            BorderStyle::ThickSidesOnly,
        ]
    }
}

/// Create a segmented line with gaps
///
/// Creates a string with a pattern of characters and spaces (2 chars on, 1 space off)
///
/// # Arguments
///
/// * `length` - The total length of the line
/// * `char` - The character to use for the line segments
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::create_segmented_line;
///
/// let line = create_segmented_line(5, '─');
/// assert_eq!(line, "── ──");
/// ```
pub fn create_segmented_line(length: usize, char: char) -> String {
    let mut result = String::with_capacity(length);
    for i in 0..length {
        // Pattern: 2 chars on, 1 char off
        if (i % 3) == 2 {
            result.push(' ');
        } else {
            result.push(char);
        }
    }
    result
}

/// Helper function to create a title `Line` for ratatui Block
///
/// Returns a [`ratatui::text::Line`] with the requested alignment applied.
/// To place the title at the bottom of a block, use [`Block::title_bottom`] instead
/// of [`Block::title`] / [`Block::title_top`] when adding the returned line.
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::create_title;
/// use ratatui::widgets::Block;
///
/// let title = create_title("My Slider", None, None);
/// let block = Block::default().title_top(title);
/// ```
pub fn create_title(
    text: impl Into<String>,
    alignment: Option<TitleAlignment>,
    _position: Option<TitlePosition>,
) -> ratatui::text::Line<'static> {
    use ratatui::text::Line;

    let alignment = alignment.unwrap_or_default();
    let title_text = text.into();

    Line::from(title_text).alignment(alignment.to_ratatui_alignment())
}

/// Helper function to create a left-aligned title
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::title_left;
/// use ratatui::widgets::Block;
///
/// let title = title_left("Volume");
/// let block = Block::default().title_top(title);
/// ```
pub fn title_left(text: impl Into<String>) -> ratatui::text::Line<'static> {
    create_title(text, Some(TitleAlignment::Left), None)
}

/// Helper function to create a centered title
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::title_center;
/// use ratatui::widgets::Block;
///
/// let title = title_center("Volume");
/// let block = Block::default().title_top(title);
/// ```
pub fn title_center(text: impl Into<String>) -> ratatui::text::Line<'static> {
    create_title(text, Some(TitleAlignment::Center), None)
}

/// Helper function to create a right-aligned title for ratatui Block
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::title_right;
/// use ratatui::widgets::Block;
///
/// let title = title_right("100%");
/// let block = Block::default().title_top(title);
/// ```
pub fn title_right(text: impl Into<String>) -> ratatui::text::Line<'static> {
    create_title(text, Some(TitleAlignment::Right), None)
}

/// Helper function to create a right-aligned title with trailing spacing for ratatui Block
///
/// This variant adds trailing spaces to prevent collision with values that may be
/// rendered on the same line. Use this when you have both a title and a separate value display.
///
/// # Examples
///
/// ```rust
/// use tui_slider::border::title_right_with_spacing;
/// use ratatui::widgets::Block;
///
/// let title = title_right_with_spacing("Status");
/// let block = Block::default().title_top(title);
/// ```
pub fn title_right_with_spacing(text: impl Into<String>) -> ratatui::text::Line<'static> {
    let text_with_spacing = format!("{}     ", text.into());
    create_title(text_with_spacing, Some(TitleAlignment::Right), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_border_set_plain() {
        let set = BorderStyle::Plain.border_set();
        assert_eq!(set.top_left, '┌');
        assert_eq!(set.horizontal, '─');
        assert!(!set.segmented);
    }

    #[test]
    fn test_border_set_rounded() {
        let set = BorderStyle::Rounded.border_set();
        assert_eq!(set.top_left, '╭');
        assert_eq!(set.top_right, '╮');
        assert!(!set.segmented);
    }

    #[test]
    fn test_border_set_double() {
        let set = BorderStyle::Double.border_set();
        assert_eq!(set.top_left, '╔');
        assert_eq!(set.horizontal, '═');
        assert!(!set.segmented);
    }

    #[test]
    fn test_border_set_thick() {
        let set = BorderStyle::Thick.border_set();
        assert_eq!(set.top_left, '┏');
        assert_eq!(set.horizontal, '━');
        assert!(!set.segmented);
    }

    #[test]
    fn test_segmented_variants() {
        assert!(BorderStyle::PlainSegmented.is_segmented());
        assert!(BorderStyle::RoundedSegmented.is_segmented());
        assert!(BorderStyle::DoubleSegmented.is_segmented());
        assert!(BorderStyle::ThickSegmented.is_segmented());

        assert!(!BorderStyle::Plain.is_segmented());
        assert!(!BorderStyle::Rounded.is_segmented());
    }

    #[test]
    fn test_create_segmented_line() {
        let line = create_segmented_line(6, '-');
        assert_eq!(line, "-- -- ");

        let line = create_segmented_line(9, '=');
        assert_eq!(line, "== == == ");
    }

    #[test]
    fn test_all_styles() {
        let styles = BorderStyle::all();
        assert_eq!(styles.len(), 12);
    }

    #[test]
    fn test_sides_only_variants() {
        assert!(BorderStyle::PlainSidesOnly.is_sides_only());
        assert!(BorderStyle::RoundedSidesOnly.is_sides_only());
        assert!(BorderStyle::DoubleSidesOnly.is_sides_only());
        assert!(BorderStyle::ThickSidesOnly.is_sides_only());

        assert!(!BorderStyle::Plain.is_sides_only());
        assert!(!BorderStyle::PlainSegmented.is_sides_only());
    }

    #[test]
    fn test_names_and_descriptions() {
        assert_eq!(BorderStyle::Plain.name(), "Plain");
        assert_eq!(BorderStyle::Rounded.name(), "Rounded");
        assert_eq!(BorderStyle::Double.name(), "Double");
        assert_eq!(BorderStyle::Thick.name(), "Thick");
        assert_eq!(BorderStyle::PlainSegmented.name(), "Plain (Segmented)");
        assert_eq!(BorderStyle::RoundedSegmented.name(), "Rounded (Segmented)");
        assert_eq!(BorderStyle::DoubleSegmented.name(), "Double (Segmented)");
        assert_eq!(BorderStyle::ThickSegmented.name(), "Thick (Segmented)");
        assert_eq!(BorderStyle::PlainSidesOnly.name(), "Plain (Sides Only)");
        assert_eq!(BorderStyle::RoundedSidesOnly.name(), "Rounded (Sides Only)");
        assert_eq!(BorderStyle::DoubleSidesOnly.name(), "Double (Sides Only)");
        assert_eq!(BorderStyle::ThickSidesOnly.name(), "Thick (Sides Only)");
    }

    #[test]
    fn test_all_descriptions() {
        assert_eq!(BorderStyle::Plain.description(), "Basic straight lines");
        assert_eq!(BorderStyle::Rounded.description(), "Smooth rounded corners");
        assert_eq!(BorderStyle::Double.description(), "Elegant double lines");
        assert_eq!(BorderStyle::Thick.description(), "Bold thick borders");
        assert_eq!(
            BorderStyle::PlainSegmented.description(),
            "Dashed lines with gaps"
        );
        assert_eq!(
            BorderStyle::RoundedSegmented.description(),
            "Rounded with gaps"
        );
        assert_eq!(
            BorderStyle::DoubleSegmented.description(),
            "Double lines with gaps"
        );
        assert_eq!(BorderStyle::ThickSegmented.description(), "Thick with gaps");
        assert_eq!(
            BorderStyle::PlainSidesOnly.description(),
            "Left and right borders only"
        );
        assert_eq!(
            BorderStyle::RoundedSidesOnly.description(),
            "Rounded sides only"
        );
        assert_eq!(
            BorderStyle::DoubleSidesOnly.description(),
            "Double sides only"
        );
        assert_eq!(
            BorderStyle::ThickSidesOnly.description(),
            "Thick sides only"
        );
    }

    #[test]
    fn test_title_alignment() {
        assert_eq!(
            TitleAlignment::Left.to_ratatui_alignment(),
            ratatui::layout::Alignment::Left
        );
        assert_eq!(
            TitleAlignment::Center.to_ratatui_alignment(),
            ratatui::layout::Alignment::Center
        );
        assert_eq!(
            TitleAlignment::Right.to_ratatui_alignment(),
            ratatui::layout::Alignment::Right
        );
    }

    #[test]
    fn test_title_helpers() {
        let title = title_center("Test");
        // Just verify it compiles and creates a title
        let _title_text = format!("{:?}", title);

        let left = title_left("Left");
        let right = title_right("Right");
        // Verify they compile
        let _ = format!("{:?}{:?}", left, right);
    }

    #[test]
    fn test_title_right_with_spacing() {
        // Verify that title_right adds no spacing
        let title_no_space = title_right("Test");
        let content_no_space = format!("{:?}", title_no_space);
        assert!(content_no_space.contains("Test"));

        // Verify that title_right_with_spacing adds trailing spaces
        let title_with_space = title_right_with_spacing("Test");
        let content_with_space = format!("{:?}", title_with_space);
        assert!(content_with_space.contains("Test     ")); // Has 5 trailing spaces
    }

    #[test]
    fn test_create_title_with_alignment() {
        use ratatui::layout::Alignment;

        let left = create_title("Left", Some(TitleAlignment::Left), None);
        assert_eq!(left.alignment, Some(Alignment::Left));

        let center = create_title("Center", Some(TitleAlignment::Center), None);
        assert_eq!(center.alignment, Some(Alignment::Center));

        let right = create_title("Right", Some(TitleAlignment::Right), None);
        assert_eq!(right.alignment, Some(Alignment::Right));
    }

    #[test]
    fn test_create_title_default_alignment() {
        use ratatui::layout::Alignment;

        // None alignment should default to Center (TitleAlignment::default())
        let title = create_title("Default", None, None);
        assert_eq!(title.alignment, Some(Alignment::Center));
    }

    #[test]
    fn test_create_title_position_bottom_ignored() {
        // TitlePosition::Bottom is accepted without panic;
        // the returned Line carries the text and alignment regardless
        let title = create_title("Bottom", None, Some(TitlePosition::Bottom));
        let dbg = format!("{:?}", title);
        assert!(dbg.contains("Bottom"));
    }

    #[test]
    fn test_create_title_position_top() {
        let title = create_title("Top", None, Some(TitlePosition::Top));
        let dbg = format!("{:?}", title);
        assert!(dbg.contains("Top"));
    }

    #[test]
    fn test_border_set_segmented_plain() {
        let set = BorderStyle::PlainSegmented.border_set();
        assert!(set.segmented);
        assert_eq!(set.top_left, '┌');
    }

    #[test]
    fn test_border_set_segmented_rounded() {
        let set = BorderStyle::RoundedSegmented.border_set();
        assert!(set.segmented);
        assert_eq!(set.top_left, '╭');
    }

    #[test]
    fn test_border_set_segmented_double() {
        let set = BorderStyle::DoubleSegmented.border_set();
        assert!(set.segmented);
        assert_eq!(set.top_left, '╔');
    }

    #[test]
    fn test_border_set_segmented_thick() {
        let set = BorderStyle::ThickSegmented.border_set();
        assert!(set.segmented);
        assert_eq!(set.top_left, '┏');
    }

    #[test]
    fn test_border_set_sides_only_plain() {
        let set = BorderStyle::PlainSidesOnly.border_set();
        assert!(set.sides_only);
        assert!(!set.segmented);
    }

    #[test]
    fn test_border_set_sides_only_rounded() {
        let set = BorderStyle::RoundedSidesOnly.border_set();
        assert!(set.sides_only);
    }

    #[test]
    fn test_border_set_sides_only_double() {
        let set = BorderStyle::DoubleSidesOnly.border_set();
        assert!(set.sides_only);
    }

    #[test]
    fn test_border_set_sides_only_thick() {
        let set = BorderStyle::ThickSidesOnly.border_set();
        assert!(set.sides_only);
    }

    #[test]
    fn test_title_position_variants() {
        // Both variants exist and are distinct
        assert_ne!(
            std::mem::discriminant(&TitlePosition::Top),
            std::mem::discriminant(&TitlePosition::Bottom),
        );
    }

    #[test]
    fn test_title_alignment_default_is_center() {
        assert_eq!(TitleAlignment::default(), TitleAlignment::Center);
    }

    #[test]
    fn test_create_segmented_line_zero_length() {
        let line = create_segmented_line(0, '-');
        assert_eq!(line, "");
    }

    #[test]
    fn test_create_segmented_line_one_char() {
        let line = create_segmented_line(1, '─');
        assert_eq!(line, "─");
    }
}
