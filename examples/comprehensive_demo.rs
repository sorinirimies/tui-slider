//! Comprehensive slider demo
//!
//! This example demonstrates both horizontal and vertical sliders
//! with various styles, with and without handles, and different
//! title and value positioning options.

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
use tui_slider::border::{title_center, title_left, title_right_with_spacing};
use tui_slider::position::{VerticalValueAlignment, VerticalValuePosition};
use tui_slider::style::SliderStyle;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct HorizontalSliderConfig {
    label: String,
    state: SliderState,
    style: SliderStyle,
    show_handle: bool,
    value_alignment: Alignment,
    title_position: TitlePosition,
}

struct VerticalSliderConfig {
    label: String,
    state: SliderState,
    style: SliderStyle,
    show_handle: bool,
    value_position: VerticalValuePosition,
    value_alignment: VerticalValueAlignment,
}

#[derive(Clone, Copy)]
enum TitlePosition {
    Left,
    Center,
    Right,
}

struct App {
    horizontal_sliders: Vec<HorizontalSliderConfig>,
    vertical_sliders: Vec<VerticalSliderConfig>,
    selected_section: usize, // 0 = horizontal, 1 = vertical
    selected_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            horizontal_sliders: vec![
                HorizontalSliderConfig {
                    label: "Default (Handle, Value Right)".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    style: SliderStyle::default_style(),
                    show_handle: true,
                    value_alignment: Alignment::Right,
                    title_position: TitlePosition::Left,
                },
                HorizontalSliderConfig {
                    label: "Progress (No Handle, Value Center)".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    style: SliderStyle::progress(),
                    show_handle: false,
                    value_alignment: Alignment::Center,
                    title_position: TitlePosition::Center,
                },
                HorizontalSliderConfig {
                    label: "Blocks (Handle, Value Left)".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    style: SliderStyle::blocks(),
                    show_handle: true,
                    value_alignment: Alignment::Left,
                    title_position: TitlePosition::Right,
                },
                HorizontalSliderConfig {
                    label: "Wave (No Handle, Value Right)".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    style: SliderStyle::wave(),
                    show_handle: false,
                    value_alignment: Alignment::Right,
                    title_position: TitlePosition::Left,
                },
                HorizontalSliderConfig {
                    label: "Gradient (Handle, Value Center)".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    style: SliderStyle::gradient(),
                    show_handle: true,
                    value_alignment: Alignment::Center,
                    title_position: TitlePosition::Center,
                },
                HorizontalSliderConfig {
                    label: "Retro (No Handle, Value Left)".to_string(),
                    state: SliderState::new(70.0, 0.0, 100.0),
                    style: SliderStyle::retro(),
                    show_handle: false,
                    value_alignment: Alignment::Left,
                    title_position: TitlePosition::Right,
                },
                HorizontalSliderConfig {
                    label: "Dots (Handle, Value Right)".to_string(),
                    state: SliderState::new(40.0, 0.0, 100.0),
                    style: SliderStyle::dots(),
                    show_handle: true,
                    value_alignment: Alignment::Right,
                    title_position: TitlePosition::Center,
                },
                HorizontalSliderConfig {
                    label: "Minimal (No Handle, Value Center)".to_string(),
                    state: SliderState::new(85.0, 0.0, 100.0),
                    style: SliderStyle::minimal(),
                    show_handle: false,
                    value_alignment: Alignment::Center,
                    title_position: TitlePosition::Left,
                },
                HorizontalSliderConfig {
                    label: "Segmented Blocks (Handle, Value Left)".to_string(),
                    state: SliderState::new(65.0, 0.0, 100.0),
                    style: SliderStyle::segmented_blocks(),
                    show_handle: true,
                    value_alignment: Alignment::Left,
                    title_position: TitlePosition::Right,
                },
                HorizontalSliderConfig {
                    label: "Segmented Squares (No Handle, Value Right)".to_string(),
                    state: SliderState::new(50.0, 0.0, 100.0),
                    style: SliderStyle::segmented_squares(),
                    show_handle: false,
                    value_alignment: Alignment::Right,
                    title_position: TitlePosition::Center,
                },
            ],
            vertical_sliders: vec![
                // Bottom-aligned values (clearer to see)
                VerticalSliderConfig {
                    label: "Def-BL".to_string(),
                    state: SliderState::new(75.0, 0.0, 100.0),
                    style: SliderStyle::default_style(),
                    show_handle: true,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Left,
                },
                VerticalSliderConfig {
                    label: "Prg-BC".to_string(),
                    state: SliderState::new(60.0, 0.0, 100.0),
                    style: SliderStyle::progress(),
                    show_handle: false,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Center,
                },
                VerticalSliderConfig {
                    label: "Blk-BR".to_string(),
                    state: SliderState::new(45.0, 0.0, 100.0),
                    style: SliderStyle::blocks(),
                    show_handle: true,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Right,
                },
                // Top-aligned values (clearer to see)
                VerticalSliderConfig {
                    label: "Wv-TL".to_string(),
                    state: SliderState::new(80.0, 0.0, 100.0),
                    style: SliderStyle::wave(),
                    show_handle: false,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Left,
                },
                VerticalSliderConfig {
                    label: "Grd-TC".to_string(),
                    state: SliderState::new(55.0, 0.0, 100.0),
                    style: SliderStyle::gradient(),
                    show_handle: true,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Center,
                },
                VerticalSliderConfig {
                    label: "Ret-TR".to_string(),
                    state: SliderState::new(70.0, 0.0, 100.0),
                    style: SliderStyle::retro(),
                    show_handle: false,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Right,
                },
                // Mixed for variety
                VerticalSliderConfig {
                    label: "Dot-BL".to_string(),
                    state: SliderState::new(40.0, 0.0, 100.0),
                    style: SliderStyle::dots(),
                    show_handle: true,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Left,
                },
                VerticalSliderConfig {
                    label: "Min-TC".to_string(),
                    state: SliderState::new(85.0, 0.0, 100.0),
                    style: SliderStyle::minimal(),
                    show_handle: false,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Center,
                },
                VerticalSliderConfig {
                    label: "SgB-BR".to_string(),
                    state: SliderState::new(65.0, 0.0, 100.0),
                    style: SliderStyle::segmented_blocks(),
                    show_handle: true,
                    value_position: VerticalValuePosition::Bottom,
                    value_alignment: VerticalValueAlignment::Right,
                },
                VerticalSliderConfig {
                    label: "SgS-TL".to_string(),
                    state: SliderState::new(50.0, 0.0, 100.0),
                    style: SliderStyle::segmented_squares(),
                    show_handle: false,
                    value_position: VerticalValuePosition::Top,
                    value_alignment: VerticalValueAlignment::Left,
                },
            ],
            selected_section: 0,
            selected_index: 0,
        }
    }

    fn next(&mut self) {
        let max = if self.selected_section == 0 {
            self.horizontal_sliders.len()
        } else {
            self.vertical_sliders.len()
        };
        self.selected_index = (self.selected_index + 1) % max;
    }

    fn previous(&mut self) {
        let max = if self.selected_section == 0 {
            self.horizontal_sliders.len()
        } else {
            self.vertical_sliders.len()
        };
        if self.selected_index > 0 {
            self.selected_index -= 1;
        } else {
            self.selected_index = max - 1;
        }
    }

    fn switch_section(&mut self) {
        self.selected_section = if self.selected_section == 0 { 1 } else { 0 };
        self.selected_index = 0;
    }

    fn increase(&mut self) {
        if self.selected_section == 0 {
            if let Some(config) = self.horizontal_sliders.get_mut(self.selected_index) {
                config.state.increase(5.0);
            }
        } else {
            if let Some(config) = self.vertical_sliders.get_mut(self.selected_index) {
                config.state.increase(5.0);
            }
        }
    }

    fn decrease(&mut self) {
        if self.selected_section == 0 {
            if let Some(config) = self.horizontal_sliders.get_mut(self.selected_index) {
                config.state.decrease(5.0);
            }
        } else {
            if let Some(config) = self.vertical_sliders.get_mut(self.selected_index) {
                config.state.decrease(5.0);
            }
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
    let title =
        Paragraph::new("Comprehensive Slider Demo - All Styles, Orientations & Positioning")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help = Paragraph::new(vec![
        ratatui::text::Line::from("Tab: Switch section (Horizontal ↔ Vertical)"),
        ratatui::text::Line::from("↑/↓ or j/k: Select slider | ←/→ or h/l: Adjust value"),
        ratatui::text::Line::from(
            "Shows various styles, handle visibility, title positions, and value alignments",
        ),
        ratatui::text::Line::from("q/ESC: Quit"),
    ])
    .style(Style::default().fg(Color::Gray))
    .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Split content area into two sections
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Render horizontal section
    render_horizontal_section(
        f,
        &app.horizontal_sliders,
        app.selected_section == 0,
        app.selected_index,
        content_chunks[0],
    );

    // Render vertical section
    render_vertical_section(
        f,
        &app.vertical_sliders,
        app.selected_section == 1,
        app.selected_index,
        content_chunks[1],
    );
}

fn render_horizontal_section(
    f: &mut Frame,
    sliders: &[HorizontalSliderConfig],
    is_active: bool,
    selected_index: usize,
    area: ratatui::layout::Rect,
) {
    let section_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .title(" HORIZONTAL SLIDERS (Multiple Title Positions & Value Alignments) ");

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

    for (i, config) in sliders.iter().enumerate() {
        if i + 1 >= chunks.len() {
            break;
        }

        let is_selected = is_active && i == selected_index;

        // Create block with title positioned according to config
        let title_text = format!(" {} ", config.label);
        let title = match config.title_position {
            TitlePosition::Left => title_left(title_text),
            TitlePosition::Center => title_center(title_text),
            TitlePosition::Right => title_right_with_spacing(title_text),
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(title);

        let slider = Slider::from_state(&config.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(config.style.filled_symbol)
            .empty_symbol(config.style.empty_symbol)
            .handle_symbol(config.style.handle_symbol)
            .filled_color(config.style.filled_color)
            .empty_color(config.style.empty_color)
            .handle_color(if is_selected {
                Color::White
            } else {
                config.style.handle_color
            })
            .show_value(true)
            .value_alignment(config.value_alignment)
            .show_handle(config.show_handle)
            .block(block);

        f.render_widget(slider, chunks[i + 1]);
    }
}

fn render_vertical_section(
    f: &mut Frame,
    sliders: &[VerticalSliderConfig],
    is_active: bool,
    selected_index: usize,
    area: ratatui::layout::Rect,
) {
    let section_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(if is_active {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .title(" VERTICAL SLIDERS (Various Value Positions & Alignments) ");

    let inner_area = section_block.inner(area);
    f.render_widget(section_block, area);

    let num_sliders = sliders.len();
    let slider_width = 14;
    let spacing = 2;

    let mut constraints = vec![Constraint::Length(1)];
    for _ in 0..num_sliders {
        constraints.push(Constraint::Length(slider_width));
        constraints.push(Constraint::Length(spacing));
    }
    constraints.push(Constraint::Min(0));

    let slider_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner_area);

    for (i, config) in sliders.iter().enumerate() {
        let chunk_index = i * 2 + 1;
        if chunk_index >= slider_chunks.len() {
            break;
        }

        let is_selected = is_active && i == selected_index;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if is_selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            })
            .title(ratatui::text::Line::from(vec![
                ratatui::text::Span::styled(
                    config.label.clone(),
                    if is_selected {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Gray)
                    },
                ),
            ]));

        // Render block first and get inner area
        let inner_area = block.inner(slider_chunks[chunk_index]);
        f.render_widget(block, slider_chunks[chunk_index]);

        // Render slider in the inner area without block
        let slider = Slider::from_state(&config.state)
            .orientation(SliderOrientation::Vertical)
            .label(&config.label)
            .filled_symbol(config.style.filled_symbol)
            .empty_symbol(config.style.empty_symbol)
            .handle_symbol(config.style.handle_symbol)
            .filled_color(config.style.filled_color)
            .empty_color(config.style.empty_color)
            .handle_color(if is_selected {
                Color::White
            } else {
                config.style.handle_color
            })
            .show_handle(config.show_handle)
            .show_value(true)
            .vertical_value_position(config.value_position)
            .vertical_value_alignment(config.value_alignment);

        f.render_widget(slider, inner_area);
    }
}
