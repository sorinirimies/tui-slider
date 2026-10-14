#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, widgets::{Block, Widget}};
use tui_slider::*;

mod common;
use common::*;

#[derive(Arbitrary, Debug)]
struct In {
    value: f64,
    min: f64,
    max: f64,
    label: Option<String>,
    filled: String,
    empty: String,
    handle: String,
    show_value: bool,
    show_handle: bool,
    segmented: bool,
    bordered: bool,
    sel: [u8; 6],
    col: (u8, u8, u8, u8),
    area: Area,
}

fuzz_target!(|i: In| {
    // Constructors document panics on NaN / non-finite / inverted bounds.
    if i.value.is_nan() || !i.min.is_finite() || !i.max.is_finite() || i.min > i.max {
        return;
    }
    let mut s = Slider::new(i.value, i.min, i.max)
        .orientation(orient(i.sel[0]))
        .show_value(i.show_value)
        .show_handle(i.show_handle)
        .segmented(i.segmented)
        .value_alignment(align(i.sel[1]))
        .vertical_label_position(vlabel(i.sel[2]))
        .vertical_value_position(vvalue(i.sel[3]))
        .vertical_value_alignment(valign(i.sel[4]))
        .horizontal_bar_alignment(hbar(i.sel[5]))
        .filled_symbol(i.filled)
        .empty_symbol(i.empty)
        .handle_symbol(i.handle)
        .filled_color(color(i.col))
        .empty_color(color(i.col))
        .handle_color(color(i.col));
    if let Some(l) = i.label {
        s = s.label(l);
    }
    if i.bordered {
        s = s.block(Block::bordered().title("t"));
    }
    let (buf_area, area) = i.area.split();
    let mut buf = Buffer::empty(buf_area);
    s.render(area, &mut buf);
});
