//! Horizontal bar alignment example
//!
//! This example demonstrates vertical alignment options for horizontal sliders.
//! Bars can be positioned at the top, center (middle), or bottom of the slider area.

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
use tui_slider::{symbols, HorizontalBarAlignment, Slider, SliderOrientation, SliderState};

struct SliderExample {
    label: String,
    state: SliderState,
    color: Color,
    alignment: HorizontalBarAlignment,
    description: String,
}

struct App {
    examples: Vec<SliderExample>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            examples: vec![
                SliderExample {
                    label: "Top Aligned".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    color: Color::Cyan,
                    alignment: HorizontalBarAlignment::Top,
                    description: "Bar positioned at the top".to_string(),
                },
                SliderExample {
                    label: "Center Aligned".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    color: Color::Green,
                    alignment: HorizontalBarAlignment::Center,
                    description: "Bar positioned at the center (default)".to_string(),
                },
                SliderExample {
                    label: "Bottom Aligned".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    color: Color::Yellow,
                    alignment: HorizontalBarAlignment::Bottom,
                    description: "Bar positioned at the bottom".to_string(),
                },
                SliderExample {
                    label: "Progress Top".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    color: Color::Magenta,
                    alignment: HorizontalBarAlignment::Top,
                    description: "Progress bar style at top".to_string(),
                },
                SliderExample {
                    label: "Progress Center".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    color: Color::Blue,
                    alignment: HorizontalBarAlignment::Center,
                    description: "Progress bar style at center".to_string(),
                },
                SliderExample {
                    label: "Progress Bottom".to_string(),
                    state: SliderState::new(90.0, 0.0, 100.0),
                    color: Color::Red,
                    alignment: HorizontalBarAlignment::Bottom,
                    description: "Progress bar style at bottom".to_string(),
                },
            ],
            selected: 0,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.examples.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.examples.len() - 1;
        }
    }

    fn increase(&mut self) {
        if let Some(example) = self.examples.get_mut(self.selected) {
            example.state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if let Some(example) = self.examples.get_mut(self.selected) {
            example.state.decrease(5.0);
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
    let title = Paragraph::new("Horizontal Bar Alignment Examples")
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

    // Render examples
    render_examples(f, app, main_chunks[1]);
}

fn render_examples(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_examples = app.examples.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_examples {
        constraints.push(Constraint::Length(8));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, example) in app.examples.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = i == app.selected;
        let chunk = chunks[i + 1];

        let title = format!(" {} - {} ", example.label, example.description);

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
            .title(title);

        // Use different styles for first 3 vs last 3 examples
        let slider = if i < 3 {
            // Standard sliders with handles
            Slider::from_state(&example.state)
                .orientation(SliderOrientation::Horizontal)
                .filled_symbol(symbols::FILLED_THICK_LINE)
                .empty_symbol(symbols::EMPTY_THIN_LINE)
                .handle_symbol(symbols::HANDLE_CIRCLE)
                .filled_color(example.color)
                .empty_color(Color::DarkGray)
                .handle_color(if is_selected {
                    Color::White
                } else {
                    example.color
                })
                .show_value(true)
                .show_handle(true)
                .horizontal_bar_alignment(example.alignment)
                .block(block)
        } else {
            // Progress bar style
            Slider::from_state(&example.state)
                .orientation(SliderOrientation::Horizontal)
                .filled_symbol(symbols::FILLED_DARK_SHADE)
                .empty_symbol(symbols::FILLED_LIGHT_SHADE)
                .filled_color(example.color)
                .empty_color(Color::DarkGray)
                .show_value(true)
                .show_handle(false)
                .horizontal_bar_alignment(example.alignment)
                .block(block)
        };

        f.render_widget(slider, chunk);
    }
}
