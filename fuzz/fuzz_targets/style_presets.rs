#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, widgets::Widget};
use tui_slider::*;

mod common;
use common::*;

#[derive(Arbitrary, Debug)]
struct In {
    pct: f64,
    which: u8,
    col: (u8, u8, u8, u8),
    segs: bool,
    area: Area,
}

fuzz_target!(|i: In| {
    if i.pct.is_nan() {
        return;
    }
    let mut st = SliderState::new(0.0, 0.0, 100.0);
    st.set_percentage(i.pct);
    let base = match i.which % 6 {
        0 => SliderStyle::progress_download(),
        1 => SliderStyle::vertical_blocks(),
        2 => SliderStyle::horizontal_dots(),
        3 => SliderStyle::segmented_stars(),
        4 => SliderStyle::wave(),
        _ => SliderStyle::custom("fuzz"),
    };
    let style = base.filled_color(color(i.col)).with_segments(i.segs);
    let (b, a) = i.area.split();
    let mut buf = Buffer::empty(b);
    Slider::from_state(&st).style(&style).orientation(orient(i.which)).render(a, &mut buf);
});
