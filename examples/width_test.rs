//! Simple width test to verify all sliders have consistent visual length

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{Block, BorderType, Borders},
    Frame, Terminal,
};
use std::io;
use tui_slider::{Slider, SliderOrientation, SliderState};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    loop {
        terminal.draw(ui)?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(f.area());

    // Test different symbol combinations
    let test_cases = [
        ("Thick Line", "━", "─", "●", 50.0, Color::Cyan),
        ("Wave", "≈", "˜", "◆", 50.0, Color::Green),
        ("Hash/Dot", "#", ".", "@", 50.0, Color::Yellow),
        ("Progress", "▰", "▱", "▶", 50.0, Color::Magenta),
        ("Block/Shade", "█", "░", "▓", 50.0, Color::Red),
        ("Double Line", "═", "─", "◉", 50.0, Color::Blue),
    ];

    for (i, (name, filled, empty, handle, value, color)) in test_cases.iter().enumerate() {
        let state = SliderState::new(*value, 0.0, 100.0);

        let slider = Slider::from_state(&state)
            .orientation(SliderOrientation::Horizontal)
            .filled_symbol(*filled)
            .empty_symbol(*empty)
            .handle_symbol(*handle)
            .filled_color(*color)
            .empty_color(Color::DarkGray)
            .handle_color(Color::White)
            .show_handle(true)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(format!(" {} ", name)),
            );

        f.render_widget(slider, chunks[i]);
    }
}
