//! Horizontal slider styles example
//!
//! This example demonstrates various horizontal slider styles including clean lines,
//! thick lines, blocks, gradients, dots, squares, and double lines.

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
                    SliderStyle::horizontal(),
                ),
                (
                    "Thick".to_string(),
                    SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_thick(),
                ),
                (
                    "Blocks".to_string(),
                    SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_blocks(),
                ),
                (
                    "Gradient".to_string(),
                    SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_gradient(),
                ),
                (
                    "Dots".to_string(),
                    SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_dots(),
                ),
                (
                    "Squares".to_string(),
                    SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_squares(),
                ),
                (
                    "Double".to_string(),
                    SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    SliderStyle::horizontal_double(),
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
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Right | KeyCode::Char('l') => app.increase(),
                    KeyCode::Left | KeyCode::Char('h') => app.decrease(),
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
    let title = Paragraph::new("Horizontal Slider Styles Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new("↑/↓ or j/k: Select | ←/→ or h/l: Adjust | q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
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

    for (i, (label, state, style)) in app.sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
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
                ratatui::text::Span::raw(" "),
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
                ratatui::text::Span::raw(" - "),
                ratatui::text::Span::styled(
                    style.name,
                    if is_selected {
                        Style::default().fg(Color::White)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    },
                ),
                ratatui::text::Span::raw(" "),
            ]));

        let slider = Slider::from_state(state)
            .orientation(SliderOrientation::Horizontal)
            .label(label)
            .show_value(true)
            .value_alignment(Alignment::Right)
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
            .show_handle(true)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
