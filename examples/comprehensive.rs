//! Comprehensive demo: a small studio mixer.
//!
//! Shows most of the crate in one screen:
//! - vertical channel faders and horizontal controls
//! - negative ranges, fractional steps (no float drift), segmented bars
//! - keyboard **and mouse** control through `SliderState::handle_key` / `handle_mouse`
//! - shared theming (`t` / `T` cycles themes)
//!
//! Run with: `cargo run --example comprehensive --features theme,crossterm`

use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{
    Slider, SliderOrientation, SliderState, VerticalLabelPosition, VerticalValuePosition,
};
use tui_slider::{Theme, ThemeSwitcher};

#[derive(Clone, Copy, PartialEq)]
enum Look {
    Fader,
    Segmented,
    Blocks,
    Line,
}

struct Control {
    name: &'static str,
    state: SliderState,
    orientation: SliderOrientation,
    look: Look,
    color: Color,
    /// Bar area from the last draw, used for mouse hit testing.
    area: Rect,
}

impl Control {
    fn new(
        name: &'static str,
        state: SliderState,
        orientation: SliderOrientation,
        look: Look,
        color: Color,
    ) -> Self {
        Self {
            name,
            state,
            orientation,
            look,
            color,
            area: Rect::default(),
        }
    }

    fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
    }
}

struct App {
    controls: Vec<Control>,
    selected: usize,
    themes: ThemeSwitcher,
}

const CHANNELS: usize = 6;

impl App {
    fn new() -> Self {
        use SliderOrientation::{Horizontal as H, Vertical as V};
        let v = |name, value, color| {
            Control::new(
                name,
                SliderState::with_step(value, 0.0, 100.0, 5.0),
                V,
                Look::Fader,
                color,
            )
        };
        Self {
            controls: vec![
                v("Gain", 72.0, Color::Cyan),
                v("Bass", 60.0, Color::Green),
                v("Mid", 48.0, Color::Yellow),
                v("Treble", 55.0, Color::Magenta),
                v("Reverb", 30.0, Color::Blue),
                v("Master", 80.0, Color::Red),
                Control::new(
                    "Pan",
                    SliderState::with_step(-10.0, -50.0, 50.0, 5.0),
                    H,
                    Look::Line,
                    Color::Cyan,
                ),
                Control::new(
                    "Tempo (bpm)",
                    SliderState::with_step(120.0, 60.0, 180.0, 5.0),
                    H,
                    Look::Blocks,
                    Color::Green,
                ),
                Control::new(
                    "Cutoff",
                    SliderState::with_step(65.0, 0.0, 100.0, 5.0),
                    H,
                    Look::Segmented,
                    Color::Yellow,
                ),
                Control::new(
                    "Resonance",
                    SliderState::with_step(40.0, 0.0, 100.0, 5.0),
                    H,
                    Look::Line,
                    Color::Blue,
                ),
                Control::new(
                    "Drive (0.1 steps)",
                    SliderState::with_step(0.3, 0.0, 1.0, 0.1),
                    H,
                    Look::Line,
                    Color::Magenta,
                ),
                Control::new(
                    "Mix (dry/wet)",
                    SliderState::with_step(35.0, 0.0, 100.0, 5.0),
                    H,
                    Look::Blocks,
                    Color::Red,
                ),
                Control::new(
                    "Attack (ms)",
                    SliderState::with_step(60.0, 0.0, 200.0, 10.0),
                    H,
                    Look::Segmented,
                    Color::Cyan,
                ),
            ],
            selected: 0,
            themes: ThemeSwitcher::new(),
        }
    }

