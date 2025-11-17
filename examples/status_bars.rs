//! Status bars example - Game-style displays without thumb indicators
//! Status bars example - Character and system status monitoring
//!
//! This example demonstrates using sliders as status bars for monitoring
//! various metrics like health, mana, CPU, memory, etc.

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
use tui_slider::{symbols, Slider, SliderOrientation, SliderState};

struct Character {
    name: String,
    health: SliderState,
    mana: SliderState,
    stamina: SliderState,
    experience: SliderState,
}

impl Character {
    fn new(name: &str, health: f64, mana: f64, stamina: f64, exp: f64) -> Self {
        Self {
            name: name.to_string(),
            health: SliderState::new(health, 0.0, 100.0),
            mana: SliderState::new(mana, 0.0, 100.0),
            stamina: SliderState::new(stamina, 0.0, 100.0),
            experience: SliderState::new(exp, 0.0, 100.0),
        }
    }

    fn take_damage(&mut self, amount: f64) {
        self.health.decrease(amount);
    }

    fn use_mana(&mut self, amount: f64) {
        self.mana.decrease(amount);
    }

    fn use_stamina(&mut self, amount: f64) {
        self.stamina.decrease(amount);
    }

    fn regenerate(&mut self) {
        self.health.increase(1.0);
        self.mana.increase(2.0);
        self.stamina.increase(3.0);
    }

    fn gain_experience(&mut self, amount: f64) {
        self.experience.increase(amount);
        if self.experience.value() >= 100.0 {
            self.experience.set_value(0.0);
        }
    }
}

struct System {
    cpu: SliderState,
    memory: SliderState,
    disk: SliderState,
    network: SliderState,
}

impl System {
    fn new() -> Self {
        Self {
            cpu: SliderState::new(45.0, 0.0, 100.0),
            memory: SliderState::new(68.0, 0.0, 100.0),
            disk: SliderState::new(82.0, 0.0, 100.0),
            network: SliderState::new(25.0, 0.0, 100.0),
        }
    }

    fn update(&mut self) {
        // Simulate fluctuating system resources
        self.cpu
            .set_value((self.cpu.value() + rand::random::<f64>() * 10.0 - 5.0).clamp(0.0, 100.0));
        self.memory
            .set_value((self.memory.value() + rand::random::<f64>() * 5.0 - 2.5).clamp(0.0, 100.0));
        self.network.set_value(
            (self.network.value() + rand::random::<f64>() * 15.0 - 7.5).clamp(0.0, 100.0),
        );
    }
}

struct App {
    characters: Vec<Character>,
    system: System,
    selected: usize,
    auto_update: bool,
    tick_count: u32,
}

impl App {
    fn new() -> Self {
        Self {
            characters: vec![
                Character::new("Warrior", 85.0, 30.0, 70.0, 65.0),
                Character::new("Mage", 45.0, 95.0, 40.0, 78.0),
                Character::new("Rogue", 60.0, 50.0, 90.0, 42.0),
            ],
            system: System::new(),
            selected: 0,
            auto_update: false,
            tick_count: 0,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.characters.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.characters.len() - 1;
        }
    }

    fn toggle_auto_update(&mut self) {
        self.auto_update = !self.auto_update;
    }

    fn update(&mut self) {
        if self.auto_update {
            self.tick_count += 1;

            // Update every 10 ticks
            if self.tick_count % 10 == 0 {
                for character in &mut self.characters {
                    character.regenerate();
                }
                self.system.update();
            }

            // Random events every 20 ticks
            if self.tick_count % 20 == 0 {
                if let Some(character) = self.characters.get_mut(self.selected) {
                    character.take_damage(5.0);
                    character.use_mana(10.0);
                    character.use_stamina(15.0);
                    character.gain_experience(2.0);
                }
            }
        }
    }

    fn damage(&mut self) {
        if let Some(character) = self.characters.get_mut(self.selected) {
            character.take_damage(10.0);
        }
    }

    fn use_mana(&mut self) {
        if let Some(character) = self.characters.get_mut(self.selected) {
            character.use_mana(15.0);
        }
    }

    fn use_stamina(&mut self) {
        if let Some(character) = self.characters.get_mut(self.selected) {
            character.use_stamina(20.0);
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
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char(' ') => app.toggle_auto_update(),
                    KeyCode::Char('h') => app.damage(),
                    KeyCode::Char('m') => app.use_mana(),
                    KeyCode::Char('s') => app.use_stamina(),
                    _ => {}
                }
            }
        }

        app.update();
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(8),
            Constraint::Length(5),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Status Bars - Game Interface (No Thumb Indicators)")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, main_chunks[0]);

    // Characters section
    render_characters(f, app, main_chunks[1]);

    // System monitoring section
    render_system_stats(f, app, main_chunks[2]);

    // Help text
    render_help(f, app, main_chunks[3]);
}

fn render_characters(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" Characters ")
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let num_chars = app.characters.len();
    let mut constraints = vec![];
    for _ in 0..num_chars {
        constraints.push(Constraint::Ratio(1, num_chars as u32));
    }

    let char_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner);

    for (i, character) in app.characters.iter().enumerate() {
        if i >= char_chunks.len() {
            break;
        }
        render_character_stats(f, character, app.selected == i, char_chunks[i]);
    }
}

