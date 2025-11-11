//! Vertical and Horizontal slider example
//!
//! This example demonstrates both vertical and horizontal sliders with consistent styling.

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

/// Configuration for consistent slider styling
#[derive(Debug, Clone)]
struct SliderConfig {
    filled_symbol: &'static str,
    empty_symbol: &'static str,
    handle_symbol: &'static str,
}

impl SliderConfig {
    fn default() -> Self {
        Self {
            filled_symbol: "━",
            empty_symbol: "─",
            handle_symbol: "●",
        }
    }
}

struct App {
    horizontal_sliders: Vec<(String, SliderState, Color)>,
    vertical_sliders: Vec<(String, SliderState, Color)>,
    selected_horizontal: usize,
    selected_vertical: usize,
    focus_horizontal: bool,
    config: SliderConfig,
}

impl App {
    fn new() -> Self {
        Self {
            horizontal_sliders: vec![
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
                    Color::Yellow,
                ),
                (
                    "Balance".to_string(),
                    SliderState::new(50.0, 0.0, 100.0),
                    Color::Magenta,
                ),
            ],
            vertical_sliders: vec![
                (
                    "Bass".to_string(),
                    SliderState::new(90.0, 0.0, 100.0),
                    Color::Red,
                ),
                (
                    "Low Mid".to_string(),
                    SliderState::new(55.0, 0.0, 100.0),
                    Color::Yellow,
                ),
                (
                    "Mid".to_string(),
                    SliderState::new(50.0, 0.0, 100.0),
                    Color::Green,
                ),
                (
                    "High Mid".to_string(),
                    SliderState::new(80.0, 0.0, 100.0),
                    Color::Cyan,
                ),
                (
                    "Treble".to_string(),
                    SliderState::new(43.0, 0.0, 100.0),
                    Color::Blue,
                ),
            ],
            selected_horizontal: 0,
            selected_vertical: 0,
            focus_horizontal: false,
            config: SliderConfig::default(),
        }
    }

    fn toggle_focus(&mut self) {
        self.focus_horizontal = !self.focus_horizontal;
    }

    fn next(&mut self) {
        if self.focus_horizontal {
            self.selected_horizontal =
                (self.selected_horizontal + 1) % self.horizontal_sliders.len();
        } else {
            self.selected_vertical = (self.selected_vertical + 1) % self.vertical_sliders.len();
        }
    }

    fn previous(&mut self) {
        if self.focus_horizontal {
            if self.selected_horizontal > 0 {
                self.selected_horizontal -= 1;
            } else {
                self.selected_horizontal = self.horizontal_sliders.len() - 1;
            }
        } else if self.selected_vertical > 0 {
            self.selected_vertical -= 1;
        } else {
            self.selected_vertical = self.vertical_sliders.len() - 1;
        }
    }

    fn increase(&mut self) {
        if self.focus_horizontal {
            if let Some((_, state, _)) = self.horizontal_sliders.get_mut(self.selected_horizontal) {
                state.increase(5.0);
            }
        } else if let Some((_, state, _)) = self.vertical_sliders.get_mut(self.selected_vertical) {
            state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if self.focus_horizontal {
            if let Some((_, state, _)) = self.horizontal_sliders.get_mut(self.selected_horizontal) {
                state.decrease(5.0);
            }
        } else if let Some((_, state, _)) = self.vertical_sliders.get_mut(self.selected_vertical) {
            state.decrease(5.0);
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
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Right | KeyCode::Char('l') => app.next(),
                    KeyCode::Left | KeyCode::Char('h') => app.previous(),
                    KeyCode::Up | KeyCode::Char('k') => app.increase(),
                    KeyCode::Down | KeyCode::Char('j') => app.decrease(),
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
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Slider Demo - Vertical & Horizontal")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Vertical section title
    let v_title = Paragraph::new(if !app.focus_horizontal {
        "Vertical Sliders / Equalizer [ACTIVE]"
    } else {
        "Vertical Sliders / Equalizer"
    })
    .style(if !app.focus_horizontal {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    })
    .alignment(Alignment::Center);
    f.render_widget(v_title, main_chunks[2]);

    // Horizontal section title
    let h_title = Paragraph::new(if app.focus_horizontal {
        "Horizontal Sliders [ACTIVE]"
    } else {
        "Horizontal Sliders"
    })
    .style(if app.focus_horizontal {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    })
    .alignment(Alignment::Center);
    f.render_widget(h_title, main_chunks[4]);

    // Help text
    let help = Paragraph::new(
        "Tab: Switch section | ←/→ or h/l: Select | ↑/↓ or k/j: Adjust | q/Esc: Quit",
    )
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[4]);

    // Render vertical sliders
    render_vertical_sliders(f, app, main_chunks[1]);

    // Render horizontal sliders
    render_horizontal_sliders(f, app, main_chunks[3]);
}

fn render_vertical_sliders(f: &mut Frame, app: &App, area: Rect) {
    let num_sliders = app.vertical_sliders.len();
    let slider_width = 12;
    let total_width = slider_width * num_sliders as u16;
    let remaining = area.width.saturating_sub(total_width);
    let spacing = remaining / (num_sliders as u16 + 1);

    let mut constraints = vec![Constraint::Length(spacing)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }

    let slider_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, (label, state, color)) in app.vertical_sliders.iter().enumerate() {
        let chunk_index = 1 + (i * 2);
        if chunk_index >= slider_chunks.len() {
            break;
        }

        let is_selected = !app.focus_horizontal && i == app.selected_vertical;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(Line::from(vec![Span::styled(
                label.clone(),
                if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                },
            )]));

        let inner_area = block.inner(slider_chunks[chunk_index]);
        f.render_widget(block, slider_chunks[chunk_index]);

        let slider = Slider::from_state(state)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol(app.config.filled_symbol)
            .empty_symbol(app.config.empty_symbol)
            .handle_symbol(app.config.handle_symbol)
            .filled_color(*color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected { Color::White } else { *color })
            .show_handle(true);

        f.render_widget(slider, inner_area);

        let value_text = format!("{:.0}", state.value());
        let value_para = Paragraph::new(value_text)
            .style(if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            })
            .alignment(Alignment::Center);

        if inner_area.height > 2 {
            let value_area = Rect {
                x: inner_area.x,
                y: inner_area.y + inner_area.height - 1,
                width: inner_area.width,
                height: 1,
            };
            f.render_widget(value_para, value_area);
        }
    }
}

fn render_horizontal_sliders(f: &mut Frame, app: &App, area: Rect) {
    let num_sliders = app.horizontal_sliders.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, (label, state, color)) in app.horizontal_sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = app.focus_horizontal && i == app.selected_horizontal;

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
            .filled_symbol(app.config.filled_symbol)
            .empty_symbol(app.config.empty_symbol)
            .handle_symbol(app.config.handle_symbol)
            .filled_color(*color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected { Color::White } else { *color })
            .show_value(true)
            .show_handle(true)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
