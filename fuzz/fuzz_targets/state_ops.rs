#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use tui_slider::SliderState;

#[derive(Arbitrary, Debug)]
enum Op {
    SetValue(f64),
    SetMin(f64),
    SetMax(f64),
    SetPercentage(f64),
    SetStep(f64),
    Increase(f64),
    Decrease(f64),
    StepUp,
    StepDown,
    SetFromPosition(u16, u16),
}

#[derive(Arbitrary, Debug)]
struct In {
    value: f64,
    min: f64,
    max: f64,
    ops: Vec<Op>,
}

fuzz_target!(|i: In| {
    if i.value.is_nan() || !i.min.is_finite() || !i.max.is_finite() || i.min >= i.max {
        return;
    }
    let mut s = SliderState::new(i.value, i.min, i.max);
    for op in i.ops.into_iter().take(64) {
        // Skip inputs the API documents as panicking (NaN / non-finite / invalid).
        match op {
            Op::SetValue(v) if !v.is_nan() => s.set_value(v),
            Op::SetMin(v) if v.is_finite() && v < s.max() => s.set_min(v),
            Op::SetMax(v) if v.is_finite() && v > s.min() => s.set_max(v),
            Op::SetPercentage(p) if !p.is_nan() => s.set_percentage(p),
            Op::SetStep(v) if v.is_finite() && v > 0.0 => s.set_step(v),
            Op::Increase(v) if v.is_finite() && v >= 0.0 => s.increase(v),
            Op::Decrease(v) if v.is_finite() && v >= 0.0 => s.decrease(v),
            Op::StepUp => s.step_up(),
            Op::StepDown => s.step_down(),
            Op::SetFromPosition(p, l) => s.set_from_position(p, l),
            _ => {}
        }
        let _ = (s.percentage(), s.range(), s.is_low(), s.is_medium(), s.is_high());
        let _ = (s.is_at_min(), s.is_at_max(), s.is_at_middle());
        let _ = (s.distance_from_min(), s.distance_from_max());
        let _ = s.percentage_string();
        let _ = s.value_string(3);
    }
});
