//! Border colors example - Demonstrating border tinting and styling
//!
//! This example shows how to color and style borders with different colors,
//! gradients, and effects to match your UI theme.

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
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct ColorExample {
    label: String,
    state: SliderState,
    border_color: Color,
    border_type: BorderType,
    filled_color: Color,
    description: String,
    use_bold: bool,
}

struct App {
    examples: Vec<ColorExample>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            examples: vec![
                ColorExample {
                    label: "Cyan".to_string(),
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    border_color: Color::Cyan,
                    border_type: BorderType::Rounded,
                    filled_color: Color::Cyan,
                    description: "Cool cyan theme".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Green".to_string(),
                    state: SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    border_color: Color::Green,
                    border_type: BorderType::Rounded,
                    filled_color: Color::Green,
                    description: "Success/healthy green".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Yellow".to_string(),
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    border_color: Color::Yellow,
                    border_type: BorderType::Rounded,
                    filled_color: Color::Yellow,
                    description: "Warning yellow".to_string(),
                    use_bold: true,
                },
                ColorExample {
                    label: "Red".to_string(),
                    state: SliderState::with_step(30.0, 0.0, 100.0, 1.0),
                    border_color: Color::Red,
                    border_type: BorderType::Thick,
                    filled_color: Color::Red,
                    description: "Critical/danger red".to_string(),
                    use_bold: true,
                },
                ColorExample {
                    label: "Magenta".to_string(),
                    state: SliderState::with_step(85.0, 0.0, 100.0, 1.0),
                    border_color: Color::Magenta,
                    border_type: BorderType::Rounded,
                    filled_color: Color::Magenta,
                    description: "Creative magenta".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Blue".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    border_color: Color::Blue,
                    border_type: BorderType::Double,
                    filled_color: Color::Blue,
                    description: "Info blue".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "RGB Custom".to_string(),
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    border_color: Color::Rgb(255, 100, 150),
                    border_type: BorderType::Rounded,
                    filled_color: Color::Rgb(255, 100, 150),
                    description: "Custom RGB pink".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Light Gray".to_string(),
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    border_color: Color::Gray,
                    border_type: BorderType::Rounded,
                    filled_color: Color::White,
                    description: "Subtle neutral".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Dark Gray".to_string(),
                    state: SliderState::with_step(40.0, 0.0, 100.0, 1.0),
                    border_color: Color::DarkGray,
                    border_type: BorderType::Rounded,
                    filled_color: Color::Gray,
                    description: "Muted disabled".to_string(),
                    use_bold: false,
                },
                ColorExample {
                    label: "Gradient RGB".to_string(),
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    border_color: Color::Rgb(100, 200, 255),
                    border_type: BorderType::Rounded,
                    filled_color: Color::Rgb(50, 150, 255),
                    description: "Ocean gradient".to_string(),
                    use_bold: false,
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

    fn cycle_border_color(&mut self) {
        if let Some(example) = self.examples.get_mut(self.selected) {
            example.border_color = match example.border_color {
                Color::Cyan => Color::Green,
                Color::Green => Color::Yellow,
                Color::Yellow => Color::Red,
                Color::Red => Color::Magenta,
                Color::Magenta => Color::Blue,
                Color::Blue => Color::White,
                Color::White => Color::Gray,
                Color::Gray => Color::Cyan,
                _ => Color::Cyan,
            };
        }
    }

    fn toggle_bold(&mut self) {
        if let Some(example) = self.examples.get_mut(self.selected) {
            example.use_bold = !example.use_bold;
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
                    KeyCode::Char('c') => app.cycle_border_color(),
                    KeyCode::Char('b') => app.toggle_bold(),
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
            Constraint::Length(4),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Border Colors - Tinting & Styling Borders")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(vec![
        ratatui::text::Line::from("↑/↓ or j/k: Select | ←/→ or h/l: Adjust value"),
        ratatui::text::Line::from("c: Cycle border color | b: Toggle bold | q/ESC: Quit"),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_examples = app.examples.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_examples {
        constraints.push(Constraint::Length(4));
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

        // Create border style with color and optional bold
        let mut border_style = Style::default().fg(if is_selected {
            Color::Yellow
        } else {
            example.border_color
        });

        if example.use_bold || is_selected {
            border_style = border_style.add_modifier(Modifier::BOLD);
        }

        let bold_indicator = if example.use_bold { " [BOLD]" } else { "" };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(example.border_type)
            .border_style(border_style)
            .title(format!(
                " {} - {}{} ",
                example.label, example.description, bold_indicator
            ));

        let slider = Slider::from_state(&example.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(symbols::FILLED_THICK_LINE)
            .empty_symbol(symbols::EMPTY_THIN_LINE)
            .handle_symbol(symbols::HANDLE_CIRCLE)
            .filled_color(example.filled_color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                example.filled_color
            })
            .show_value(true)
            .show_handle(true)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
