//! Border styles example - Demonstrating different border types
//!
//! This example shows the various border styles available in ratatui,
//! including rounded corners, thick borders, double lines, and more.

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

struct BorderExample {
    label: String,
    border_type: BorderType,
    state: SliderState,
    description: String,
    color: Color,
}

struct App {
    examples: Vec<BorderExample>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            examples: vec![
                BorderExample {
                    label: "Plain".to_string(),
                    border_type: BorderType::Plain,
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    description: "Basic straight lines".to_string(),
                    color: Color::Cyan,
                },
                BorderExample {
                    label: "Rounded".to_string(),
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    description: "Smooth rounded corners".to_string(),
                    color: Color::Green,
                },
                BorderExample {
                    label: "Double".to_string(),
                    border_type: BorderType::Double,
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    description: "Elegant double lines".to_string(),
                    color: Color::Yellow,
                },
                BorderExample {
                    label: "Thick".to_string(),
                    border_type: BorderType::Thick,
                    state: SliderState::with_step(40.0, 0.0, 100.0, 1.0),
                    description: "Bold thick borders".to_string(),
                    color: Color::Magenta,
                },
                BorderExample {
                    label: "QuadrantInside".to_string(),
                    border_type: BorderType::QuadrantInside,
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    description: "Quadrant blocks inside".to_string(),
                    color: Color::Blue,
                },
                BorderExample {
                    label: "QuadrantOutside".to_string(),
                    border_type: BorderType::QuadrantOutside,
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    description: "Quadrant blocks outside".to_string(),
                    color: Color::Red,
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
            example.state.step_up();
        }
    }

    fn decrease(&mut self) {
        if let Some(example) = self.examples.get_mut(self.selected) {
            example.state.step_down();
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
    let title = Paragraph::new("Border Styles - Rounded Corners & More")
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

    // Render border examples
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

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(example.border_type)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(format!(" {} - {} ", example.label, example.description));

        let slider = Slider::from_state(&example.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol("━")
            .empty_symbol("─")
            .handle_symbol("●")
            .filled_color(example.color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                example.color
            })
            .show_value(true)
            .show_handle(true)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
