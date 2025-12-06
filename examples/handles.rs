//! Handle/Thumb visibility example
//!
//! This example demonstrates sliders with and without handles in a side-by-side comparison.
//! The left section shows sliders with handles (interactive controls),
//! the right section shows the same sliders WITHOUT handles (progress bar style).

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

struct SliderPair {
    label: String,
    state: SliderState,
    filled_symbol: &'static str,
    empty_symbol: &'static str,
    handle_symbol: &'static str,
    filled_color: Color,
    description: String,
}

struct App {
    pairs: Vec<SliderPair>,
    selected_section: usize, // 0 = left (with handles), 1 = right (without handles)
    selected_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            pairs: vec![
                SliderPair {
                    label: "Volume".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_THICK_LINE,
                    empty_symbol: symbols::EMPTY_THIN_LINE,
                    handle_symbol: symbols::HANDLE_CIRCLE,
                    filled_color: Color::Cyan,
                    description: "Circle handle".to_string(),
                },
                SliderPair {
                    label: "Balance".to_string(),
                    state: SliderState::new(50.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_DOUBLE_LINE,
                    empty_symbol: symbols::EMPTY_THIN_LINE,
                    handle_symbol: symbols::HANDLE_DOUBLE_CIRCLE,
                    filled_color: Color::Magenta,
                    description: "Double circle handle".to_string(),
                },
                SliderPair {
                    label: "Bass".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_BLOCK,
                    empty_symbol: symbols::FILLED_LIGHT_SHADE,
                    handle_symbol: symbols::HANDLE_SQUARE,
                    filled_color: Color::Green,
                    description: "Square handle".to_string(),
                },
                SliderPair {
                    label: "Treble".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_WAVE,
                    empty_symbol: symbols::EMPTY_WAVE,
                    handle_symbol: symbols::HANDLE_DIAMOND,
                    filled_color: Color::Blue,
                    description: "Diamond handle".to_string(),
                },
                SliderPair {
                    label: "Mix".to_string(),
                    state: SliderState::new(70.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_DARK_SHADE,
                    empty_symbol: symbols::FILLED_LIGHT_SHADE,
                    handle_symbol: symbols::HANDLE_HEXAGON,
                    filled_color: Color::Yellow,
                    description: "Hexagon handle".to_string(),
                },
                SliderPair {
                    label: "Progress".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_PROGRESS,
                    empty_symbol: symbols::EMPTY_PROGRESS,
                    handle_symbol: symbols::HANDLE_TRIANGLE_RIGHT,
                    filled_color: Color::LightYellow,
                    description: "Triangle handle".to_string(),
                },
                SliderPair {
                    label: "Health".to_string(),
                    state: SliderState::new(85.0, 0.0, 100.0),
                    filled_symbol: symbols::FILLED_DARK_SHADE,
                    empty_symbol: symbols::FILLED_LIGHT_SHADE,
                    handle_symbol: symbols::HANDLE_BULLSEYE,
                    filled_color: Color::Red,
                    description: "Bullseye handle".to_string(),
                },
            ],
            selected_section: 0,
            selected_index: 0,
        }
    }

    fn next(&mut self) {
        self.selected_index = (self.selected_index + 1) % self.pairs.len();
    }

    fn previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = self.pairs.len() - 1;
        }
    }

    fn switch_section(&mut self) {
        self.selected_section = if self.selected_section == 0 { 1 } else { 0 };
    }

    fn increase(&mut self) {
        if let Some(pair) = self.pairs.get_mut(self.selected_index) {
            pair.state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if let Some(pair) = self.pairs.get_mut(self.selected_index) {
            pair.state.decrease(5.0);
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
                    KeyCode::Tab => app.switch_section(),
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
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Handle/Thumb Visibility - Side-by-Side Comparison")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(vec![
        ratatui::text::Line::from(
            "Tab: Switch section (Left: with handles | Right: WITHOUT handles)",
        ),
        ratatui::text::Line::from("↑/↓ or j/k: Select slider | ←/→ or h/l: Adjust value"),
        ratatui::text::Line::from("Same sliders, different display - See the difference!"),
        ratatui::text::Line::from("q/ESC: Quit"),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Split content area into two columns
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Render left section (with handles)
    render_section(
        f,
        "WITH HANDLES (Interactive)",
        &app.pairs,
        app.selected_section == 0,
        app.selected_index,
        true,
        content_chunks[0],
    );

    // Render right section (without handles)
    render_section(
        f,
        "WITHOUT HANDLES (Progress)",
        &app.pairs,
        app.selected_section == 1,
        app.selected_index,
        false,
        content_chunks[1],
    );
}

fn render_section(
    f: &mut Frame,
    title: &str,
    pairs: &[SliderPair],
    is_active_section: bool,
    selected_index: usize,
    show_handles: bool,
    area: ratatui::layout::Rect,
) {
    // Section container
    let section_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_active_section {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .title(format!(" {} ", title));

    let inner_area = section_block.inner(area);
    f.render_widget(section_block, area);

    // Layout for sliders
    let num_sliders = pairs.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(4));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (i, pair) in pairs.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = is_active_section && i == selected_index;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(format!(" {} - {} ", pair.label, pair.description));

        let slider = Slider::from_state(&pair.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(pair.filled_symbol)
            .empty_symbol(pair.empty_symbol)
            .handle_symbol(pair.handle_symbol)
            .filled_color(pair.filled_color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                pair.filled_color
            })
            .show_value(true)
            .show_handle(show_handles)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
