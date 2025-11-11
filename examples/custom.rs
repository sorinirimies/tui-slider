//! Custom slider styles showcase
//!
//! This example demonstrates how to create custom slider styles using the builder pattern.
//! It shows various custom styles with RGB colors and different symbol combinations.

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
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::style::SliderStyle;
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct App {
    sliders: Vec<(String, SliderState, SliderStyle)>,
    selected: usize,
}

impl App {
    fn new() -> Self {
        Self {
            sliders: vec![
                (
                    "Sunset Gradient".to_string(),
                    SliderState::new(75.0, 0.0, 100.0),
                    SliderStyle::custom("Sunset")
                        .filled_symbol(symbols::FILLED_BLOCK)
                        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
                        .handle_symbol(symbols::HANDLE_CIRCLE)
                        .filled_color(Color::Rgb(255, 100, 50))
                        .empty_color(Color::Rgb(80, 40, 30))
                        .handle_color(Color::Rgb(255, 200, 100)),
                ),
                (
                    "Ocean Wave".to_string(),
                    SliderState::new(60.0, 0.0, 100.0),
                    SliderStyle::custom("Ocean")
                        .filled_symbol(symbols::FILLED_WAVE)
                        .empty_symbol(symbols::EMPTY_WAVE)
                        .handle_symbol(symbols::HANDLE_DOUBLE_CIRCLE)
                        .filled_color(Color::Rgb(50, 150, 255))
                        .empty_color(Color::Rgb(30, 60, 100))
                        .handle_color(Color::Rgb(150, 220, 255)),
                ),
                (
                    "Forest Green".to_string(),
                    SliderState::new(45.0, 0.0, 100.0),
                    SliderStyle::custom("Forest")
                        .filled_symbol(symbols::FILLED_DARK_SHADE)
                        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
                        .handle_symbol(symbols::HANDLE_DIAMOND)
                        .filled_color(Color::Rgb(50, 150, 50))
                        .empty_color(Color::Rgb(30, 60, 30))
                        .handle_color(Color::Rgb(100, 255, 100)),
                ),
                (
                    "Purple Haze".to_string(),
                    SliderState::new(80.0, 0.0, 100.0),
                    SliderStyle::custom("Purple")
                        .filled_symbol(symbols::FILLED_MEDIUM_SHADE)
                        .empty_symbol(symbols::EMPTY_LIGHT_SHADE)
                        .handle_symbol(symbols::HANDLE_HEXAGON)
                        .filled_color(Color::Rgb(150, 50, 200))
                        .empty_color(Color::Rgb(60, 30, 80))
                        .handle_color(Color::Rgb(220, 150, 255)),
                ),
                (
                    "Fire".to_string(),
                    SliderState::new(90.0, 0.0, 100.0),
                    SliderStyle::custom("Fire")
                        .filled_symbol(symbols::FILLED_BAR)
                        .empty_symbol(symbols::EMPTY_BAR_OUTLINE)
                        .handle_symbol(symbols::HANDLE_TRIANGLE_RIGHT)
                        .filled_color(Color::Rgb(255, 50, 0))
                        .empty_color(Color::Rgb(100, 30, 0))
                        .handle_color(Color::Rgb(255, 200, 0)),
                ),
                (
                    "Ice Crystal".to_string(),
                    SliderState::new(35.0, 0.0, 100.0),
                    SliderStyle::custom("Ice")
                        .filled_symbol(symbols::FILLED_DIAMOND)
                        .empty_symbol(symbols::EMPTY_DIAMOND)
                        .handle_symbol(symbols::HANDLE_DOUBLE_DIAMOND)
                        .filled_color(Color::Rgb(100, 200, 255))
                        .empty_color(Color::Rgb(40, 80, 120))
                        .handle_color(Color::Rgb(200, 240, 255)),
                ),
                (
                    "Neon Pink".to_string(),
                    SliderState::new(70.0, 0.0, 100.0),
                    SliderStyle::custom("Neon")
                        .filled_symbol(symbols::FILLED_LOWER_BAR)
                        .empty_symbol(symbols::EMPTY_LOWER_BAR)
                        .handle_symbol(symbols::HANDLE_LOWER_BAR)
                        .filled_color(Color::Rgb(255, 0, 150))
                        .empty_color(Color::Rgb(80, 0, 60))
                        .handle_color(Color::Rgb(0, 255, 200)),
                ),
                (
                    "Gold Rush".to_string(),
                    SliderState::new(55.0, 0.0, 100.0),
                    SliderStyle::custom("Gold")
                        .filled_symbol(symbols::FILLED_STAR)
                        .empty_symbol(symbols::EMPTY_STAR)
                        .handle_symbol(symbols::HANDLE_FILLED_STAR)
                        .filled_color(Color::Rgb(255, 200, 0))
                        .empty_color(Color::Rgb(100, 80, 0))
                        .handle_color(Color::Rgb(255, 255, 150)),
                ),
                (
                    "Cyber Segmented".to_string(),
                    SliderState::new(65.0, 0.0, 100.0),
                    SliderStyle::custom("Cyber")
                        .filled_symbol("▰")
                        .empty_symbol("▱")
                        .handle_symbol(symbols::HANDLE_BULLSEYE)
                        .filled_color(Color::Rgb(0, 255, 150))
                        .empty_color(Color::Rgb(20, 80, 60))
                        .handle_color(Color::Rgb(150, 255, 200))
                        .with_segments(true),
                ),
                (
                    "Lava Segmented".to_string(),
                    SliderState::new(85.0, 0.0, 100.0),
                    SliderStyle::custom("Lava")
                        .filled_symbol("█")
                        .empty_symbol("░")
                        .handle_symbol(symbols::HANDLE_SQUARE)
                        .filled_color(Color::Rgb(255, 80, 0))
                        .empty_color(Color::Rgb(100, 30, 0))
                        .handle_color(Color::Rgb(255, 255, 0))
                        .with_segments(true),
                ),
                (
                    "Electric Blue".to_string(),
                    SliderState::new(40.0, 0.0, 100.0),
                    SliderStyle::custom("Electric")
                        .filled_symbol("━")
                        .empty_symbol("╌")
                        .handle_symbol(symbols::HANDLE_LARGE_CIRCLE)
                        .filled_color(Color::Rgb(0, 150, 255))
                        .empty_color(Color::Rgb(0, 50, 100))
                        .handle_color(Color::Rgb(150, 220, 255))
                        .with_segments(true),
                ),
                (
                    "Rainbow Dots".to_string(),
                    SliderState::new(50.0, 0.0, 100.0),
                    SliderStyle::custom("Rainbow")
                        .filled_symbol("●")
                        .empty_symbol("○")
                        .handle_symbol(symbols::HANDLE_DIAMOND)
                        .filled_color(Color::Rgb(200, 100, 255))
                        .empty_color(Color::Rgb(60, 40, 80))
                        .handle_color(Color::Rgb(255, 200, 100))
                        .with_segments(true),
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
            state.increase(5.0);
        }
    }

    fn decrease(&mut self) {
        if let Some((_, state, _)) = self.sliders.get_mut(self.selected) {
            state.decrease(5.0);
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
    let title = Paragraph::new("Custom Slider Styles Showcase")
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
                .show_handle(true)
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
    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.width < 3 || inner.height < 1 {
        return;
    }

    // Calculate segments
    let segment_count = 20;
    let segment_width = 3; // symbol + space + space
    let available_width = inner.width as usize;
    let max_segments = available_width / segment_width;
    let actual_segments = segment_count.min(max_segments);

    let percentage = state.percentage();
    let filled_segments = (actual_segments as f64 * percentage).round() as usize;

    // Build the segmented bar
    let mut segments = Vec::new();
    for i in 0..actual_segments {
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
        segments.push(Span::raw(" "));
    }

    // Add handle at the correct position
    let handle_pos = (actual_segments as f64 * percentage).round() as usize;
    if handle_pos > 0 && handle_pos <= actual_segments {
        let insert_pos = (handle_pos * 2).saturating_sub(1);
        if insert_pos < segments.len() {
            segments.insert(
                insert_pos,
                Span::styled(
                    style.handle_symbol,
                    Style::default().fg(if is_selected {
                        Color::White
                    } else {
                        style.handle_color
                    }),
                ),
            );
        }
    }

    // Add value display
    segments.push(Span::raw("  "));
    segments.push(Span::styled(
        format!("{:.1}", state.value()),
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
