#![allow(dead_code)]
use arbitrary::Arbitrary;
use ratatui::layout::{Alignment, Rect};
use ratatui::style::Color;
use tui_slider::*;

#[derive(Arbitrary, Debug, Clone, Copy)]
pub struct Area {
    pub x: u16,
    pub y: u16,
    pub w: u8,
    pub h: u8,
}

impl Area {
    /// Bounded (<=200x100 buffer), possibly zero-size, offset from origin.
    pub fn split(self) -> (Rect, Rect) {
        let buf = Rect::new(0, 0, 200, 100);
        let x = self.x % 200;
        let y = self.y % 100;
        let w = (self.w as u16 % 201).min(200 - x);
        let h = (self.h as u16 % 101).min(100 - y);
        (buf, Rect::new(x, y, w, h))
    }
}

pub fn orient(n: u8) -> SliderOrientation {
    if n & 1 == 0 { SliderOrientation::Horizontal } else { SliderOrientation::Vertical }
}
pub fn vlabel(n: u8) -> VerticalLabelPosition {
    [VerticalLabelPosition::Top, VerticalLabelPosition::Bottom][(n % 2) as usize]
}
pub fn vvalue(n: u8) -> VerticalValuePosition {
    [VerticalValuePosition::Top, VerticalValuePosition::Middle, VerticalValuePosition::Bottom][(n % 3) as usize]
}
pub fn valign(n: u8) -> VerticalValueAlignment {
    [VerticalValueAlignment::Left, VerticalValueAlignment::Center, VerticalValueAlignment::Right][(n % 3) as usize]
}
pub fn hbar(n: u8) -> HorizontalBarAlignment {
    [HorizontalBarAlignment::Top, HorizontalBarAlignment::Center, HorizontalBarAlignment::Bottom][(n % 3) as usize]
}
pub fn align(n: u8) -> Alignment {
    [Alignment::Left, Alignment::Center, Alignment::Right][(n % 3) as usize]
}
pub fn color(c: (u8, u8, u8, u8)) -> Color {
    match c.0 % 3 {
        0 => Color::Rgb(c.1, c.2, c.3),
        1 => Color::Indexed(c.1),
        _ => Color::Reset,
    }
}
