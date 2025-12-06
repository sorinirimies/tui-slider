//! Horizontal slider styles example
//!
//! This example demonstrates various horizontal slider styles organized into two pages:
//! - Page 1: Standard Styles (basic lines, blocks, gradients, progress bars)
//! - Page 2: Specialty Styles (segmented, dots, squares, stars)

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

struct SliderExample {
    label: String,
    state: SliderState,
    style: SliderStyle,
    show_handle: bool,
    description: String,
}

struct App {
    examples: Vec<SliderExample>,
    selected: usize,
    current_page: usize,
    items_per_page: usize,
}

impl App {
    fn new() -> Self {
        Self {
            examples: vec![
                // Page 1: Standard Styles
                SliderExample {
                    label: "Default".to_string(),
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::default_style(),
                    show_handle: true,
                    description: "Clean default style".to_string(),
                },
                SliderExample {
                    label: "Horizontal".to_string(),
                    state: SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal(),
                    show_handle: true,
                    description: "Standard horizontal".to_string(),
                },
                SliderExample {
                    label: "Thick Line".to_string(),
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_thick(),
                    show_handle: true,
                    description: "Bold thick line".to_string(),
                },
                SliderExample {
                    label: "Blocks".to_string(),
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::blocks(),
                    show_handle: true,
                    description: "Solid block style".to_string(),
                },
                SliderExample {
                    label: "Block Style".to_string(),
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_blocks(),
                    show_handle: true,
                    description: "Block variation".to_string(),
                },
                SliderExample {
                    label: "Gradient".to_string(),
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::gradient(),
                    show_handle: true,
                    description: "Shaded gradient".to_string(),
                },
                SliderExample {
                    label: "Gradient Style".to_string(),
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_gradient(),
                    show_handle: true,
                    description: "Gradient variation".to_string(),
                },
                SliderExample {
                    label: "Progress".to_string(),
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::progress(),
                    show_handle: true,
                    description: "Progress bar style".to_string(),
                },
                SliderExample {
                    label: "Double Line".to_string(),
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_double(),
                    show_handle: true,
                    description: "Double line border".to_string(),
                },
                // Page 2: Specialty Styles
                SliderExample {
                    label: "Segmented".to_string(),
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::segmented(),
                    show_handle: true,
                    description: "Segmented sections".to_string(),
                },
                SliderExample {
                    label: "Segmented Blocks".to_string(),
                    state: SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::segmented_blocks(),
                    show_handle: true,
                    description: "Block segments".to_string(),
                },
                SliderExample {
                    label: "Dots".to_string(),
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_dots(),
                    show_handle: false,
                    description: "Dotted style".to_string(),
                },
                SliderExample {
                    label: "Segmented Dots".to_string(),
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::segmented_dots(),
                    show_handle: false,
                    description: "Dotted segments".to_string(),
                },
                SliderExample {
                    label: "Squares".to_string(),
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::horizontal_squares(),
                    show_handle: false,
                    description: "Square blocks".to_string(),
                },
                SliderExample {
                    label: "Segmented Squares".to_string(),
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::segmented_squares(),
                    show_handle: false,
                    description: "Square segments".to_string(),
                },
                SliderExample {
                    label: "Stars".to_string(),
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    style: SliderStyle::segmented_stars(),
                    show_handle: true,
                    description: "Star segments".to_string(),
                },
            ],
            selected: 0,
            current_page: 0,
            items_per_page: 9,
        }
    }

    fn total_pages(&self) -> usize {
        self.examples.len().div_ceil(self.items_per_page)
    }

    fn current_page_examples(&self) -> &[SliderExample] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.examples.len());
        &self.examples[start..end]
    }

    fn page_title(&self) -> &str {
        match self.current_page {
            0 => "Standard Styles",
            1 => "Specialty Styles",
            _ => "Horizontal Sliders",
        }
    }

    fn next(&mut self) {
        let page_examples = self.current_page_examples().len();
        if self.selected < page_examples - 1 {
            self.selected += 1;
        }
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn next_page(&mut self) {
        if self.current_page < self.total_pages() - 1 {
            self.current_page += 1;
            self.selected = 0;
        }
    }

    fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
            self.selected = 0;
        }
    }

    fn increase(&mut self) {
        let absolute_idx = self.current_page * self.items_per_page + self.selected;
        if let Some(example) = self.examples.get_mut(absolute_idx) {
            example.state.step_up();
        }
    }

    fn decrease(&mut self) {
        let absolute_idx = self.current_page * self.items_per_page + self.selected;
        if let Some(example) = self.examples.get_mut(absolute_idx) {
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
                    KeyCode::PageDown | KeyCode::Char('n') => app.next_page(),
                    KeyCode::PageUp | KeyCode::Char('p') => app.prev_page(),
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
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new(format!("Horizontal Slider Styles - {}", app.page_title()))
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
        ratatui::text::Line::from("n/PageDown: Next page | p/PageUp: Previous page | q/ESC: Quit"),
        ratatui::text::Line::from(format!(
            "Page {}/{} - {} styles on this page",
            app.current_page + 1,
            app.total_pages(),
            app.current_page_examples().len()
        )),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let examples = app.current_page_examples();
    let num_examples = examples.len();

    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_examples {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, example) in examples.iter().enumerate() {
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
            .title(format!(
                " {} - {} - {} ",
                example.label, example.style.name, example.description
            ));

        if example.style.segmented {
            // Render segmented slider with custom logic
            render_segmented_slider(
                f,
                &example.state,
                &example.style,
                is_selected,
                example.show_handle,
                block,
                chunks[i + 1],
            );
        } else {
            let slider = Slider::from_state(&example.state)
                .orientation(SliderOrientation::Horizontal)
                .filled_symbol(example.style.filled_symbol)
                .empty_symbol(example.style.empty_symbol)
                .handle_symbol(example.style.handle_symbol)
                .filled_color(example.style.filled_color)
                .empty_color(example.style.empty_color)
                .handle_color(if is_selected {
                    Color::White
                } else {
                    example.style.handle_color
                })
                .show_value(true)
                .show_handle(example.show_handle)
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
    show_handle: bool,
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

    // Add handle at the correct position (only if show_handle is true)
    if show_handle {
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
