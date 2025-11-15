//! Horizontal slider example with different styles
//!
//! This example demonstrates horizontal sliders with various styling configurations.

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
                    "Volume".to_string(),
                    SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    SliderStyle::default_style(),
                ),
                (
                    "Bass".to_string(),
                    SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    SliderStyle::blocks(),
                ),
                (
                    "Delay".to_string(),
                    SliderState::with_step(30.0, 0.0, 100.0, 1.0),
                    SliderStyle::wave(),
                ),
                (
                    "Chorus".to_string(),
                    SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    SliderStyle::progress(),
                ),
                (
                    "Distortion".to_string(),
                    SliderState::with_step(40.0, 0.0, 100.0, 1.0),
                    SliderStyle::thick(),
                ),
                (
                    "Compression".to_string(),
                    SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    SliderStyle::gradient(),
                ),
                (
                    "Phaser".to_string(),
                    SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    SliderStyle::retro(),
                ),
                (
                    "Mix".to_string(),
                    SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented(),
                ),
                (
                    "Attack".to_string(),
                    SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_blocks(),
                ),
                (
                    "Release".to_string(),
                    SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_dots(),
                ),
                (
                    "Decay".to_string(),
                    SliderState::with_step(35.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_squares(),
                ),
                (
                    "Cutoff".to_string(),
                    SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_stars(),
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
            .title(format!(" {} - {} Style ", label, style.name));

        if style.segmented {
            // Render segmented slider with custom logic
            render_segmented_slider(f, state, style, is_selected, block, chunks[i + 1]);
        } else {
            let slider = Slider::from_state(state)
                .orientation(SliderOrientation::Horizontal)
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
                .show_value(true)
                .block(block);

            f.render_widget(slider, chunks[i + 1]);
        }
    }
}

fn render_segmented_slider(
    f: &mut Frame,
    state: &SliderState,
    style: &SliderStyle,
    is_selected: bool,
    block: Block,
    area: ratatui::layout::Rect,
) {
    use ratatui::text::{Line, Span};

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.width < 3 || inner.height < 1 {
        return;
    }

    // Calculate segments to fill the entire width
    let available_width = inner.width as usize;
    let value_str = format!("{:.1}", state.value());
    let value_width = value_str.len() + 2; // "  " + value
    let bar_width = available_width.saturating_sub(value_width);

    // Each segment is: symbol (1 width) + space (1 width) = 2 total
    // Use half the bar width as segment count to ensure we fill the space
    let segment_count = (bar_width / 2).max(1);

    let percentage = state.percentage();
    let filled_segments = (segment_count as f64 * percentage).round() as usize;

    // Build the segmented bar - fill exactly bar_width
    let mut segments = Vec::new();
    let mut current_width = 0;

    for i in 0..segment_count {
        if current_width >= bar_width {
            break;
        }

        if i < filled_segments {
            segments.push(Span::styled(
                style.filled_symbol,
                Style::default().fg(style.filled_color),
            ));
        } else {
            segments.push(Span::styled(
                style.empty_symbol,
                Style::default().fg(style.empty_color),
            ));
        }
        current_width += 1;

        // Add space between segments if there's room
        if current_width < bar_width {
            segments.push(Span::raw(" "));
            current_width += 1;
        }
    }

    // Fill any remaining width with spaces to ensure consistent length
    while current_width < bar_width {
        segments.push(Span::raw(" "));
        current_width += 1;
    }

    // Add handle at the correct position
    let handle_pos = (segment_count as f64 * percentage).round() as usize;
    if handle_pos > 0 && handle_pos <= segment_count {
        let insert_pos = (handle_pos * 2).saturating_sub(1).min(segments.len());
        if insert_pos < segments.len() {
            segments[insert_pos] = Span::styled(
                style.handle_symbol,
                Style::default().fg(if is_selected {
                    Color::White
                } else {
                    style.handle_color
                }),
            );
        }
    }

    // Add value display
    segments.push(Span::raw("  "));
    segments.push(Span::styled(
        value_str,
        Style::default().fg(if is_selected {
            Color::Cyan
        } else {
            Color::Gray
        }),
    ));

    let line = Line::from(segments);
    let para = Paragraph::new(line).alignment(Alignment::Left);
    f.render_widget(para, inner);
}
