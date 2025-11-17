//! Slider style configuration module
//!
//! This module provides style configuration for sliders, including predefined styles
//! and a builder pattern for creating custom styles.
//!
//! # Examples
//!
//! ## Using predefined styles
//!
//! ```rust
//! use tui_slider::style::SliderStyle;
//!
//! let style = SliderStyle::default();
//! let blocks = SliderStyle::blocks();
//! let dots = SliderStyle::dots();
//! ```
//!
//! ## Creating custom styles
//!
//! ```rust
//! use tui_slider::style::SliderStyle;
//! use tui_slider::symbols;
//! use ratatui::style::Color;
//!
//! let custom = SliderStyle::custom("My Style")
//!     .filled_symbol(symbols::FILLED_BLOCK)
//!     .empty_symbol(symbols::EMPTY_LIGHT_SHADE)
//!     .handle_symbol(symbols::HANDLE_DIAMOND)
//!     .filled_color(Color::Cyan)
//!     .empty_color(Color::DarkGray)
//!     .handle_color(Color::White);
//! ```

use crate::symbols;
use ratatui::style::Color;

/// Style configuration for sliders
///
/// This struct defines the visual appearance of a slider, including symbols and colors.
/// You can use predefined styles or create custom ones with the builder pattern.
#[derive(Debug, Clone)]
pub struct SliderStyle {
    /// Display name for the style
    pub name: &'static str,
    /// Symbol for the filled portion
    pub filled_symbol: &'static str,
    /// Symbol for the empty portion
    pub empty_symbol: &'static str,
    /// Symbol for the slider handle/thumb
    pub handle_symbol: &'static str,
    /// Color for filled portion
    pub filled_color: Color,
    /// Color for empty portion
    pub empty_color: Color,
    /// Color for the handle
    pub handle_color: Color,
    /// Whether to render as discrete segments with spaces
    pub segmented: bool,
}

