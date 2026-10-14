#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use ratatui::{buffer::Buffer, widgets::Widget};
use tui_slider::*;

mod common;
use common::*;

#[derive(Arbitrary, Debug)]
struct In {
    state: (f64, f64, f64),
    step: f64,
    set_value: f64,
    style: u8,
    sel: u8,
    area: Area,
}

fuzz_target!(|i: In| {
    let (v, lo, hi) = i.state;
    if v.is_nan() || !lo.is_finite() || !hi.is_finite() || lo >= hi {
        return;
    }
    let mut st = SliderState::new(v, lo, hi);
    if !i.set_value.is_nan() {
        st.set_value(i.set_value);
    }
    if i.step.is_finite() && i.step > 0.0 {
        st.set_step(i.step);
    }
    let style = match i.style % 8 {
        0 => SliderStyle::default_style(),
        1 => SliderStyle::blocks(),
        2 => SliderStyle::segmented(),
        3 => SliderStyle::progress_battery(),
        4 => SliderStyle::vertical_equalizer(),
        5 => SliderStyle::horizontal_double(),
        6 => SliderStyle::gradient(),
        _ => SliderStyle::retro(),
    };
    let s = Slider::from_state(&st)
        .style(&style)
        .orientation(orient(i.sel))
        .label("x");
    let (b, a) = i.area.split();
    let mut buf = Buffer::empty(b);
    s.render(a, &mut buf);
});
