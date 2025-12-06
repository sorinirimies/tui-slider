//! Symbols for slider widget
//!
//! This module provides predefined symbol sets that can be used with the [`Slider`] widget.
//! Each symbol set defines characters for the filled portion, empty portion, and handle of the slider.
//!
//! # Examples
//!
//! ```rust
//! use tui_slider::{Slider, SliderState};
//! use tui_slider::symbols;
//!
//! let state = SliderState::new(50.0, 0.0, 100.0);
//! let slider = Slider::from_state(&state)
//!     .filled_symbol(symbols::FILLED_THICK_LINE)
//!     .empty_symbol(symbols::EMPTY_THIN_LINE)
//!     .handle_symbol(symbols::HANDLE_CIRCLE);
//! ```
//!
//! [`Slider`]: crate::Slider

// ============================================================================
// FILLED SYMBOLS - Used for the filled portion of the slider
// ============================================================================

/// Default filled symbol - thick horizontal line
pub const FILLED_THICK_LINE: &str = "━";

/// Filled symbol - thin horizontal line
pub const FILLED_THIN_LINE: &str = "─";

/// Filled symbol - double horizontal line
pub const FILLED_DOUBLE_LINE: &str = "═";

/// Filled symbol - full block
pub const FILLED_BLOCK: &str = "█";

/// Filled symbol - dark shade
pub const FILLED_DARK_SHADE: &str = "▓";

/// Filled symbol - medium shade
pub const FILLED_MEDIUM_SHADE: &str = "▒";

/// Filled symbol - light shade
pub const FILLED_LIGHT_SHADE: &str = "░";

/// Filled symbol - horizontal bar
pub const FILLED_BAR: &str = "▬";

/// Filled symbol - progress bar filled
pub const FILLED_PROGRESS: &str = "▰";

/// Filled symbol - braille full
pub const FILLED_BRAILLE: &str = "⣿";

/// Filled symbol - wave
pub const FILLED_WAVE: &str = "≈";

/// Filled symbol - diamond
pub const FILLED_DIAMOND: &str = "◆";

/// Filled symbol - hash/number sign
pub const FILLED_HASH: &str = "#";

/// Filled symbol - equals sign
pub const FILLED_EQUALS: &str = "=";

/// Filled symbol - underscore lower bar
pub const FILLED_LOWER_BAR: &str = "▂";

/// Filled symbol - star
pub const FILLED_STAR: &str = "★";

/// Filled symbol - plus sign
pub const FILLED_PLUS: &str = "+";

/// Filled symbol - asterisk
pub const FILLED_ASTERISK: &str = "*";

/// Filled symbol - vertical rectangle
pub const FILLED_VERTICAL_RECT: &str = "▮";

/// Filled symbol - small square
pub const FILLED_SQUARE: &str = "■";

/// Filled symbol - circle
pub const FILLED_CIRCLE: &str = "●";

/// Filled symbol - vertical bar (for segmented blocks)
pub const FILLED_VERTICAL_BAR: &str = "│";

/// Filled symbol - segment dash (for segmented style)
pub const FILLED_SEGMENT: &str = "─";

/// Filled symbol - vertical line (for vertical sliders)
pub const FILLED_VERTICAL_LINE: &str = "│";

/// Filled symbol - horizontal line (for horizontal sliders)
pub const FILLED_HORIZONTAL_LINE: &str = "─";

// ============================================================================
// EMPTY SYMBOLS - Used for the unfilled portion of the slider
// ============================================================================

/// Default empty symbol - thin horizontal line
pub const EMPTY_THIN_LINE: &str = "─";

/// Empty symbol - space (invisible)
pub const EMPTY_SPACE: &str = " ";

/// Empty symbol - light shade
pub const EMPTY_LIGHT_SHADE: &str = "░";

/// Empty symbol - dotted line
pub const EMPTY_DOTTED: &str = "┄";

/// Empty symbol - segment dash
pub const EMPTY_SEGMENT: &str = "─";

/// Empty symbol - dashed line
pub const EMPTY_DASHED: &str = "╌";

/// Empty symbol - double thin line
pub const EMPTY_DOUBLE_THIN: &str = "─";

/// Empty symbol - progress bar empty
pub const EMPTY_PROGRESS: &str = "▱";

/// Empty symbol - braille lower dots
pub const EMPTY_BRAILLE: &str = "⣀";

/// Empty symbol - tilde/wave
pub const EMPTY_WAVE: &str = "˜";

