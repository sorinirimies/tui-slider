//! Vertical slider positioning example
//!
//! This example demonstrates different label and value positioning options for vertical sliders.
//! Labels can be positioned at the top or bottom, values can be positioned at top/middle/bottom,
//! and values can be aligned left/center/right.

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
use tui_slider::{
    Slider, SliderOrientation, SliderState, VerticalLabelPosition, VerticalValueAlignment,
    VerticalValuePosition,
};

struct SliderExample {
    label: String,
    state: SliderState,
    color: Color,
    label_position: VerticalLabelPosition,
    value_position: VerticalValuePosition,
    value_alignment: VerticalValueAlignment,
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
                    label: "Volume".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    color: Color::Cyan,
                    label_position: VerticalLabelPosition::Top,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Center,
                    description: "Label: Top, Value: Bottom/Center".to_string(),
                },
                SliderExample {
                    label: "Bass".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    color: Color::Green,
                    label_position: VerticalLabelPosition::Bottom,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Center,
                    description: "Label: Bottom, Value: Top/Center".to_string(),
                },
                SliderExample {
                    label: "Treble".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    color: Color::Yellow,
                    label_position: VerticalLabelPosition::Top,
                    value_position: VerticalValuePosition::Middle,
                    value_alignment: VerticalValueAlignment::Left,
                    description: "Label: Top, Value: Middle/Left".to_string(),
                },
                SliderExample {
                    label: "Delay".to_string(),
                    state: SliderState::new(30.0, 0.0, 100.0),
                    color: Color::Magenta,
                    label_position: VerticalLabelPosition::Top,
                    value_position: VerticalValuePosition::Middle,
                    value_alignment: VerticalValueAlignment::Right,
                    description: "Label: Top, Value: Middle/Right".to_string(),
                },
                SliderExample {
                    label: "Reverb".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    color: Color::Blue,
                    label_position: VerticalLabelPosition::Bottom,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Left,
                    description: "Label: Bottom, Value: Bottom/Left".to_string(),
                },
                SliderExample {
                    label: "Chorus".to_string(),
                    state: SliderState::new(65.0, 0.0, 100.0),
                    color: Color::Red,
                    label_position: VerticalLabelPosition::Top,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Left,
                    description: "Label: Top, Value: Top/Left".to_string(),
                },
                SliderExample {
                    label: "Gain".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    color: Color::LightCyan,
                    label_position: VerticalLabelPosition::Bottom,
                    value_position: VerticalValuePosition::Middle,
                    value_alignment: VerticalValueAlignment::Center,
                    description: "Label: Bottom, Value: Middle/Center".to_string(),
                },
                SliderExample {
                    label: "Attack".to_string(),
                    state: SliderState::new(40.0, 0.0, 100.0),
                    color: Color::LightGreen,
                    label_position: VerticalLabelPosition::Top,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Right,
                    description: "Label: Top, Value: Bottom/Right".to_string(),
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
            Constraint::Length(4), // Space for descriptions
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Vertical Slider Positioning Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);

    // Render descriptions
    render_descriptions(f, app, main_chunks[2]);

    // Help text
    let help = Paragraph::new("←/→ or h/l: Select | ↑/↓ or k/j: Adjust | q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[3]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.examples.len();
    let slider_width = 14;
    let spacing = 2;

    let mut constraints = vec![];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }
    constraints.push(Constraint::Min(0));

    let slider_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, example) in app.examples.iter().enumerate() {
        let chunk_index = i * 2;
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
            });

        let inner_area = block.inner(slider_chunks[chunk_index]);
        f.render_widget(block, slider_chunks[chunk_index]);

        // Create slider with positioning options
        // Hide handle for last 2 examples (Gain and Attack)
        let show_handle = i < 6;
        let slider = Slider::from_state(&example.state)
            .orientation(SliderOrientation::Vertical)
            .label(&example.label)
            .show_value(true)
            .filled_symbol("│")
            .empty_symbol("│")
            .handle_symbol("━")
            .filled_color(example.color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                example.color
            })
            .show_handle(show_handle)
            .vertical_label_position(example.label_position)
            .vertical_value_position(example.value_position)
            .vertical_value_alignment(example.value_alignment);

        f.render_widget(slider, inner_area);
    }
}

fn render_descriptions(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.examples.len();
    let slider_width = 14;
    let spacing = 2;

    let mut constraints = vec![];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }
    constraints.push(Constraint::Min(0));

    let desc_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, example) in app.examples.iter().enumerate() {
        let chunk_index = i * 2;
        if chunk_index >= desc_chunks.len() {
            break;
        }

        let is_selected = i == app.selected;

        let desc = Paragraph::new(example.description.as_str())
            .style(if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(desc, desc_chunks[chunk_index]);
    }
}
