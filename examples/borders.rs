//! Borders example - Demonstrating border types, styles, and colors
//!
//! This example shows various border configurations including different types
//! (Plain, Rounded, Double, Thick), styles (Full, Segmented, Sides Only),
//! and color theming options.

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
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::border::BorderStyle;
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct BorderExample {
    label: String,
    border_style: BorderStyle,
    border_type: BorderType,
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
                // Colored borders with standard types
                BorderExample {
                    label: "Cyan Theme".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    description: "Cool cyan theme".to_string(),
                    color: Color::Cyan,
                },
                BorderExample {
                    label: "Green Theme".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(60.0, 0.0, 100.0, 1.0),
                    description: "Success green".to_string(),
                    color: Color::Green,
                },
                BorderExample {
                    label: "Yellow Theme".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    description: "Warning yellow".to_string(),
                    color: Color::Yellow,
                },
                BorderExample {
                    label: "Red Theme".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Thick,
                    state: SliderState::with_step(30.0, 0.0, 100.0, 1.0),
                    description: "Critical red".to_string(),
                    color: Color::Red,
                },
                BorderExample {
                    label: "Magenta Theme".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(85.0, 0.0, 100.0, 1.0),
                    description: "Creative magenta".to_string(),
                    color: Color::Magenta,
                },
                BorderExample {
                    label: "Custom RGB".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    description: "Custom pink".to_string(),
                    color: Color::Rgb(255, 100, 150),
                },
                // Plain border styles
                BorderExample {
                    label: "Plain".to_string(),
                    border_style: BorderStyle::Plain,
                    border_type: BorderType::Plain,
                    state: SliderState::with_step(50.0, 0.0, 100.0, 1.0),
                    description: "Plain full border".to_string(),
                    color: Color::Cyan,
                },
                BorderExample {
                    label: "Plain Segmented".to_string(),
                    border_style: BorderStyle::PlainSegmented,
                    border_type: BorderType::Plain,
                    state: SliderState::with_step(55.0, 0.0, 100.0, 1.0),
                    description: "Plain with gaps".to_string(),
                    color: Color::Cyan,
                },
                BorderExample {
                    label: "Plain Sides".to_string(),
                    border_style: BorderStyle::PlainSidesOnly,
                    border_type: BorderType::Plain,
                    state: SliderState::with_step(58.0, 0.0, 100.0, 1.0),
                    description: "Left/right only".to_string(),
                    color: Color::Cyan,
                },
                // Rounded border styles
                BorderExample {
                    label: "Rounded".to_string(),
                    border_style: BorderStyle::Rounded,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(65.0, 0.0, 100.0, 1.0),
                    description: "Rounded full".to_string(),
                    color: Color::Green,
                },
                BorderExample {
                    label: "Rounded Segmented".to_string(),
                    border_style: BorderStyle::RoundedSegmented,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(70.0, 0.0, 100.0, 1.0),
                    description: "Rounded with gaps".to_string(),
                    color: Color::Green,
                },
                BorderExample {
                    label: "Rounded Sides".to_string(),
                    border_style: BorderStyle::RoundedSidesOnly,
                    border_type: BorderType::Rounded,
                    state: SliderState::with_step(73.0, 0.0, 100.0, 1.0),
                    description: "Left/right only".to_string(),
                    color: Color::Green,
                },
                // Double border styles
                BorderExample {
                    label: "Double".to_string(),
                    border_style: BorderStyle::Double,
                    border_type: BorderType::Double,
                    state: SliderState::with_step(75.0, 0.0, 100.0, 1.0),
                    description: "Double line full".to_string(),
                    color: Color::Yellow,
                },
                BorderExample {
                    label: "Double Segmented".to_string(),
                    border_style: BorderStyle::DoubleSegmented,
                    border_type: BorderType::Double,
                    state: SliderState::with_step(80.0, 0.0, 100.0, 1.0),
                    description: "Double with gaps".to_string(),
                    color: Color::Yellow,
                },
                BorderExample {
                    label: "Double Sides".to_string(),
                    border_style: BorderStyle::DoubleSidesOnly,
                    border_type: BorderType::Double,
                    state: SliderState::with_step(83.0, 0.0, 100.0, 1.0),
                    description: "Left/right only".to_string(),
                    color: Color::Yellow,
                },
                // Thick border styles
                BorderExample {
                    label: "Thick".to_string(),
                    border_style: BorderStyle::Thick,
                    border_type: BorderType::Thick,
                    state: SliderState::with_step(40.0, 0.0, 100.0, 1.0),
                    description: "Thick full border".to_string(),
                    color: Color::Magenta,
                },
                BorderExample {
                    label: "Thick Segmented".to_string(),
                    border_style: BorderStyle::ThickSegmented,
                    border_type: BorderType::Thick,
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    description: "Thick with gaps".to_string(),
                    color: Color::Magenta,
                },
                BorderExample {
                    label: "Thick Sides".to_string(),
                    border_style: BorderStyle::ThickSidesOnly,
                    border_type: BorderType::Thick,
                    state: SliderState::with_step(48.0, 0.0, 100.0, 1.0),
                    description: "Left/right only".to_string(),
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
    let title = Paragraph::new("Border Styles & Colors - Types, Styles, and Themes")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(vec![
        Line::from("↑/↓ or j/k: Select | ←/→ or h/l: Adjust value"),
        Line::from("n/PageDown: Next page | p/PageUp: Previous page | q/ESC: Quit"),
        Line::from(format!(
            "Page {}/{} - {} total border styles",
            app.current_page + 1,
            app.total_pages(),
            app.examples.len()
        )),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render examples
    render_examples(f, app, main_chunks[1]);
}

fn render_examples(f: &mut Frame, app: &App, area: Rect) {
    let examples = app.current_page_examples();
    let num_examples = examples.len();

    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_examples {
        constraints.push(Constraint::Length(4));
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

        // Render based on border style
        match example.border_style {
            BorderStyle::Plain
            | BorderStyle::Rounded
            | BorderStyle::Double
            | BorderStyle::Thick => {
                // Standard full borders
                let border_style = Style::default()
                    .fg(if is_selected {
                        Color::White
                    } else {
                        example.color
                    })
                    .add_modifier(if is_selected {
                        Modifier::BOLD
                    } else {
                        Modifier::empty()
                    });

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(example.border_type)
                    .border_style(border_style)
                    .title(format!(" {} - {} ", example.label, example.description));

                let slider = Slider::from_state(&example.state)
                    .orientation(SliderOrientation::Horizontal)
                    .filled_symbol(symbols::FILLED_THICK_LINE)
                    .empty_symbol(symbols::EMPTY_THIN_LINE)
                    .handle_symbol(symbols::HANDLE_CIRCLE)
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
            BorderStyle::PlainSegmented
            | BorderStyle::RoundedSegmented
            | BorderStyle::DoubleSegmented
            | BorderStyle::ThickSegmented => {
                // Segmented borders (with gaps)
                render_segmented_border(f, example, is_selected, chunks[i + 1]);
            }
            BorderStyle::PlainSidesOnly
            | BorderStyle::RoundedSidesOnly
            | BorderStyle::DoubleSidesOnly
            | BorderStyle::ThickSidesOnly => {
                // Sides-only borders
                render_sides_only_border(f, example, is_selected, chunks[i + 1]);
            }
        }
    }
}

fn render_segmented_border(f: &mut Frame, example: &BorderExample, is_selected: bool, area: Rect) {
    if area.width < 4 || area.height < 3 {
        return;
    }

    let color = if is_selected {
        Color::White
    } else {
        example.color
    };
    let style = Style::default().fg(color);

    // Determine border characters based on type
    let (tl, tr, bl, br, h, v) = match example.border_type {
        BorderType::Plain => ('┌', '┐', '└', '┘', '─', '│'),
        BorderType::Rounded => ('╭', '╮', '╰', '╯', '─', '│'),
        BorderType::Double => ('╔', '╗', '╚', '╝', '═', '║'),
        BorderType::Thick => ('┏', '┓', '┗', '┛', '━', '┃'),
        _ => ('┌', '┐', '└', '┘', '─', '│'),
    };

    // Draw corners
    f.render_widget(
        Paragraph::new(tl.to_string()).style(style),
        Rect::new(area.x, area.y, 1, 1),
    );
    f.render_widget(
        Paragraph::new(tr.to_string()).style(style),
        Rect::new(area.x + area.width - 1, area.y, 1, 1),
    );
    f.render_widget(
        Paragraph::new(bl.to_string()).style(style),
        Rect::new(area.x, area.y + area.height - 1, 1, 1),
    );
    f.render_widget(
        Paragraph::new(br.to_string()).style(style),
        Rect::new(area.x + area.width - 1, area.y + area.height - 1, 1, 1),
    );

    // Draw segmented top and bottom borders
    let segment_len = 3;
    let gap_len = 2;
    let _inner_width = area.width.saturating_sub(2) as usize;

    for i in (1..area.width - 1).step_by(segment_len + gap_len) {
        let seg_width = segment_len.min((area.width - 1 - i) as usize);
        let segment = h.to_string().repeat(seg_width);
        // Top
        f.render_widget(
            Paragraph::new(segment.clone()).style(style),
            Rect::new(area.x + i, area.y, seg_width as u16, 1),
        );
        // Bottom
        f.render_widget(
            Paragraph::new(segment).style(style),
            Rect::new(area.x + i, area.y + area.height - 1, seg_width as u16, 1),
        );
    }

    // Draw full side borders
    for y in 1..area.height - 1 {
        f.render_widget(
            Paragraph::new(v.to_string()).style(style),
            Rect::new(area.x, area.y + y, 1, 1),
        );
        f.render_widget(
            Paragraph::new(v.to_string()).style(style),
            Rect::new(area.x + area.width - 1, area.y + y, 1, 1),
        );
    }

    // Title
    let title = format!(" {} - {} ", example.label, example.description);
    let title_len = title.len() as u16;
    let title_x = (area.width.saturating_sub(title_len)) / 2;
    f.render_widget(
        Paragraph::new(title).style(style.add_modifier(Modifier::BOLD)),
        Rect::new(area.x + title_x, area.y, title_len, 1),
    );

    // Draw slider inside
    let inner = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );

    let slider = Slider::from_state(&example.state)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_THICK_LINE)
        .empty_symbol(symbols::EMPTY_THIN_LINE)
        .handle_symbol(symbols::HANDLE_CIRCLE)
        .filled_color(example.color)
        .empty_color(Color::DarkGray)
        .handle_color(color)
        .show_value(true)
        .show_handle(true);

    f.render_widget(slider, inner);
}

fn render_sides_only_border(f: &mut Frame, example: &BorderExample, is_selected: bool, area: Rect) {
    if area.width < 4 || area.height < 3 {
        return;
    }

    let color = if is_selected {
        Color::White
    } else {
        example.color
    };
    let style = Style::default().fg(color);

    // Determine border characters
    let v = match example.border_type {
        BorderType::Plain => '│',
        BorderType::Rounded => '│',
        BorderType::Double => '║',
        BorderType::Thick => '┃',
        _ => '│',
    };

    // Draw left and right borders only
    for y in 0..area.height {
        f.render_widget(
            Paragraph::new(v.to_string()).style(style),
            Rect::new(area.x, area.y + y, 1, 1),
        );
        f.render_widget(
            Paragraph::new(v.to_string()).style(style),
            Rect::new(area.x + area.width - 1, area.y + y, 1, 1),
        );
    }

    // Title at top
    let title = format!(" {} - {} ", example.label, example.description);
    let title_len = title.len() as u16;
    let title_x = (area.width.saturating_sub(title_len)) / 2;
    f.render_widget(
        Paragraph::new(title).style(style.add_modifier(Modifier::BOLD)),
        Rect::new(area.x + title_x, area.y, title_len, 1),
    );

    // Draw slider inside
    let inner = Rect::new(
        area.x + 1,
        area.y + 1,
        area.width.saturating_sub(2),
        area.height.saturating_sub(2),
    );

    let slider = Slider::from_state(&example.state)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_THICK_LINE)
        .empty_symbol(symbols::EMPTY_THIN_LINE)
        .handle_symbol(symbols::HANDLE_CIRCLE)
        .filled_color(example.color)
        .empty_color(Color::DarkGray)
        .handle_color(color)
        .show_value(true)
        .show_handle(true);

    f.render_widget(slider, inner);
}