/// Empty symbol - white diamond
pub const EMPTY_DIAMOND: &str = "◇";

/// Empty symbol - period/dot
pub const EMPTY_DOT: &str = ".";

/// Empty symbol - hyphen
pub const EMPTY_HYPHEN: &str = "-";

/// Empty symbol - underscore
pub const EMPTY_UNDERSCORE: &str = "_";

/// Empty symbol - lower bar
pub const EMPTY_LOWER_BAR: &str = "▁";

/// Empty symbol - white star
pub const EMPTY_STAR: &str = "☆";

/// Empty symbol - horizontal bar outline
pub const EMPTY_BAR_OUTLINE: &str = "▭";

/// Empty symbol - small white square
pub const EMPTY_SQUARE: &str = "□";

/// Empty symbol - white circle
pub const EMPTY_CIRCLE: &str = "○";

/// Empty symbol - vertical bar (for segmented blocks)
pub const EMPTY_VERTICAL_BAR: &str = "│";

/// Empty symbol - colon
pub const EMPTY_COLON: &str = ":";

/// Empty symbol - vertical line (for vertical sliders)
pub const EMPTY_VERTICAL_LINE: &str = "│";

/// Empty symbol - horizontal line (for horizontal sliders)
pub const EMPTY_HORIZONTAL_LINE: &str = "─";

// ============================================================================
// HANDLE SYMBOLS - Used for the slider handle/thumb
// ============================================================================

/// Default handle symbol - circle
pub const HANDLE_CIRCLE: &str = "●";

/// Handle symbol - white circle
pub const HANDLE_WHITE_CIRCLE: &str = "○";

/// Handle symbol - double circle
pub const HANDLE_DOUBLE_CIRCLE: &str = "◉";

/// Handle symbol - large circle
pub const HANDLE_LARGE_CIRCLE: &str = "◯";

/// Handle symbol - bullseye
pub const HANDLE_BULLSEYE: &str = "◎";

/// Handle symbol - black circle
pub const HANDLE_BLACK_CIRCLE: &str = "⬤";

/// Handle symbol - square
pub const HANDLE_SQUARE: &str = "■";

/// Handle symbol - white square
pub const HANDLE_WHITE_SQUARE: &str = "□";

/// Handle symbol - small square
pub const HANDLE_SMALL_SQUARE: &str = "▪";

/// Handle symbol - medium shade block
pub const HANDLE_MEDIUM_BLOCK: &str = "▓";

/// Handle symbol - diamond
pub const HANDLE_DIAMOND: &str = "◆";

/// Handle symbol - white diamond
pub const HANDLE_WHITE_DIAMOND: &str = "◇";

/// Handle symbol - double diamond
pub const HANDLE_DOUBLE_DIAMOND: &str = "◈";

/// Handle symbol - triangle right
pub const HANDLE_TRIANGLE_RIGHT: &str = "▶";

/// Handle symbol - triangle left
pub const HANDLE_TRIANGLE_LEFT: &str = "◀";

/// Handle symbol - triangle up
pub const HANDLE_TRIANGLE_UP: &str = "▲";

/// Handle symbol - triangle down
pub const HANDLE_TRIANGLE_DOWN: &str = "▼";

/// Handle symbol - vertical bar
pub const HANDLE_VERTICAL_BAR: &str = "│";

/// Handle symbol - pipe
pub const HANDLE_PIPE: &str = "|";

/// Handle symbol - at sign
pub const HANDLE_AT: &str = "@";

/// Handle symbol - star
pub const HANDLE_STAR: &str = "✦";

/// Handle symbol - sparkle
pub const HANDLE_SPARKLE: &str = "✨";

/// Handle symbol - white star
pub const HANDLE_WHITE_STAR: &str = "☆";

/// Handle symbol - filled star
pub const HANDLE_FILLED_STAR: &str = "★";

/// Handle symbol - hexagon
pub const HANDLE_HEXAGON: &str = "⬢";

/// Handle symbol - octagon
pub const HANDLE_OCTAGON: &str = "⬣";

/// Handle symbol - lower bar
pub const HANDLE_LOWER_BAR: &str = "▃";

/// Handle symbol - arrow up
pub const HANDLE_ARROW_UP: &str = "↑";

/// Handle symbol - arrow down
pub const HANDLE_ARROW_DOWN: &str = "↓";

/// Handle symbol - arrow left
pub const HANDLE_ARROW_LEFT: &str = "←";

/// Handle symbol - arrow right
pub const HANDLE_ARROW_RIGHT: &str = "→";

