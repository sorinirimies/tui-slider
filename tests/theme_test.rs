#![cfg(feature = "theme")]
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{Block, Borders, Widget},
};
use tui_slider::{Slider, SliderOrientation, SliderStyle, Theme, Themed};

fn render(s: Slider<'_>) -> Buffer {
    let area = Rect::new(0, 0, 30, 6);
    let mut buf = Buffer::empty(area);
    s.render(area, &mut buf);
    buf
}

fn fg_set(buf: &Buffer) -> Vec<Color> {
    buf.content().iter().map(|c| c.fg).collect()
}

#[test]
fn no_theme_unchanged() {
    let a = render(Slider::new(50.0, 0.0, 100.0));
    let b = render(Slider::new(50.0, 0.0, 100.0).filled_color(Color::Cyan));
    assert_eq!(a, b);
}

#[test]
fn theme_maps_roles() {
    let t = Theme::nord();
    let buf = render(Slider::new(50.0, 0.0, 100.0).theme(&t));
    let fgs = fg_set(&buf);
    assert!(fgs.contains(&t.primary));
    assert!(fgs.contains(&t.disabled));
}

#[test]
fn explicit_wins_either_order() {
    let t = Theme::dracula();
    let before = Slider::new(50.0, 0.0, 100.0)
        .filled_color(Color::Red)
        .theme(&t);
    let after = Slider::new(50.0, 0.0, 100.0)
        .theme(&t)
        .filled_color(Color::Red);
    assert_eq!(render(before), render(after.clone()));
    assert!(fg_set(&render(after)).contains(&Color::Red));
}

#[test]
fn style_theme_and_provider() {
    let t = Theme::nord();
    let s = SliderStyle::blocks().theme(&t);
    assert_eq!(s.filled_color, t.primary);
    assert_eq!(s.empty_color, t.disabled);
    let s2 = SliderStyle::blocks().themed(&t);
    assert_eq!(s2.handle_color, t.foreground);
}

#[test]
fn all_themes_render() {
    for t in Theme::all() {
        for o in [SliderOrientation::Horizontal, SliderOrientation::Vertical] {
            let s = Slider::new(40.0, 0.0, 100.0)
                .orientation(o)
                .label("L")
                .show_value(true)
                .block(Block::default().borders(Borders::ALL))
                .theme(&t);
            render(s);
        }
        let _ = Slider::default().style(&SliderStyle::wave().theme(&t));
    }
}
