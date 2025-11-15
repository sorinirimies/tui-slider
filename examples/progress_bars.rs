//! Progress bars example - sliders without thumb indicators
//!
//! This example demonstrates how to use sliders as progress bars by hiding
//! the thumb/handle indicator. Perfect for loading screens, downloads, health bars, etc.

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
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

struct ProgressBar {
    label: String,
    state: SliderState,
    filled_symbol: &'static str,
    empty_symbol: &'static str,
    filled_color: Color,
    empty_color: Color,
    description: String,
}

struct App {
    progress_bars: Vec<ProgressBar>,
    auto_progress: bool,
}

impl App {
    fn new() -> Self {
        Self {
            progress_bars: vec![
                ProgressBar {
                    label: "Download".to_string(),
                    state: SliderState::with_step(65.0, 0.0, 100.0, 2.0),
                    filled_symbol: "█",
                    empty_symbol: "░",
                    filled_color: Color::Green,
                    empty_color: Color::DarkGray,
                    description: "File download progress".to_string(),
                },
                ProgressBar {
                    label: "Upload".to_string(),
                    state: SliderState::with_step(45.0, 0.0, 100.0, 1.0),
                    filled_symbol: "▰",
                    empty_symbol: "▱",
                    filled_color: Color::Blue,
                    empty_color: Color::DarkGray,
                    description: "Cloud upload status".to_string(),
                },
                ProgressBar {
                    label: "Health".to_string(),
                    state: SliderState::with_step(85.0, 0.0, 100.0, 5.0),
                    filled_symbol: "▓",
                    empty_symbol: "░",
                    filled_color: Color::Red,
                    empty_color: Color::Rgb(40, 40, 40),
                    description: "Player health bar".to_string(),
                },
                ProgressBar {
                    label: "Mana".to_string(),
                    state: SliderState::with_step(60.0, 0.0, 100.0, 3.0),
                    filled_symbol: "▓",
                    empty_symbol: "░",
                    filled_color: Color::Cyan,
                    empty_color: Color::Rgb(40, 40, 40),
                    description: "Magic power reserves".to_string(),
                },
                ProgressBar {
                    label: "Experience".to_string(),
                    state: SliderState::with_step(78.0, 0.0, 100.0, 1.0),
                    filled_symbol: "━",
                    empty_symbol: "─",
                    filled_color: Color::Yellow,
                    empty_color: Color::DarkGray,
                    description: "XP until next level".to_string(),
                },
                ProgressBar {
                    label: "Loading".to_string(),
                    state: SliderState::with_step(30.0, 0.0, 100.0, 2.5),
                    filled_symbol: "═",
                    empty_symbol: "─",
                    filled_color: Color::Magenta,
                    empty_color: Color::DarkGray,
                    description: "Application startup".to_string(),
                },
                ProgressBar {
                    label: "Installation".to_string(),
                    state: SliderState::with_step(92.0, 0.0, 100.0, 1.5),
                    filled_symbol: "▬",
                    empty_symbol: "▭",
                    filled_color: Color::LightGreen,
                    empty_color: Color::DarkGray,
                    description: "Package installation".to_string(),
                },
                ProgressBar {
                    label: "Battery".to_string(),
                    state: SliderState::with_step(42.0, 0.0, 100.0, 5.0),
                    filled_symbol: "■",
                    empty_symbol: "□",
                    filled_color: Color::LightYellow,
                    empty_color: Color::DarkGray,
                    description: "Device battery level".to_string(),
                },
            ],
            auto_progress: false,
        }
    }

    fn toggle_auto_progress(&mut self) {
        self.auto_progress = !self.auto_progress;
    }

    fn update_progress(&mut self) {
        if self.auto_progress {
            for bar in &mut self.progress_bars {
                // Use the configured step size for each bar
                bar.state.step_up();

                // Reset to 0 when reaching 100
                if bar.state.value() >= 100.0 {
                    bar.state.set_value(0.0);
                }
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

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char(' ') => app.toggle_auto_progress(),
                    _ => {}
                }
            }
        } else {
            // Update progress when no key event
            app.update_progress();
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
    let title = Paragraph::new("Progress Bars - No Thumb Indicators")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Help text
    let help_lines = vec![
        Line::from(vec![
            Span::styled("SPACE", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle auto-progress "),
            Span::styled(
                if app.auto_progress { "[ON]" } else { "[OFF]" },
                Style::default().fg(if app.auto_progress {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("q/ESC", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
    ];

    let help = Paragraph::new(help_lines)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(help, main_chunks[2]);

    // Render progress bars
    render_progress_bars(f, app, main_chunks[1]);
}

fn render_progress_bars(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let num_bars = app.progress_bars.len();
    let mut constraints = vec![];
    for _ in 0..num_bars {
        constraints.push(Constraint::Length(4));
    }
    constraints.push(Constraint::Min(0));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

    for (i, bar) in app.progress_bars.iter().enumerate() {
        if i >= chunks.len() {
            break;
        }

        // Determine color based on percentage
        let percentage = bar.state.percentage();
        let filled_color = if percentage < 0.25 {
            Color::Red
        } else if percentage < 0.50 {
            Color::Yellow
        } else if percentage < 0.75 {
            Color::LightGreen
        } else {
            bar.filled_color
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(format!(" {} - {} ", bar.label, bar.description));

        let slider = Slider::from_state(&bar.state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(bar.filled_symbol)
            .empty_symbol(bar.empty_symbol)
            .filled_color(filled_color)
            .empty_color(bar.empty_color)
            .show_value(true)
            .show_handle(false) // This is the key - no thumb indicator!
            .block(block);

        f.render_widget(slider, chunks[i]);
    }
}
