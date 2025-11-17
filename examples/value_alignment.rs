//! Value alignment example
//!
//! This example demonstrates different value alignment options for sliders.
//! The value can be positioned left, center, or right above the slider.

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
use tui_slider::border::title_center;
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct SliderExample {
    label: String,
    state: SliderState,
    color: Color,
    value_alignment: Alignment,
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
                    label: "Value Left".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    color: Color::Cyan,
                    value_alignment: Alignment::Left,
                    description: "Value aligned to the left".to_string(),
                },
                SliderExample {
                    label: "Value Center".to_string(),
                    state: SliderState::new(65.0, 0.0, 100.0),
                    color: Color::Green,
                    value_alignment: Alignment::Center,
                    description: "Value aligned to the center".to_string(),
                },
                SliderExample {
                    label: "Value Right".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    color: Color::Yellow,
                    value_alignment: Alignment::Right,
                    description: "Value aligned to the right (default)".to_string(),
                },
                SliderExample {
                    label: "Progress Bar (Left)".to_string(),
                    state: SliderState::new(42.0, 0.0, 100.0),
                    color: Color::Magenta,
                    value_alignment: Alignment::Left,
                    description: "Progress bar with left value".to_string(),
                },
                SliderExample {
                    label: "Progress Bar (Center)".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    color: Color::Blue,
                    value_alignment: Alignment::Center,
                    description: "Progress bar with centered value".to_string(),
                },
                SliderExample {
                    label: "Progress Bar (Right)".to_string(),
                    state: SliderState::new(33.0, 0.0, 100.0),
                    color: Color::Red,
                    value_alignment: Alignment::Right,
                    description: "Progress bar with right value".to_string(),
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
    let title = Paragraph::new("Value Alignment Examples")
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
        constraints.push(Constraint::Length(5));
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

        // Different rendering for regular sliders vs progress bars
        if i < 3 {
            render_regular_slider(f, example, is_selected, chunk);
        } else {
            render_progress_bar(f, example, is_selected, chunk);
        }
    }
}

fn render_regular_slider(
    f: &mut Frame,
    example: &SliderExample,
    is_selected: bool,
    area: ratatui::layout::Rect,
) {
    let title = title_center(format!(" {} - {} ", example.label, example.description));

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

    let slider = Slider::from_state(&example.state)
        .orientation(SliderOrientation::Horizontal)
        .label(&example.label)
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
        .value_alignment(example.value_alignment)
        .show_handle(true)
        .block(block);

    f.render_widget(slider, area);
}

fn render_progress_bar(
    f: &mut Frame,
    example: &SliderExample,
    is_selected: bool,
    area: ratatui::layout::Rect,
) {
    let title = title_center(format!(" {} - {} ", example.label, example.description));

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .title(title);

    let slider = Slider::from_state(&example.state)
        .orientation(SliderOrientation::Horizontal)
        .label(&example.label)
        .filled_symbol(symbols::FILLED_DARK_SHADE)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(example.color)
        .empty_color(Color::DarkGray)
        .show_value(true)
        .value_alignment(example.value_alignment)
        .show_handle(false) // Progress bar style - no handle
        .block(block);

    f.render_widget(slider, area);
}
