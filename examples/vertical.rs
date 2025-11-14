//! Vertical slider example with different styles
//!
//! This example demonstrates vertical sliders with various styling configurations.

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
                    "Attack".to_string(),
                    SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_blocks(),
                ),
                (
                    "Decay".to_string(),
                    SliderState::with_step(35.0, 0.0, 100.0, 1.0),
                    SliderStyle::segmented_squares(),
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
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Vertical Slider Styles Demo")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new("←/→ or h/l: Select | ↑/↓ or k/j: Adjust | q/Esc: Quit")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render sliders
    render_sliders(f, app, main_chunks[1]);
}

fn render_sliders(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_sliders = app.sliders.len();
    let slider_width = 12;
    let spacing = 2;
    let total_width = (slider_width + spacing) * num_sliders as u16;

    // Center the sliders if there's extra space
    let remaining = area.width.saturating_sub(total_width);
    let left_margin = remaining / 2;

    let mut constraints = vec![Constraint::Length(left_margin)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }
    constraints.push(Constraint::Min(0));

    let slider_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, (label, state, style)) in app.sliders.iter().enumerate() {
        let chunk_index = 1 + (i * 2);
        if chunk_index >= slider_chunks.len() {
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
            .title(ratatui::text::Line::from(vec![
                ratatui::text::Span::styled(
                    label.clone(),
                    if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                ),
            ]));

        if style.segmented {
            // Render segmented slider
            render_segmented_slider(
                f,
                state,
                style,
                is_selected,
                block,
                slider_chunks[chunk_index],
            );
        } else {
            let inner_area = block.inner(slider_chunks[chunk_index]);
            f.render_widget(block, slider_chunks[chunk_index]);

            // Reserve space for value at bottom
            let slider_area = ratatui::layout::Rect {
                x: inner_area.x,
                y: inner_area.y,
                width: inner_area.width,
                height: inner_area.height.saturating_sub(2),
            };

            let slider = Slider::from_state(state)
                .orientation(SliderOrientation::Vertical)
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
                .show_handle(true);

            f.render_widget(slider, slider_area);

            // Render value below slider in separate area
            let value_area = ratatui::layout::Rect {
                x: inner_area.x,
                y: inner_area.y + slider_area.height,
                width: inner_area.width,
                height: 1,
            };

            let value_text = format!("{:.0}", state.value());
            let value_para = Paragraph::new(value_text)
                .style(if is_selected {
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                })
                .alignment(Alignment::Center);

            f.render_widget(value_para, value_area);
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

    if inner.width < 1 || inner.height < 3 {
        return;
    }

    // Use full available height (minus space for value at bottom)
    let total_height = inner.height.saturating_sub(2) as usize;

    if total_height < 10 {
        // Not enough height, render non-segmented
        let slider = Slider::from_state(state)
            .orientation(SliderOrientation::Vertical)
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
            .show_handle(true);
        f.render_widget(slider, inner);
        return;
    }

    let percentage = state.percentage();

    // Render segments with spaces to fill entire height
    // Each segment takes 2 lines: symbol + space
    let num_segments = total_height / 2;
    let filled_segments = (num_segments as f64 * percentage).round() as usize;
    let handle_position = filled_segments.saturating_sub(1);

    // Build the segmented bar from top to bottom (to match visual order)
    let mut lines = Vec::new();

    for i in (0..num_segments).rev() {
        // Add handle at the correct position
        if i == handle_position && filled_segments > 0 {
            lines.push(Line::from(vec![Span::styled(
                style.handle_symbol,
                Style::default().fg(if is_selected {
                    Color::White
                } else {
                    style.handle_color
                }),
            )]));
        } else {
            let symbol = if i < filled_segments {
                Span::styled(style.filled_symbol, Style::default().fg(style.filled_color))
            } else {
                Span::styled(style.empty_symbol, Style::default().fg(style.empty_color))
            };
            lines.push(Line::from(vec![symbol]));
        }

        // Add space after each segment (except last one)
        if lines.len() < total_height {
            lines.push(Line::from(vec![Span::raw(" ")]));
        }
    }

    // Fill any remaining height
    while lines.len() < total_height {
        lines.push(Line::from(vec![Span::raw(" ")]));
    }

    // Render the slider lines taking full height
    let slider_area = ratatui::layout::Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: total_height as u16,
    };

    let para = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(para, slider_area);

    // Add value at the bottom in a separate area
    let value_area = ratatui::layout::Rect {
        x: inner.x,
        y: inner.y + total_height as u16,
        width: inner.width,
        height: 1,
    };

    let value_para = Paragraph::new(format!("{:.0}", state.value()))
        .style(if is_selected {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        })
        .alignment(Alignment::Center);

    f.render_widget(value_para, value_area);
}
