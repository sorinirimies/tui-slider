#![no_main]
//! Render invariants: with single-column ASCII symbols the handle must be inside
//! the area, the fill must be contiguous up to it, and `SliderState::cell*`
//! must agree with where the handle was drawn.
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tui_slider::*;

#[derive(Arbitrary, Debug)]
struct In {
    value: f64,
    min: f64,
    max: f64,
    vertical: bool,
    len: u8,
    cross: u8,
    x: u8,
    y: u8,
}

fuzz_target!(|i: In| {
    if i.value.is_nan() || !i.min.is_finite() || !i.max.is_finite() || i.min >= i.max {
        return;
    }
    let len = (i.len % 100) as u16 + 2; // 2..=101 cells along the axis
    let cross = (i.cross % 4) as u16 + 1;
    let (w, h) = if i.vertical { (cross, len) } else { (len, cross) };
    let (x0, y0) = ((i.x % 8) as u16, (i.y % 8) as u16);
    let area = Rect::new(x0, y0, w, h);
    let mut buf = Buffer::empty(Rect::new(0, 0, x0 + w + 4, y0 + h + 4));

    let orientation = if i.vertical { SliderOrientation::Vertical } else { SliderOrientation::Horizontal };
    Slider::new(i.value, i.min, i.max)
        .orientation(orientation)
        .filled_symbol("#")
        .empty_symbol("-")
        .handle_symbol("H")
        .show_handle(true)
        .render(area, &mut buf);

    let state = SliderState::new(i.value, i.min, i.max);
    // Vertical symbols are centred across the bar; horizontal bars sit on one row.
    let col_x = x0 + (cross - 1) / 2;
    let row_y = (y0..y0 + h).find(|&y| (x0..x0 + w).any(|x| buf[(x, y)].symbol() == "H")).unwrap_or(y0);
    let sym = |k: u16| if i.vertical { buf[(col_x, y0 + k)].symbol().to_string() } else { buf[(x0 + k, row_y)].symbol().to_string() };

    let handle = (0..len).find(|&k| sym(k) == "H").expect("handle drawn inside area");
    if i.vertical {
        assert_eq!(state.cell_vertical(len), handle);
        // top = max: cells above handle empty, below filled
        for k in 0..handle { assert_eq!(sym(k), "-"); }
        for k in handle + 1..len { assert_eq!(sym(k), "#"); }
    } else {
        assert_eq!(state.cell(len), handle);
        for k in 0..handle { assert_eq!(sym(k), "#"); }
        for k in handle + 1..len { assert_eq!(sym(k), "-"); }
    }
});