fn render_character_stats(f: &mut Frame, character: &Character, is_selected: bool, area: Rect) {
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
        .title(format!(" {} ", character.name));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    // Health bar
    let health_color = if character.health.percentage() < 0.3 {
        Color::Red
    } else if character.health.percentage() < 0.6 {
        Color::Yellow
    } else {
        Color::Green
    };

    let health = Slider::from_state(&character.health)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_DARK_SHADE)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(health_color)
        .empty_color(Color::Rgb(40, 40, 40))
        .show_value(true)
        .show_handle(false)
        .block(Block::default().borders(Borders::BOTTOM).title("HP"));
    f.render_widget(health, chunks[0]);

    // Mana bar
    let mana = Slider::from_state(&character.mana)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_DARK_SHADE)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(Color::Cyan)
        .empty_color(Color::Rgb(40, 40, 40))
        .show_value(true)
        .show_handle(false)
        .block(Block::default().borders(Borders::BOTTOM).title("MP"));
    f.render_widget(mana, chunks[1]);

    // Stamina bar
    let stamina = Slider::from_state(&character.stamina)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_DARK_SHADE)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(Color::LightGreen)
        .empty_color(Color::Rgb(40, 40, 40))
        .show_value(true)
        .show_handle(false)
        .block(Block::default().borders(Borders::BOTTOM).title("ST"));
    f.render_widget(stamina, chunks[2]);

    // Experience bar
    let exp = Slider::from_state(&character.experience)
        .orientation(SliderOrientation::Horizontal)
        .filled_symbol(symbols::FILLED_THICK_LINE)
        .empty_symbol(symbols::EMPTY_THIN_LINE)
        .filled_color(Color::Yellow)
        .empty_color(Color::DarkGray)
        .show_value(true)
        .show_handle(false)
        .block(Block::default().borders(Borders::BOTTOM).title("XP"));
    f.render_widget(exp, chunks[3]);
}

fn render_system_stats(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" System Monitor ")
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    // CPU
    let cpu_color = if app.system.cpu.percentage() > 0.8 {
        Color::Red
    } else if app.system.cpu.percentage() > 0.6 {
        Color::Yellow
    } else {
        Color::Green
    };

    let cpu = Slider::from_state(&app.system.cpu)
        .filled_symbol(symbols::FILLED_BLOCK)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(cpu_color)
        .empty_color(Color::DarkGray)
        .show_value(false)
        .show_handle(false);
    f.render_widget(
        Paragraph::new(format!(
            "CPU   [{}]",
            " ".repeat((chunks[0].width - 10) as usize)
        )),
        chunks[0],
    );
    f.render_widget(
        cpu,
        Rect::new(chunks[0].x + 7, chunks[0].y, chunks[0].width - 7, 1),
    );

    // Memory
    let mem_color = if app.system.memory.percentage() > 0.9 {
        Color::Red
    } else if app.system.memory.percentage() > 0.7 {
        Color::Yellow
    } else {
        Color::Blue
    };

    let memory = Slider::from_state(&app.system.memory)
        .filled_symbol(symbols::FILLED_BLOCK)
        .empty_symbol(symbols::FILLED_LIGHT_SHADE)
        .filled_color(mem_color)
        .empty_color(Color::DarkGray)
        .show_value(false)
        .show_handle(false);
    f.render_widget(
        Paragraph::new(format!(
            "Memory [{}]",
            " ".repeat((chunks[1].width - 10) as usize)
        )),
        chunks[1],
    );
    f.render_widget(
        memory,
        Rect::new(chunks[1].x + 7, chunks[1].y, chunks[1].width - 7, 1),
    );

    // Disk
    let disk = Slider::from_state(&app.system.disk)
        .filled_symbol(symbols::FILLED_PROGRESS)
        .empty_symbol(symbols::EMPTY_PROGRESS)
        .filled_color(Color::Magenta)
        .empty_color(Color::DarkGray)
        .show_value(false)
        .show_handle(false);
    f.render_widget(
        Paragraph::new(format!(
            "Disk   [{}]",
            " ".repeat((chunks[2].width - 10) as usize)
        )),
        chunks[2],
    );
    f.render_widget(
        disk,
        Rect::new(chunks[2].x + 7, chunks[2].y, chunks[2].width - 7, 1),
    );

    // Network
    let network = Slider::from_state(&app.system.network)
        .filled_symbol(symbols::FILLED_PROGRESS)
        .empty_symbol(symbols::EMPTY_PROGRESS)
        .filled_color(Color::Cyan)
        .empty_color(Color::DarkGray)
        .show_value(false)
        .show_handle(false);
    f.render_widget(
        Paragraph::new(format!(
            "Network[{}]",
            " ".repeat((chunks[3].width - 10) as usize)
        )),
        chunks[3],
    );
    f.render_widget(
        network,
        Rect::new(chunks[3].x + 7, chunks[3].y, chunks[3].width - 7, 1),
    );
}

fn render_help(f: &mut Frame, app: &App, area: Rect) {
    let help_lines = vec![
        Line::from(vec![
            Span::styled("↑/↓ or j/k", Style::default().fg(Color::Yellow)),
            Span::raw(": Select character  "),
            Span::styled("h", Style::default().fg(Color::Yellow)),
            Span::raw(": Damage  "),
            Span::styled("m", Style::default().fg(Color::Yellow)),
            Span::raw(": Use mana  "),
            Span::styled("s", Style::default().fg(Color::Yellow)),
            Span::raw(": Use stamina"),
        ]),
        Line::from(vec![
            Span::styled("SPACE", Style::default().fg(Color::Yellow)),
            Span::raw(": Auto-update "),
            Span::styled(
                if app.auto_update { "[ON]" } else { "[OFF]" },
                Style::default().fg(if app.auto_update {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
            Span::raw("  "),
            Span::styled("q/ESC", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
    ];

    let help = Paragraph::new(help_lines)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Controls "),
        );
    f.render_widget(help, area);
}
