#![no_main]
use libfuzzer_sys::fuzz_target;
use tui_slider::border::*;

fuzz_target!(|d: (u16, char, String, u8)| {
    let (len, ch, text, sel) = d;
    let _ = create_segmented_line(len as usize, ch);
    let al = [TitleAlignment::Left, TitleAlignment::Center, TitleAlignment::Right];
    let _ = create_title(text.clone(), Some(al[(sel % 3) as usize]), None);
    let _ = title_left(text.clone());
    let _ = title_center(text.clone());
    let _ = title_right(text.clone());
    let _ = title_right_with_spacing(text);
    for b in BorderStyle::all() {
        let _ = (b.border_set(), b.name(), b.description(), b.is_segmented(), b.is_sides_only());
    }
});
