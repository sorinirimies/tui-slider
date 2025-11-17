//! Vertical slider styles example
//!
//! This example demonstrates various vertical slider styles including clean lines,
//! blocks, gradients, dots, squares, and equalizer bars.

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
use tui_slider::style::SliderStyle;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct App {
    sliders: Vec<(String, SliderState, SliderStyle)>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            sliders: vec![
                (
                    "Clean".to_string(),
                    SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical(),
                ),
                (
                    "Blocks".to_string(),
                    SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical_blocks(),
                ),
                (
                    "Gradient".to_string(),
                    SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical_gradient(),
                ),
                (
                    "Dots".to_string(),
                    SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical_dots(),
                ),
                (
                    "Squares".to_string(),
                    SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical_squares(),
                ),
                (
                    "Equalizer".to_string(),
                    SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    SliderStyle::vertical_equalizer(),
                ),
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
        if let Some((_, state, _)) = self.sliders.get_mut(self.selected) {
            state.step_up();
        }
    }

    fn decrease(&mut self) {
        if let Some((_, state, _)) = self.sliders.get_mut(self.selected) {
            state.step_down();
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
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Vertical Slider Styles Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new("←/→ or h/l: Select | ↑/↓ or k/j: Adjust | q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.sliders.len();
    let slider_width = 16;
    let spacing = 2;
    let total_width = (slider_width + spacing) * num_sliders as u16;

    // Center the sliders if there's extra space
    let remaining = area.width.saturating_sub(total_width);
    let left_margin = remaining / 2;

    let mut constraints = vec![Constraint::Length(left_margin)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }
    constraints.push(Constraint::Min(0));

    let slider_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, (label, state, style)) in app.sliders.iter().enumerate() {
        let chunk_index = 1 + (i * 2);
        if chunk_index >= slider_chunks.len() {
            break;
        }

        let is_selected = i == app.selected;

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
            .title(ratatui::text::Line::from(vec![
                ratatui::text::Span::styled(
                    label.clone(),
                    if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                ),
            ]));

        let inner_area = block.inner(slider_chunks[chunk_index]);
        f.render_widget(block, slider_chunks[chunk_index]);

        // Reserve space for value at bottom
        let slider_area = ratatui::layout::Rect {
            x: inner_area.x,
            y: inner_area.y,
            width: inner_area.width,
            height: inner_area.height.saturating_sub(2),
        };

        let slider = Slider::from_state(state)
            .orientation(SliderOrientation::Vertical)
            .filled_symbol(style.filled_symbol)
            .empty_symbol(style.empty_symbol)
            .handle_symbol(style.handle_symbol)
            .filled_color(style.filled_color)
            .empty_color(style.empty_color)
            .handle_color(if is_selected {
                Color::White
            } else {
                style.handle_color
            })
            .show_handle(true);

        f.render_widget(slider, slider_area);

        // Render value below slider in separate area
        let value_area = ratatui::layout::Rect {
            x: inner_area.x,
            y: inner_area.y + slider_area.height,
            width: inner_area.width,
            height: 1,
        };

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

        f.render_widget(value_para, value_area);
    }
}
