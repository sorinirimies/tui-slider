//! Thumb toggle example
//!
//! This example demonstrates how to show or hide the slider thumb/handle indicator.
//! Press Space to toggle the thumb visibility on all sliders.

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct App {
    sliders: Vec<(String, SliderState, Color)>,
    selected: usize,
    show_thumbs: bool,
}

impl App {
    fn new() -> Self {
        Self {
            sliders: vec![
                (
                    "Volume".to_string(),
                    SliderState::new(75.0, 0.0, 100.0),
                    Color::Cyan,
                ),
                (
                    "Bass".to_string(),
                    SliderState::new(60.0, 0.0, 100.0),
                    Color::Green,
                ),
                (
                    "Treble".to_string(),
                    SliderState::new(55.0, 0.0, 100.0),
                    Color::Blue,
                ),
                (
                    "Balance".to_string(),
                    SliderState::new(50.0, 0.0, 100.0),
                    Color::Yellow,
                ),
                (
                    "Gain".to_string(),
                    SliderState::new(35.0, 0.0, 100.0),
                    Color::Red,
                ),
                (
                    "Reverb".to_string(),
                    SliderState::new(45.0, 0.0, 100.0),
                    Color::Magenta,
                ),
            ],
            selected: 0,
            show_thumbs: true,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.sliders.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.sliders.len() - 1;
        }
    }

    fn increase(&mut self) {
        if let Some((_, state, _)) = self.sliders.get_mut(self.selected) {
            state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if let Some((_, state, _)) = self.sliders.get_mut(self.selected) {
            state.decrease(5.0);
        }
    }

    fn toggle_thumbs(&mut self) {
        self.show_thumbs = !self.show_thumbs;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Right | KeyCode::Char('l') => app.increase(),
                    KeyCode::Left | KeyCode::Char('h') => app.decrease(),
                    KeyCode::Char(' ') => app.toggle_thumbs(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Thumb/Handle Toggle Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let thumb_status = if app.show_thumbs {
        "ON (visible)"
    } else {
        "OFF (hidden)"
    };
    let help_text = format!(
        "Thumb Indicators: {} - Press SPACE to toggle\n\n↑/↓ or j/k: Select | ←/→ or h/l: Adjust | q/Esc: Quit",
        thumb_status
    );
    let help_widget = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help_widget, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.sliders.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(4));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, (label, state, color)) in app.sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = i == app.selected;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(format!(" {} ", label));

        let slider = Slider::from_state(state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol("━")
            .empty_symbol("─")
            .handle_symbol("●")
            .filled_color(*color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected { Color::White } else { *color })
            .show_value(true)
            .show_thumb(app.show_thumbs) // Toggle thumb visibility
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
