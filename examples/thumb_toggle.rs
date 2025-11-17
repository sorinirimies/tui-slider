//! Thumb/Handle visibility example
//!
//! This example demonstrates sliders with and without thumb/handle indicators.
//! The top sliders show thumbs, the bottom sliders hide them.

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
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct SliderGroup {
    label: String,
    state: SliderState,
    color: Color,
}

struct App {
    with_thumbs: Vec<SliderGroup>,
    without_thumbs: Vec<SliderGroup>,
    selected_section: usize, // 0 = with thumbs, 1 = without thumbs
    selected_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            with_thumbs: vec![
                SliderGroup {
                    label: "Volume".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    color: Color::Cyan,
                },
                SliderGroup {
                    label: "Bass".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    color: Color::Green,
                },
                SliderGroup {
                    label: "Treble".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    color: Color::Blue,
                },
            ],
            without_thumbs: vec![
                SliderGroup {
                    label: "Progress".to_string(),
                    state: SliderState::new(65.0, 0.0, 100.0),
                    color: Color::Yellow,
                },
                SliderGroup {
                    label: "Loading".to_string(),
                    state: SliderState::new(40.0, 0.0, 100.0),
                    color: Color::Magenta,
                },
                SliderGroup {
                    label: "Status".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    color: Color::Red,
                },
            ],
            selected_section: 0,
            selected_index: 0,
        }
    }

    fn next(&mut self) {
        let max_index = if self.selected_section == 0 {
            self.with_thumbs.len()
        } else {
            self.without_thumbs.len()
        };
        self.selected_index = (self.selected_index + 1) % max_index;
    }

    fn previous(&mut self) {
        let max_index = if self.selected_section == 0 {
            self.with_thumbs.len()
        } else {
            self.without_thumbs.len()
        };
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = max_index - 1;
        }
    }

    fn switch_section(&mut self) {
        self.selected_section = if self.selected_section == 0 { 1 } else { 0 };
        let max_index = if self.selected_section == 0 {
            self.with_thumbs.len()
        } else {
            self.without_thumbs.len()
        };
        if self.selected_index >= max_index {
            self.selected_index = max_index - 1;
        }
    }

    fn increase(&mut self) {
        if self.selected_section == 0 {
            if let Some(group) = self.with_thumbs.get_mut(self.selected_index) {
                group.state.increase(5.0);
            }
        } else if let Some(group) = self.without_thumbs.get_mut(self.selected_index) {
            group.state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if self.selected_section == 0 {
            if let Some(group) = self.with_thumbs.get_mut(self.selected_index) {
                group.state.decrease(5.0);
            }
        } else if let Some(group) = self.without_thumbs.get_mut(self.selected_index) {
            group.state.decrease(5.0);
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
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Thumb/Handle Visibility Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(
        "Tab: Switch section | ↑/↓ or j/k: Select | ←/→ or h/l: Adjust | q/Esc: Quit",
    )
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Split content area into two sections
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Render with thumbs section
    render_section(
        f,
        "WITH THUMB INDICATORS",
        &app.with_thumbs,
        app.selected_section == 0,
        app.selected_index,
        true,
        content_chunks[0],
    );

    // Render without thumbs section
    render_section(
        f,
        "WITHOUT THUMB INDICATORS (Progress Bar Style)",
        &app.without_thumbs,
        app.selected_section == 1,
        app.selected_index,
        false,
        content_chunks[1],
    );
}

fn render_section(
    f: &mut Frame,
    title: &str,
    sliders: &[SliderGroup],
    is_active_section: bool,
    selected_index: usize,
    show_thumbs: bool,
    area: ratatui::layout::Rect,
) {
    // Section container
    let section_block = Block::default()
        .borders(Borders::ALL)
        .border_style(if is_active_section {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .title(format!(" {} ", title));

    let inner_area = section_block.inner(area);
    f.render_widget(section_block, area);

    // Layout for sliders
    let num_sliders = sliders.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(4));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner_area);

    for (i, group) in sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = is_active_section && i == selected_index;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(format!(" {} ", group.label));

        let slider = Slider::from_state(&group.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(symbols::FILLED_THICK_LINE)
            .empty_symbol(symbols::EMPTY_THIN_LINE)
            .handle_symbol(symbols::HANDLE_CIRCLE)
            .filled_color(group.color)
            .empty_color(Color::DarkGray)
            .handle_color(if is_selected {
                Color::White
            } else {
                group.color
            })
            .show_value(true)
            .show_handle(show_thumbs)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}