impl SliderStyle {
    /// Default style - clean and professional
    pub fn default_style() -> Self {
        Self {
            name: "Default",
            filled_symbol: symbols::FILLED_THICK_LINE,
            empty_symbol: symbols::EMPTY_THIN_LINE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Block style - bold and solid
    pub fn blocks() -> Self {
        Self {
            name: "Blocks",
            filled_symbol: symbols::FILLED_BLOCK,
            empty_symbol: symbols::FILLED_LIGHT_SHADE,
            handle_symbol: symbols::FILLED_DARK_SHADE,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Dotted style - braille patterns
    pub fn dots() -> Self {
        Self {
            name: "Dots",
            filled_symbol: symbols::FILLED_BRAILLE,
            empty_symbol: symbols::EMPTY_BRAILLE,
            handle_symbol: symbols::HANDLE_BLACK_CIRCLE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Arrow style - geometric shapes
    pub fn arrows() -> Self {
        Self {
            name: "Arrows",
            filled_symbol: symbols::FILLED_BAR,
            empty_symbol: symbols::EMPTY_BAR_OUTLINE,
            handle_symbol: symbols::HANDLE_DIAMOND,
            filled_color: Color::Magenta,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Minimal style - clean and subtle
    pub fn minimal() -> Self {
        Self {
            name: "Minimal",
            filled_symbol: symbols::FILLED_THIN_LINE,
            empty_symbol: symbols::EMPTY_SPACE,
            handle_symbol: symbols::HANDLE_VERTICAL_BAR,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::Cyan,
            segmented: false,
        }
    }

    /// Double line style - formal appearance
    pub fn double_line() -> Self {
        Self {
            name: "Double Line",
            filled_symbol: symbols::FILLED_DOUBLE_LINE,
            empty_symbol: symbols::EMPTY_THIN_LINE,
            handle_symbol: symbols::HANDLE_DOUBLE_CIRCLE,
            filled_color: Color::Red,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Wave style - fluid appearance
    pub fn wave() -> Self {
        Self {
            name: "Wave",
            filled_symbol: symbols::FILLED_WAVE,
            empty_symbol: symbols::EMPTY_WAVE,
            handle_symbol: symbols::HANDLE_DOUBLE_DIAMOND,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Progress style - progress bar look
    pub fn progress() -> Self {
        Self {
            name: "Progress",
            filled_symbol: symbols::FILLED_PROGRESS,
            empty_symbol: symbols::EMPTY_PROGRESS,
            handle_symbol: symbols::HANDLE_TRIANGLE_RIGHT,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::Yellow,
            segmented: false,
        }
    }

    /// Thick style - bold appearance
    pub fn thick() -> Self {
        Self {
            name: "Thick",
            filled_symbol: symbols::FILLED_BAR,
            empty_symbol: symbols::FILLED_BAR,
            handle_symbol: symbols::HANDLE_SQUARE,
            filled_color: Color::Magenta,
            empty_color: Color::Rgb(60, 60, 60),
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Gradient style - shaded effect
    pub fn gradient() -> Self {
        Self {
            name: "Gradient",
            filled_symbol: symbols::FILLED_DARK_SHADE,
            empty_symbol: symbols::FILLED_LIGHT_SHADE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::Cyan,
            segmented: false,
        }
    }

    /// Rounded style - soft appearance
    pub fn rounded() -> Self {
        Self {
            name: "Rounded",
            filled_symbol: symbols::FILLED_THIN_LINE,
            empty_symbol: symbols::EMPTY_DASHED,
            handle_symbol: symbols::HANDLE_LARGE_CIRCLE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Retro style - old-school ASCII
    pub fn retro() -> Self {
        Self {
            name: "Retro",
            filled_symbol: symbols::FILLED_HASH,
            empty_symbol: symbols::EMPTY_DOT,
            handle_symbol: symbols::HANDLE_AT,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Segmented style - discrete segments with spaces
    pub fn segmented() -> Self {
        Self {
            name: "Segmented",
            filled_symbol: "─",
            empty_symbol: "─",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Red,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Segmented blocks style
    pub fn segmented_blocks() -> Self {
        Self {
            name: "Segmented Blocks",
            filled_symbol: "█",
            empty_symbol: "░",
            handle_symbol: symbols::HANDLE_SQUARE,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Segmented dots style
    pub fn segmented_dots() -> Self {
        Self {
            name: "Segmented Dots",
            filled_symbol: "●",
            empty_symbol: "○",
            handle_symbol: symbols::HANDLE_DIAMOND,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::Yellow,
            segmented: true,
        }
    }

    /// Segmented bars style
    pub fn segmented_bars() -> Self {
        Self {
            name: "Segmented Bars",
            filled_symbol: "│",
            empty_symbol: "┆",
            handle_symbol: symbols::HANDLE_TRIANGLE_RIGHT,
            filled_color: Color::Magenta,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Segmented squares style
    pub fn segmented_squares() -> Self {
        Self {
            name: "Segmented Squares",
            filled_symbol: "■",
            empty_symbol: "□",
            handle_symbol: symbols::HANDLE_DOUBLE_CIRCLE,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::Cyan,
            segmented: true,
        }
    }

    /// Segmented diamonds style
    pub fn segmented_diamonds() -> Self {
        Self {
            name: "Segmented Diamonds",
            filled_symbol: "◆",
            empty_symbol: "◇",
            handle_symbol: symbols::HANDLE_HEXAGON,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Segmented stars style
    pub fn segmented_stars() -> Self {
        Self {
            name: "Segmented Stars",
            filled_symbol: "★",
            empty_symbol: "☆",
            handle_symbol: symbols::HANDLE_SPARKLE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::Cyan,
            segmented: true,
        }
    }

    /// Segmented arrows style
    pub fn segmented_arrows() -> Self {
        Self {
            name: "Segmented Arrows",
            filled_symbol: "▶",
            empty_symbol: "▷",
            handle_symbol: symbols::HANDLE_TRIANGLE_RIGHT,
            filled_color: Color::Red,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Segmented thick style
    pub fn segmented_thick() -> Self {
        Self {
            name: "Segmented Thick",
            filled_symbol: "━",
            empty_symbol: "╌",
            handle_symbol: symbols::HANDLE_LARGE_CIRCLE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: true,
        }
    }

    /// Create a custom slider style with builder pattern
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tui_slider::style::SliderStyle;
    /// use tui_slider::symbols;
    /// use ratatui::style::Color;
    ///
    /// let custom = SliderStyle::custom("Rainbow")
    ///     .filled_symbol(symbols::FILLED_BLOCK)
    ///     .empty_symbol(symbols::EMPTY_LIGHT_SHADE)
    ///     .handle_symbol(symbols::HANDLE_CIRCLE)
    ///     .filled_color(Color::Rgb(255, 100, 200))
    ///     .empty_color(Color::Rgb(50, 50, 50))
    ///     .handle_color(Color::White);
    /// ```
    pub fn custom(name: &'static str) -> Self {
        Self {
            name,
            filled_symbol: symbols::FILLED_THICK_LINE,
            empty_symbol: symbols::EMPTY_THIN_LINE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Set the filled symbol for the custom style
    pub fn filled_symbol(mut self, symbol: &'static str) -> Self {
        self.filled_symbol = symbol;
        self
    }

    /// Set the empty symbol for the custom style
    pub fn empty_symbol(mut self, symbol: &'static str) -> Self {
        self.empty_symbol = symbol;
        self
    }

    /// Set the handle symbol for the custom style
    pub fn handle_symbol(mut self, symbol: &'static str) -> Self {
        self.handle_symbol = symbol;
        self
    }

    /// Set the filled color for the custom style
    pub fn filled_color(mut self, color: Color) -> Self {
        self.filled_color = color;
        self
    }

    /// Set the empty color for the custom style
    pub fn empty_color(mut self, color: Color) -> Self {
        self.empty_color = color;
        self
    }

    /// Set the handle color for the custom style
    pub fn handle_color(mut self, color: Color) -> Self {
        self.handle_color = color;
        self
    }

    /// Enable or disable segmented rendering for the custom style
    pub fn with_segments(mut self, enabled: bool) -> Self {
        self.segmented = enabled;
        self
    }
}

/// Progress bar style presets
///
/// These styles are specifically designed for progress bars (sliders without handles)
impl SliderStyle {
    /// Download progress bar style
    pub fn progress_download() -> Self {
        Self {
            name: "Download",
            filled_symbol: "█",
            empty_symbol: "░",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Upload progress bar style
    pub fn progress_upload() -> Self {
        Self {
            name: "Upload",
            filled_symbol: "▰",
            empty_symbol: "▱",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Health bar style (gaming UI)
    pub fn progress_health() -> Self {
        Self {
            name: "Health",
            filled_symbol: "▓",
            empty_symbol: "░",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Red,
            empty_color: Color::Rgb(40, 40, 40),
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Mana bar style (gaming UI)
    pub fn progress_mana() -> Self {
        Self {
            name: "Mana",
            filled_symbol: "▓",
            empty_symbol: "░",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Cyan,
            empty_color: Color::Rgb(40, 40, 40),
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Experience bar style (gaming UI)
    pub fn progress_experience() -> Self {
        Self {
            name: "Experience",
            filled_symbol: "━",
            empty_symbol: "─",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Loading bar style
    pub fn progress_loading() -> Self {
        Self {
            name: "Loading",
            filled_symbol: "═",
            empty_symbol: "─",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Magenta,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Installation progress bar style
    pub fn progress_installation() -> Self {
        Self {
            name: "Installation",
            filled_symbol: "▬",
            empty_symbol: "▭",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::LightGreen,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Battery level bar style
    pub fn progress_battery() -> Self {
        Self {
            name: "Battery",
            filled_symbol: "■",
            empty_symbol: "□",
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::LightYellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }
}

/// Vertical slider style presets
///
/// These styles are specifically designed for vertical sliders (like audio mixers, equalizers)
impl SliderStyle {
    /// Vertical slider style - clean vertical lines
    pub fn vertical() -> Self {
        Self {
            name: "Vertical",
            filled_symbol: symbols::FILLED_VERTICAL_LINE,
            empty_symbol: symbols::EMPTY_VERTICAL_LINE,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Vertical slider style - bold blocks
    pub fn vertical_blocks() -> Self {
        Self {
            name: "Vertical Blocks",
            filled_symbol: symbols::FILLED_BLOCK,
            empty_symbol: symbols::EMPTY_VERTICAL_BAR,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Vertical slider style - shaded gradient
    pub fn vertical_gradient() -> Self {
        Self {
            name: "Vertical Gradient",
            filled_symbol: symbols::FILLED_DARK_SHADE,
            empty_symbol: symbols::FILLED_LIGHT_SHADE,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::Magenta,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Vertical slider style - dots/circles
    pub fn vertical_dots() -> Self {
        Self {
            name: "Vertical Dots",
            filled_symbol: symbols::FILLED_CIRCLE,
            empty_symbol: symbols::EMPTY_CIRCLE,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Vertical slider style - squares
    pub fn vertical_squares() -> Self {
        Self {
            name: "Vertical Squares",
            filled_symbol: symbols::FILLED_SQUARE,
            empty_symbol: symbols::EMPTY_SQUARE,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Vertical slider style - equalizer bars
    pub fn vertical_equalizer() -> Self {
        Self {
            name: "Equalizer",
            filled_symbol: symbols::FILLED_VERTICAL_BAR,
            empty_symbol: symbols::EMPTY_VERTICAL_BAR,
            handle_symbol: symbols::HANDLE_HORIZONTAL_LINE,
            filled_color: Color::LightGreen,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }
}

/// Horizontal slider style presets
///
/// These styles are specifically designed for horizontal sliders (like volume controls, seekbars)
impl SliderStyle {
    /// Horizontal slider style - clean horizontal lines
    pub fn horizontal() -> Self {
        Self {
            name: "Horizontal",
            filled_symbol: symbols::FILLED_HORIZONTAL_LINE,
            empty_symbol: symbols::EMPTY_HORIZONTAL_LINE,
            handle_symbol: symbols::HANDLE_VERTICAL_LINE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - thick lines
    pub fn horizontal_thick() -> Self {
        Self {
            name: "Horizontal Thick",
            filled_symbol: symbols::FILLED_THICK_LINE,
            empty_symbol: symbols::EMPTY_THIN_LINE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Cyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - bold blocks
    pub fn horizontal_blocks() -> Self {
        Self {
            name: "Horizontal Blocks",
            filled_symbol: symbols::FILLED_BLOCK,
            empty_symbol: symbols::FILLED_LIGHT_SHADE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Green,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - shaded gradient
    pub fn horizontal_gradient() -> Self {
        Self {
            name: "Horizontal Gradient",
            filled_symbol: symbols::FILLED_DARK_SHADE,
            empty_symbol: symbols::FILLED_LIGHT_SHADE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Magenta,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - dots/circles
    pub fn horizontal_dots() -> Self {
        Self {
            name: "Horizontal Dots",
            filled_symbol: symbols::FILLED_CIRCLE,
            empty_symbol: symbols::EMPTY_CIRCLE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Yellow,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - squares
    pub fn horizontal_squares() -> Self {
        Self {
            name: "Horizontal Squares",
            filled_symbol: symbols::FILLED_SQUARE,
            empty_symbol: symbols::EMPTY_SQUARE,
            handle_symbol: symbols::HANDLE_CIRCLE,
            filled_color: Color::Blue,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }

    /// Horizontal slider style - double lines
    pub fn horizontal_double() -> Self {
        Self {
            name: "Horizontal Double",
            filled_symbol: symbols::FILLED_DOUBLE_LINE,
            empty_symbol: symbols::EMPTY_THIN_LINE,
            handle_symbol: symbols::HANDLE_DOUBLE_CIRCLE,
            filled_color: Color::LightCyan,
            empty_color: Color::DarkGray,
            handle_color: Color::White,
            segmented: false,
        }
    }
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self::default_style()
    }
}