/// Handle symbol - horizontal line (for vertical sliders)
pub const HANDLE_HORIZONTAL_LINE: &str = "━";

/// Handle symbol - vertical line (for horizontal sliders)
pub const HANDLE_VERTICAL_LINE: &str = "│";

// ============================================================================
// PREDEFINED STYLE SETS
// ============================================================================

/// A complete symbol set for a slider style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolSet {
    /// Symbol for the filled portion
    pub filled: &'static str,
    /// Symbol for the empty portion
    pub empty: &'static str,
    /// Symbol for the handle
    pub handle: &'static str,
}

impl SymbolSet {
    /// Create a new custom symbol set
    pub const fn new(filled: &'static str, empty: &'static str, handle: &'static str) -> Self {
        Self {
            filled,
            empty,
            handle,
        }
    }
}

/// Default style - clean and professional
pub const STYLE_DEFAULT: SymbolSet = SymbolSet {
    filled: FILLED_THICK_LINE,
    empty: EMPTY_THIN_LINE,
    handle: HANDLE_CIRCLE,
};

/// Block style - bold and solid
pub const STYLE_BLOCK: SymbolSet = SymbolSet {
    filled: FILLED_BLOCK,
    empty: FILLED_LIGHT_SHADE,
    handle: FILLED_DARK_SHADE,
};

/// Dotted style - braille patterns
pub const STYLE_DOTTED: SymbolSet = SymbolSet {
    filled: FILLED_BRAILLE,
    empty: EMPTY_BRAILLE,
    handle: HANDLE_BLACK_CIRCLE,
};

/// Minimal style - clean and subtle
pub const STYLE_MINIMAL: SymbolSet = SymbolSet {
    filled: FILLED_THIN_LINE,
    empty: EMPTY_SPACE,
    handle: HANDLE_VERTICAL_BAR,
};

/// Double line style - formal appearance
pub const STYLE_DOUBLE_LINE: SymbolSet = SymbolSet {
    filled: FILLED_DOUBLE_LINE,
    empty: EMPTY_THIN_LINE,
    handle: HANDLE_DOUBLE_CIRCLE,
};

/// Wave style - fluid appearance
pub const STYLE_WAVE: SymbolSet = SymbolSet {
    filled: FILLED_WAVE,
    empty: EMPTY_WAVE,
    handle: HANDLE_DOUBLE_DIAMOND,
};

/// Progress style - progress bar look
pub const STYLE_PROGRESS: SymbolSet = SymbolSet {
    filled: FILLED_PROGRESS,
    empty: EMPTY_PROGRESS,
    handle: HANDLE_TRIANGLE_RIGHT,
};

/// Thick style - bold appearance
pub const STYLE_THICK: SymbolSet = SymbolSet {
    filled: FILLED_BAR,
    empty: FILLED_BAR,
    handle: HANDLE_SQUARE,
};

/// Gradient style - shaded effect
pub const STYLE_GRADIENT: SymbolSet = SymbolSet {
    filled: FILLED_DARK_SHADE,
    empty: FILLED_LIGHT_SHADE,
    handle: HANDLE_CIRCLE,
};

/// Rounded style - soft appearance
pub const STYLE_ROUNDED: SymbolSet = SymbolSet {
    filled: FILLED_THIN_LINE,
    empty: EMPTY_DASHED,
    handle: HANDLE_LARGE_CIRCLE,
};

/// Retro style - old-school ASCII
pub const STYLE_RETRO: SymbolSet = SymbolSet {
    filled: FILLED_HASH,
    empty: EMPTY_DOT,
    handle: HANDLE_AT,
};

/// Neon style - modern look
pub const STYLE_NEON: SymbolSet = SymbolSet {
    filled: FILLED_LOWER_BAR,
    empty: EMPTY_LOWER_BAR,
    handle: HANDLE_LOWER_BAR,
};

/// Diamond style - elegant look
pub const STYLE_DIAMOND: SymbolSet = SymbolSet {
    filled: FILLED_DIAMOND,
    empty: EMPTY_DIAMOND,
    handle: HANDLE_DOUBLE_DIAMOND,
};

/// Star style - decorative look
pub const STYLE_STAR: SymbolSet = SymbolSet {
    filled: FILLED_STAR,
    empty: EMPTY_STAR,
    handle: HANDLE_FILLED_STAR,
};

/// Arrow style - directional look
pub const STYLE_ARROW: SymbolSet = SymbolSet {
    filled: FILLED_BAR,
    empty: EMPTY_BAR_OUTLINE,
    handle: HANDLE_DIAMOND,
};

