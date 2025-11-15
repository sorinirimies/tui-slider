//! Step sizes example - Demonstrating configurable step intervals
//!
//! This example shows how to control the increment/decrement step size
//! when adjusting slider values with keyboard controls.

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
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct SliderConfig {
    label: String,
    state: SliderState,
    description: String,
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
                    label: "Fine Control".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 0.5),
                    description: "Step: 0.5 - Very precise adjustments".to_string(),
                    filled_color: Color::Cyan,
                },
                SliderConfig {
                    label: "Standard".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    description: "Step: 1.0 - Default one-by-one".to_string(),
                    filled_color: Color::Green,
                },
                SliderConfig {
                    label: "Medium Steps".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 2.5),
                    description: "Step: 2.5 - Moderate jumps".to_string(),
                    filled_color: Color::Yellow,
                },
                SliderConfig {
                    label: "Coarse".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 5.0),
                    description: "Step: 5.0 - Quick adjustments".to_string(),
                    filled_color: Color::Magenta,
                },
                SliderConfig {
                    label: "Large Jumps".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 10.0),
                    description: "Step: 10.0 - Big increments".to_string(),
                    filled_color: Color::Red,
                },
                SliderConfig {
                    label: "Quarter Steps".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 25.0),
                    description: "Step: 25.0 - Jump by quarters".to_string(),
                    filled_color: Color::Blue,
                },
                SliderConfig {
                    label: "Percentage".to_string(),
                    state: SliderState::with_step(0.5, 0.0, 1.0, 0.01),
                    description: "Step: 0.01 - Range 0-1 in 1% increments".to_string(),
                    filled_color: Color::LightCyan,
                },
                SliderConfig {
                    label: "Volume dB".to_string(),
                    state: SliderState::with_step(0.0, -60.0, 0.0, 0.5),
                    description: "Step: 0.5 - Range -60 to 0 dB".to_string(),
                    filled_color: Color::LightGreen,
                },
                SliderConfig {
                    label: "Temperature".to_string(),
                    state: SliderState::with_step(20.0, -20.0, 50.0, 0.1),
                    description: "Step: 0.1 - Range -20°C to 50°C".to_string(),
                    filled_color: Color::LightRed,
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
            slider.state.step_up();
        }
    }

    fn decrease(&mut self) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            slider.state.step_down();
        }
    }

    fn change_step(&mut self, new_step: f64) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            slider.state.set_step(new_step);
            slider.description = format!("Step: {} - Custom step size", new_step);
        }
    }

    fn double_step(&mut self) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            let new_step = (slider.state.step() * 2.0).min(50.0);
            slider.state.set_step(new_step);
            slider.description = format!("Step: {} - Doubled", new_step);
        }
    }

    fn halve_step(&mut self) {
        if let Some(slider) = self.sliders.get_mut(self.selected) {
            let new_step = (slider.state.step() / 2.0).max(0.1);
            slider.state.set_step(new_step);
            slider.description = format!("Step: {} - Halved", new_step);
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
                    KeyCode::Char('1') => app.change_step(1.0),
                    KeyCode::Char('2') => app.change_step(2.0),
                    KeyCode::Char('5') => app.change_step(5.0),
                    KeyCode::Char('+') | KeyCode::Char('=') => app.double_step(),
                    KeyCode::Char('-') | KeyCode::Char('_') => app.halve_step(),
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
            Constraint::Length(6),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Step Sizes - Configurable Increment/Decrement Intervals")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help_lines = vec![
        Line::from(vec![
            Span::styled("↑/↓ or j/k", Style::default().fg(Color::Yellow)),
            Span::raw(": Select slider  "),
            Span::styled("←/→ or h/l", Style::default().fg(Color::Yellow)),
            Span::raw(": Adjust value (uses configured step)"),
        ]),
        Line::from(vec![
            Span::styled("1", Style::default().fg(Color::Yellow)),
            Span::raw(": Set step to 1.0  "),
            Span::styled("2", Style::default().fg(Color::Yellow)),
            Span::raw(": Set step to 2.0  "),
            Span::styled("5", Style::default().fg(Color::Yellow)),
            Span::raw(": Set step to 5.0"),
        ]),
        Line::from(vec![
            Span::styled("+", Style::default().fg(Color::Yellow)),
            Span::raw(": Double step size  "),
            Span::styled("-", Style::default().fg(Color::Yellow)),
            Span::raw(": Halve step size  "),
            Span::styled("q/ESC", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
    ];

    let help = Paragraph::new(help_lines)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Controls "),
        );
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

        let title = format!(" {} - {} ", config.label, config.description);

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

        let slider = Slider::from_state(&config.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol("━")
            .empty_symbol("─")
            .handle_symbol("●")
            .filled_color(config.filled_color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                Color::Gray
            })
            .show_value(true)
            .show_handle(true)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
