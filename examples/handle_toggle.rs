//! Example demonstrating the show_handle/show_thumb toggle functionality
//!
//! This example shows how to toggle the visibility of the slider handle/thumb indicator.
//! Press 'h' to toggle the handle visibility on the selected slider.

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
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct SliderConfig {
    label: String,
    state: SliderState,
    show_handle: bool,
    filled_symbol: &'static str,
    empty_symbol: &'static str,
    handle_symbol: &'static str,
    filled_color: Color,
}

struct App {
    sliders: Vec<SliderConfig>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            sliders: vec![
                SliderConfig {
                    label: "Volume - With Handle".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    show_handle: true,
                    filled_symbol: "━",
                    empty_symbol: "─",
                    handle_symbol: "●",
                    filled_color: Color::Cyan,
                },
                SliderConfig {
                    label: "Progress - Without Handle".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    show_handle: false,
                    filled_symbol: "█",
                    empty_symbol: "░",
                    handle_symbol: "▓",
                    filled_color: Color::Green,
                },
                SliderConfig {
                    label: "Loading - No Handle".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    show_handle: false,
                    filled_symbol: "▰",
                    empty_symbol: "▱",
                    handle_symbol: "▶",
                    filled_color: Color::Yellow,
                },
                SliderConfig {
                    label: "Balance - With Handle".to_string(),
                    state: SliderState::new(50.0, 0.0, 100.0),
                    show_handle: true,
                    filled_symbol: "═",
                    empty_symbol: "─",
                    handle_symbol: "◉",
                    filled_color: Color::Magenta,
                },
                SliderConfig {
                    label: "Health Bar - No Handle".to_string(),
                    state: SliderState::new(85.0, 0.0, 100.0),
                    show_handle: false,
                    filled_symbol: "▓",
                    empty_symbol: "░",
                    handle_symbol: "■",
                    filled_color: Color::Red,
                },
            ],
            selected: 0,
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
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            slider.state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            slider.state.decrease(5.0);
        }
    }

    fn toggle_handle(&mut self) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            slider.show_handle = !slider.show_handle;
        }
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
                    KeyCode::Char('H') => app.toggle_handle(),
                    KeyCode::Char(' ') => app.toggle_handle(),
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
    let title = Paragraph::new("Handle/Thumb Toggle Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help_text = [
        "↑/↓ or j/k: Select slider",
        "←/→ or h/l: Adjust value",
        "SHIFT+H or SPACE: Toggle handle visibility",
        "q/Esc: Quit",
    ];
    let help = Paragraph::new(help_text.join(" | "))
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(help, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.sliders.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, config) in app.sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = i == app.selected;

        let handle_status = if config.show_handle {
            "Handle: ON"
        } else {
            "Handle: OFF"
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(format!(" {} - {} ", config.label, handle_status));

        let slider = Slider::from_state(&config.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(config.filled_symbol)
            .empty_symbol(config.empty_symbol)
            .handle_symbol(config.handle_symbol)
            .filled_color(config.filled_color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                Color::LightYellow
            })
            .show_value(true)
            .show_handle(config.show_handle)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
