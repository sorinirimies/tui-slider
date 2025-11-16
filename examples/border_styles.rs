//! Border styles example - Demonstrating different border types and configurations
//!
//! This example shows various border styles including full borders, borders with gaps/segments,
//! sides-only borders, and different border types (Plain, Rounded, Double, Thick).

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::border::{title_center, BorderStyle};
use tui_slider::{Slider, SliderOrientation, SliderState};

struct BorderExample {
    label: String,
    border_style: BorderStyle,
    state: SliderState,
    description: String,
    color: Color,
}

struct App {
    examples: Vec<BorderExample>,
    selected: usize,
    current_page: usize,
    items_per_page: usize,
}

impl App {
    fn new() -> Self {
        Self {
            examples: vec![
                // Plain borders - Full
                BorderExample {
                    label: BorderStyle::Plain.name().to_string(),
                    border_style: BorderStyle::Plain,
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::Plain.description().to_string(),
                    color: Color::Cyan,
                },
                // Plain borders - Segmented
                BorderExample {
                    label: BorderStyle::PlainSegmented.name().to_string(),
                    border_style: BorderStyle::PlainSegmented,
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::PlainSegmented.description().to_string(),
                    color: Color::Cyan,
                },
                // Plain borders - Sides Only
                BorderExample {
                    label: BorderStyle::PlainSidesOnly.name().to_string(),
                    border_style: BorderStyle::PlainSidesOnly,
                    state: SliderState::with_step(58.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::PlainSidesOnly.description().to_string(),
                    color: Color::Cyan,
                },
                // Rounded borders - Full
                BorderExample {
                    label: BorderStyle::Rounded.name().to_string(),
                    border_style: BorderStyle::Rounded,
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::Rounded.description().to_string(),
                    color: Color::Green,
                },
                // Rounded borders - Segmented
                BorderExample {
                    label: BorderStyle::RoundedSegmented.name().to_string(),
                    border_style: BorderStyle::RoundedSegmented,
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::RoundedSegmented.description().to_string(),
                    color: Color::Green,
                },
                // Rounded borders - Sides Only
                BorderExample {
                    label: BorderStyle::RoundedSidesOnly.name().to_string(),
                    border_style: BorderStyle::RoundedSidesOnly,
                    state: SliderState::with_step(73.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::RoundedSidesOnly.description().to_string(),
                    color: Color::Green,
                },
                // Double borders - Full
                BorderExample {
                    label: BorderStyle::Double.name().to_string(),
                    border_style: BorderStyle::Double,
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::Double.description().to_string(),
                    color: Color::Yellow,
                },
                // Double borders - Segmented
                BorderExample {
                    label: BorderStyle::DoubleSegmented.name().to_string(),
                    border_style: BorderStyle::DoubleSegmented,
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::DoubleSegmented.description().to_string(),
                    color: Color::Yellow,
                },
                // Double borders - Sides Only
                BorderExample {
                    label: BorderStyle::DoubleSidesOnly.name().to_string(),
                    border_style: BorderStyle::DoubleSidesOnly,
                    state: SliderState::with_step(83.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::DoubleSidesOnly.description().to_string(),
                    color: Color::Yellow,
                },
                // Thick borders - Full
                BorderExample {
                    label: BorderStyle::Thick.name().to_string(),
                    border_style: BorderStyle::Thick,
                    state: SliderState::with_step(40.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::Thick.description().to_string(),
                    color: Color::Magenta,
                },
                // Thick borders - Segmented
                BorderExample {
                    label: BorderStyle::ThickSegmented.name().to_string(),
                    border_style: BorderStyle::ThickSegmented,
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::ThickSegmented.description().to_string(),
                    color: Color::Magenta,
                },
                // Thick borders - Sides Only
                BorderExample {
                    label: BorderStyle::ThickSidesOnly.name().to_string(),
                    border_style: BorderStyle::ThickSidesOnly,
                    state: SliderState::with_step(48.0, 0.0, 100.0, 1.0),
                    description: BorderStyle::ThickSidesOnly.description().to_string(),
                    color: Color::Magenta,
                },
            ],
            selected: 0,
            current_page: 0,
            items_per_page: 6,
        }
    }

    fn total_pages(&self) -> usize {
        self.examples.len().div_ceil(self.items_per_page)
    }

    fn current_page_examples(&self) -> &[BorderExample] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(self.examples.len());
        &self.examples[start..end]
    }

    fn next(&mut self) {
        let page_size = self.current_page_examples().len();
        self.selected = (self.selected + 1) % page_size;
    }

    fn previous(&mut self) {
        let page_size = self.current_page_examples().len();
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = page_size - 1;
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
        let global_index = self.current_page * self.items_per_page + self.selected;
        if let Some(example) = self.examples.get_mut(global_index) {
            example.state.step_up();
        }
    }

    fn decrease(&mut self) {
        let global_index = self.current_page * self.items_per_page + self.selected;
        if let Some(example) = self.examples.get_mut(global_index) {
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
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title_text = format!(
        "Border Styles - Full, Segmented & Sides-Only Variants (Page {}/{})",
        app.current_page + 1,
        app.total_pages()
    );
    let title = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(
        "↑/↓ or j/k: Select | ←/→ or h/l: Adjust | PgUp/PgDn or p/n: Page | q/Esc: Quit",
    )
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render border examples
    render_examples(f, app, main_chunks[1]);
}

fn render_examples(f: &mut Frame, app: &App, area: Rect) {
    let page_examples = app.current_page_examples();
    let num_examples = page_examples.len();
    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_examples {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, example) in page_examples.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = i == app.selected;
        let chunk = chunks[i + 1];

        if example.border_style.is_segmented() {
            // Render custom segmented border
            render_segmented_border(f, example, is_selected, chunk);
        } else if example.border_style.is_sides_only() {
            // Render sides-only border (left and right only)
            render_sides_only_border(f, example, is_selected, chunk);
        } else {
            // Use standard ratatui borders
            let border_type = match example.border_style {
                BorderStyle::Plain => BorderType::Plain,
                BorderStyle::Rounded => BorderType::Rounded,
                BorderStyle::Double => BorderType::Double,
                BorderStyle::Thick => BorderType::Thick,
                _ => BorderType::Plain,
            };

            let title = title_center(format!(" {} - {} ", example.label, example.description));

            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(border_type)
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

            f.render_widget(slider, chunk);
        }
    }
}

fn render_segmented_border(f: &mut Frame, example: &BorderExample, is_selected: bool, area: Rect) {
    if area.width < 4 || area.height < 4 {
        return;
    }

    let border_color = if is_selected {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let border_style = Style::default().fg(border_color);

    // Get border character set from the border style
    let border_set = example.border_style.border_set();
    let (top_left, top_right, bottom_left, bottom_right, vertical, horizontal) = (
        border_set.top_left,
        border_set.top_right,
        border_set.bottom_left,
        border_set.bottom_right,
        border_set.vertical,
        border_set.horizontal,
    );

    // Render corners
    f.render_widget(
        Span::styled(top_left.to_string(), border_style),
        Rect {
            x: area.x,
            y: area.y,
            width: 1,
            height: 1,
        },
    );
    f.render_widget(
        Span::styled(top_right.to_string(), border_style),
        Rect {
            x: area.x + area.width - 1,
            y: area.y,
            width: 1,
            height: 1,
        },
    );
    f.render_widget(
        Span::styled(bottom_left.to_string(), border_style),
        Rect {
            x: area.x,
            y: area.y + area.height - 1,
            width: 1,
            height: 1,
        },
    );
    f.render_widget(
        Span::styled(bottom_right.to_string(), border_style),
        Rect {
            x: area.x + area.width - 1,
            y: area.y + area.height - 1,
            width: 1,
            height: 1,
        },
    );

    // Render top border with segments (gaps)
    let title = format!(" {} - {} ", example.label, example.description);
    let title_len = title.len() as u16;
    let available_width = area.width.saturating_sub(2);

    if title_len < available_width {
        let before_title = (available_width - title_len) / 2;
        let after_title = available_width - title_len - before_title;

        // Top border segments
        let top_before = create_segmented_line(before_title as usize, horizontal);
        let top_after = create_segmented_line(after_title as usize, horizontal);

        let top_line = Line::from(vec![
            Span::styled(top_left.to_string(), border_style),
            Span::styled(top_before, border_style),
            Span::styled(
                title.clone(),
                if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                },
            ),
            Span::styled(top_after, border_style),
            Span::styled(top_right.to_string(), border_style),
        ]);

        f.render_widget(
            Paragraph::new(top_line),
            Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 1,
            },
        );
    }

    // Render bottom border with segments
    let bottom_segments = create_segmented_line(available_width as usize, horizontal);
    let bottom_line = Line::from(vec![
        Span::styled(bottom_left.to_string(), border_style),
        Span::styled(bottom_segments, border_style),
        Span::styled(bottom_right.to_string(), border_style),
    ]);

    f.render_widget(
        Paragraph::new(bottom_line),
        Rect {
            x: area.x,
            y: area.y + area.height - 1,
            width: area.width,
            height: 1,
        },
    );

    // Render left and right borders with segments
    for y in 1..(area.height - 1) {
        // Create segmented pattern: 2 chars on, 1 char off
        let show_segment = (y % 3) != 2;
        let char = if show_segment {
            vertical.to_string()
        } else {
            " ".to_string()
        };

        // Left border
        f.render_widget(
            Span::styled(char.clone(), border_style),
            Rect {
                x: area.x,
                y: area.y + y,
                width: 1,
                height: 1,
            },
        );

        // Right border
        f.render_widget(
            Span::styled(char, border_style),
            Rect {
                x: area.x + area.width - 1,
                y: area.y + y,
                width: 1,
                height: 1,
            },
        );
    }

    // Render slider in the inner area
    let inner = Rect {
        x: area.x + 1,
        y: area.y + 1,
        width: area.width.saturating_sub(2),
        height: area.height.saturating_sub(2),
    };

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
        .show_handle(true);

    f.render_widget(slider, inner);
}

fn create_segmented_line(length: usize, char: char) -> String {
    tui_slider::border::create_segmented_line(length, char)
}

fn render_sides_only_border(f: &mut Frame, example: &BorderExample, is_selected: bool, area: Rect) {
    if area.width < 2 || area.height < 2 {
        return;
    }

    let border_color = if is_selected {
        Color::Yellow
    } else {
        Color::DarkGray
    };

    let border_style = Style::default().fg(border_color);

    // Get border character set
    let border_set = example.border_style.border_set();
    let vertical = border_set.vertical;

    // Render title at the top (centered)
    let title = format!(" {} - {} ", example.label, example.description);
    let title_style = if is_selected {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    f.render_widget(
        Paragraph::new(title)
            .style(title_style)
            .alignment(Alignment::Center),
        Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        },
    );

    // Render left and right borders for the slider area
    let slider_area_start = area.y + 1;
    let slider_area_height = area.height.saturating_sub(1);

    for y in 0..slider_area_height {
        // Left border
        f.render_widget(
            Span::styled(vertical.to_string(), border_style),
            Rect {
                x: area.x,
                y: slider_area_start + y,
                width: 1,
                height: 1,
            },
        );

        // Right border
        f.render_widget(
            Span::styled(vertical.to_string(), border_style),
            Rect {
                x: area.x + area.width - 1,
                y: slider_area_start + y,
                width: 1,
                height: 1,
            },
        );
    }

    // Render slider in the inner area (with side padding for borders)
    let inner = Rect {
        x: area.x + 1,
        y: slider_area_start,
        width: area.width.saturating_sub(2),
        height: slider_area_height,
    };

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
        .show_handle(true);

    f.render_widget(slider, inner);
}