    fn select(&mut self, delta: isize) {
        let n = self.controls.len() as isize;
        self.selected = (self.selected as isize + delta).rem_euclid(n) as usize;
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    res
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()>
where
    B::Error: Send + Sync + 'static,
{
    loop {
        terminal.draw(|f| ui(f, app))?;
        if !event::poll(std::time::Duration::from_millis(16))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) if key.kind != KeyEventKind::Release => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('t') => {
                    app.themes.next();
                }
                KeyCode::Char('T') => {
                    app.themes.prev();
                }
                KeyCode::Tab | KeyCode::Char('l') => app.select(1),
                KeyCode::BackTab | KeyCode::Char('h') => app.select(-1),
                // Arrows, PageUp/PageDown, Home/End adjust the selected control.
                _ => {
                    app.controls[app.selected].state.handle_key(key);
                }
            },
            Event::Mouse(m) => {
                // A press or scroll selects the control under the pointer.
                let pressed = matches!(
                    m.kind,
                    MouseEventKind::Down(_) | MouseEventKind::ScrollUp | MouseEventKind::ScrollDown
                );
                if pressed {
                    if let Some(i) = app
                        .controls
                        .iter()
                        .position(|c| c.contains(m.column, m.row))
                    {
                        app.selected = i;
                    }
                }
                let c = &mut app.controls[app.selected];
                c.state.handle_mouse(m, c.area, c.orientation);
            }
            _ => {}
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let th = app.themes.current().clone().downgrade_for_terminal();
    f.render_widget(Block::default().style(th.style()), f.area());

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(12),
            Constraint::Length(2),
        ])
        .split(f.area());

    f.render_widget(
        Paragraph::new(format!("tui-slider · Studio Mixer · theme: {}", th.name))
            .style(th.title_style())
            .alignment(Alignment::Center),
        rows[0],
    );

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(rows[1]);

    render_channels(f, app, &th, cols[0]);
    render_controls(f, app, &th, cols[1]);

    f.render_widget(
        Paragraph::new("Tab/h/l: select · ←→↑↓ PgUp/PgDn Home/End: adjust · click/drag/scroll · t/T: theme · q: quit")
            .style(th.muted_style())
            .alignment(Alignment::Center),
        rows[2],
    );
}

fn panel(th: &Theme, selected: bool, title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if selected {
            th.border_style(true).add_modifier(Modifier::BOLD)
        } else {
            th.border_style(false)
        })
        .title(title.to_string())
}

fn render_channels(f: &mut Frame, app: &mut App, th: &Theme, area: Rect) {
    let outer = panel(th, false, " Channel strip ");
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let cells = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Ratio(1, CHANNELS as u32); CHANNELS])
        .split(inner);

    for i in 0..CHANNELS {
        let selected = i == app.selected;
        let block = panel(th, selected, "");
        let box_area = cells[i];
        let inside = block.inner(box_area);
        f.render_widget(block, box_area);

        // Keep one row for the label (top) and one for the value (bottom).
        let bar = Rect::new(
            inside.x,
            inside.y + 1,
            inside.width,
            inside.height.saturating_sub(2),
        );
        let c = &mut app.controls[i];
        c.area = bar;

        let slider = Slider::from_state(&c.state)
            .theme(th)
            .focused(selected)
            .orientation(SliderOrientation::Vertical)
            .label(c.name)
            .show_value(true)
            .vertical_label_position(VerticalLabelPosition::Top)
            .vertical_value_position(VerticalValuePosition::Bottom)
            .filled_symbol("┃")
            .empty_symbol("│")
            .handle_symbol("▬")
            .filled_color(c.color)
            .empty_color(Color::DarkGray)
            .handle_color(if selected { Color::White } else { c.color });
        f.render_widget(slider, bar);
    }
}

fn render_controls(f: &mut Frame, app: &mut App, th: &Theme, area: Rect) {
    let outer = panel(th, false, " Controls ");
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let n = app.controls.len() - CHANNELS;
    let cells = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Ratio(1, n as u32); n])
        .split(inner);

    for k in 0..n {
        let i = CHANNELS + k;
        let selected = i == app.selected;
        let block = panel(th, selected, &format!(" {} ", app.controls[i].name));
        let bar = block.inner(cells[k]);
        let c = &mut app.controls[i];
        c.area = bar;

        let mut slider = Slider::from_state(&c.state)
            .theme(th)
            .focused(selected)
            .orientation(SliderOrientation::Horizontal)
            .show_value(true)
            .filled_color(c.color)
            .empty_color(Color::DarkGray)
            .handle_color(if selected { Color::White } else { c.color })
            .block(block);
        slider = match c.look {
            Look::Segmented => slider
                .segmented(true)
                .filled_symbol("█")
                .empty_symbol("█")
                .show_handle(false),
            Look::Blocks => slider
                .filled_symbol("█")
                .empty_symbol("░")
                .handle_symbol("▌"),
            Look::Line | Look::Fader => slider
                .filled_symbol("━")
                .empty_symbol("─")
                .handle_symbol("●"),
        };
        f.render_widget(slider, cells[k]);
    }
}