/// Segmented style - discrete segments with dashes
pub const STYLE_SEGMENTED: SymbolSet = SymbolSet {
    filled: FILLED_SEGMENT,
    empty: EMPTY_SPACE,
    handle: HANDLE_CIRCLE,
};

/// Segmented blocks style - vertical bars
pub const STYLE_SEGMENTED_BLOCKS: SymbolSet = SymbolSet {
    filled: FILLED_VERTICAL_BAR,
    empty: EMPTY_VERTICAL_BAR,
    handle: HANDLE_CIRCLE,
};

/// Segmented dots style - filled and empty circles
pub const STYLE_SEGMENTED_DOTS: SymbolSet = SymbolSet {
    filled: FILLED_CIRCLE,
    empty: EMPTY_CIRCLE,
    handle: HANDLE_CIRCLE,
};

/// Segmented squares style - filled and empty squares
pub const STYLE_SEGMENTED_SQUARES: SymbolSet = SymbolSet {
    filled: FILLED_SQUARE,
    empty: EMPTY_SQUARE,
    handle: HANDLE_CIRCLE,
};

// ============================================================================
// VERTICAL SLIDER STYLES
// ============================================================================

/// Vertical slider style - clean vertical lines
pub const STYLE_VERTICAL: SymbolSet = SymbolSet {
    filled: FILLED_VERTICAL_LINE,
    empty: EMPTY_VERTICAL_LINE,
    handle: HANDLE_HORIZONTAL_LINE,
};

/// Vertical slider style - bold blocks
pub const STYLE_VERTICAL_BLOCKS: SymbolSet = SymbolSet {
    filled: FILLED_BLOCK,
    empty: EMPTY_VERTICAL_BAR,
    handle: HANDLE_HORIZONTAL_LINE,
};

/// Vertical slider style - shaded gradient
pub const STYLE_VERTICAL_GRADIENT: SymbolSet = SymbolSet {
    filled: FILLED_DARK_SHADE,
    empty: FILLED_LIGHT_SHADE,
    handle: HANDLE_HORIZONTAL_LINE,
};

/// Vertical slider style - dots/circles
pub const STYLE_VERTICAL_DOTS: SymbolSet = SymbolSet {
    filled: FILLED_CIRCLE,
    empty: EMPTY_CIRCLE,
    handle: HANDLE_HORIZONTAL_LINE,
};

/// Vertical slider style - squares
pub const STYLE_VERTICAL_SQUARES: SymbolSet = SymbolSet {
    filled: FILLED_SQUARE,
    empty: EMPTY_SQUARE,
    handle: HANDLE_HORIZONTAL_LINE,
};

// ============================================================================
// HORIZONTAL SLIDER STYLES
// ============================================================================

/// Horizontal slider style - clean horizontal lines
pub const STYLE_HORIZONTAL: SymbolSet = SymbolSet {
    filled: FILLED_HORIZONTAL_LINE,
    empty: EMPTY_HORIZONTAL_LINE,
    handle: HANDLE_VERTICAL_LINE,
};

/// Horizontal slider style - thick lines
pub const STYLE_HORIZONTAL_THICK: SymbolSet = SymbolSet {
    filled: FILLED_THICK_LINE,
    empty: EMPTY_THIN_LINE,
    handle: HANDLE_CIRCLE,
};

/// Horizontal slider style - bold blocks
pub const STYLE_HORIZONTAL_BLOCKS: SymbolSet = SymbolSet {
    filled: FILLED_BLOCK,
    empty: FILLED_LIGHT_SHADE,
    handle: HANDLE_CIRCLE,
};

/// Horizontal slider style - shaded gradient
pub const STYLE_HORIZONTAL_GRADIENT: SymbolSet = SymbolSet {
    filled: FILLED_DARK_SHADE,
    empty: FILLED_LIGHT_SHADE,
    handle: HANDLE_CIRCLE,
};

/// Horizontal slider style - dots/circles
pub const STYLE_HORIZONTAL_DOTS: SymbolSet = SymbolSet {
    filled: FILLED_CIRCLE,
    empty: EMPTY_CIRCLE,
    handle: HANDLE_CIRCLE,
};

/// Horizontal slider style - squares
pub const STYLE_HORIZONTAL_SQUARES: SymbolSet = SymbolSet {
    filled: FILLED_SQUARE,
    empty: EMPTY_SQUARE,
    handle: HANDLE_CIRCLE,
};
