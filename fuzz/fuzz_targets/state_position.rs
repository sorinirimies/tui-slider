#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tui_slider::SliderState;

#[derive(Arbitrary, Debug)]
struct In {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    len: u16,
    pos: u16,
    decimals: u8,
}

fuzz_target!(|i: In| {
    if i.value.is_nan() || !i.min.is_finite() || !i.max.is_finite() || i.min >= i.max {
        return;
    }
    let mut s = if i.step.is_finite() && i.step > 0.0 {
        SliderState::with_step(i.value, i.min, i.max, i.step)
    } else {
        SliderState::new(i.value, i.min, i.max)
    };
    let _ = s.position(i.len);
    s.set_from_position(i.pos, i.len);
    let _ = s.position(i.len);
    let _ = s.value_string(i.decimals as usize);
});
