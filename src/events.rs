//! Optional crossterm event handling for [`SliderState`] (feature `crossterm`).
//!
//! ```ignore
//! match event::read()? {
//!     Event::Key(key) => { state.handle_key(key); }
//!     Event::Mouse(m) => { state.handle_mouse(m, slider_area, SliderOrientation::Horizontal); }
//!     _ => {}
//! }
//! ```

use crate::{SliderOrientation, SliderState};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::layout::Rect;

/// Number of steps moved by `PageUp` / `PageDown`.
const PAGE_STEPS: u32 = 10;

impl SliderState {
    /// Updates the value from a key event. Returns `true` if the value changed.
    ///
    /// | Key | Action |
    /// |---|---|
    /// | `Right`, `Up` | step up |
    /// | `Left`, `Down` | step down |
    /// | `PageUp` / `PageDown` | 10 steps up / down |
    /// | `Home` / `End` | minimum / maximum |
    ///
    /// Key release events are ignored.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        if key.kind == KeyEventKind::Release {
            return false;
        }
        let before = self.value();
        match key.code {
            KeyCode::Right | KeyCode::Up => self.step_up(),
            KeyCode::Left | KeyCode::Down => self.step_down(),
            KeyCode::PageUp => self.increase(self.step() * PAGE_STEPS as f64),
            KeyCode::PageDown => self.decrease(self.step() * PAGE_STEPS as f64),
            KeyCode::Home => self.set_value(self.min()),
            KeyCode::End => self.set_value(self.max()),
            _ => return false,
        }
        self.value() != before
    }

    /// Updates the value from a mouse event over a slider rendered in `area`.
    /// Returns `true` if the value changed.
    ///
    /// - Left click inside `area` jumps to that cell; dragging keeps following
    ///   the pointer along the slider axis, even outside `area` (clamped).
    /// - Scroll wheel steps the value while the pointer is inside `area`.
    ///
    /// Cell mapping matches the rendered handle (see [`SliderState::set_from_cell`]).
    pub fn handle_mouse(
        &mut self,
        mouse: MouseEvent,
        area: Rect,
        orientation: SliderOrientation,
    ) -> bool {
        if area.width == 0 || area.height == 0 {
            return false;
        }
        let inside = mouse.column >= area.x
            && mouse.column < area.x + area.width
            && mouse.row >= area.y
            && mouse.row < area.y + area.height;
        let before = self.value();

        match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) if inside => {
                self.set_from_mouse(mouse, area, orientation)
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                self.set_from_mouse(mouse, area, orientation)
            }
            MouseEventKind::ScrollUp if inside => self.step_up(),
            MouseEventKind::ScrollDown if inside => self.step_down(),
            _ => return false,
        }
        self.value() != before
    }

    fn set_from_mouse(&mut self, mouse: MouseEvent, area: Rect, orientation: SliderOrientation) {
        match orientation {
            SliderOrientation::Horizontal => {
                let cell = mouse.column.saturating_sub(area.x).min(area.width - 1);
                self.set_from_cell(cell, area.width);
            }
            SliderOrientation::Vertical => {
                let row = mouse.row.saturating_sub(area.y).min(area.height - 1);
                self.set_from_cell_vertical(row, area.height);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventState, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn mouse(kind: MouseEventKind, column: u16, row: u16) -> MouseEvent {
        MouseEvent {
            kind,
            column,
            row,
            modifiers: KeyModifiers::NONE,
        }
    }

    #[test]
    fn keys_step_and_jump() {
        let mut s = SliderState::with_step(50.0, 0.0, 100.0, 5.0);
        assert!(s.handle_key(key(KeyCode::Right)));
        assert_eq!(s.value(), 55.0);
        assert!(s.handle_key(key(KeyCode::Down)));
        assert_eq!(s.value(), 50.0);
        assert!(s.handle_key(key(KeyCode::PageUp)));
        assert_eq!(s.value(), 100.0);
        assert!(!s.handle_key(key(KeyCode::Up)), "already at max");
        assert!(s.handle_key(key(KeyCode::Home)));
        assert_eq!(s.value(), 0.0);
        assert!(s.handle_key(key(KeyCode::End)));
        assert_eq!(s.value(), 100.0);
        assert!(!s.handle_key(key(KeyCode::Char('x'))));
    }

    #[test]
    fn key_release_is_ignored() {
        let mut s = SliderState::new(50.0, 0.0, 100.0);
        let mut k = key(KeyCode::Right);
        k.kind = KeyEventKind::Release;
        k.state = KeyEventState::NONE;
        assert!(!s.handle_key(k));
        assert_eq!(s.value(), 50.0);
    }

    #[test]
    fn mouse_click_horizontal_reaches_both_ends() {
        let area = Rect::new(5, 2, 10, 1);
        let mut s = SliderState::new(50.0, 0.0, 100.0);
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 14, 2),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 100.0);
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 5, 2),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 0.0);
    }

    #[test]
    fn mouse_click_outside_is_ignored_but_drag_clamps() {
        let area = Rect::new(5, 2, 10, 1);
        let mut s = SliderState::new(50.0, 0.0, 100.0);
        assert!(!s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 30, 2),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 50.0);
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Drag(MouseButton::Left), 30, 9),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 100.0);
        assert!(s.handle_mouse(
            mouse(MouseEventKind::Drag(MouseButton::Left), 0, 9),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 0.0);
    }

    #[test]
    fn mouse_vertical_top_is_max() {
        let area = Rect::new(0, 3, 1, 8);
        let mut s = SliderState::new(50.0, 0.0, 100.0);
        s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 0, 3),
            area,
            SliderOrientation::Vertical,
        );
        assert_eq!(s.value(), 100.0);
        s.handle_mouse(
            mouse(MouseEventKind::Down(MouseButton::Left), 0, 10),
            area,
            SliderOrientation::Vertical,
        );
        assert_eq!(s.value(), 0.0);
    }

    #[test]
    fn mouse_scroll_steps_only_inside() {
        let area = Rect::new(0, 0, 10, 1);
        let mut s = SliderState::with_step(50.0, 0.0, 100.0, 5.0);
        assert!(s.handle_mouse(
            mouse(MouseEventKind::ScrollUp, 3, 0),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 55.0);
        assert!(!s.handle_mouse(
            mouse(MouseEventKind::ScrollUp, 30, 0),
            area,
            SliderOrientation::Horizontal
        ));
        assert!(s.handle_mouse(
            mouse(MouseEventKind::ScrollDown, 3, 0),
            area,
            SliderOrientation::Horizontal
        ));
        assert_eq!(s.value(), 50.0);
    }

    #[test]
    fn zero_sized_area_is_ignored() {
        let mut s = SliderState::new(50.0, 0.0, 100.0);
        let m = mouse(MouseEventKind::Drag(MouseButton::Left), 0, 0);
        assert!(!s.handle_mouse(m, Rect::new(0, 0, 0, 0), SliderOrientation::Horizontal));
    }
}
