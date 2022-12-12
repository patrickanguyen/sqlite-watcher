mod ui;

use crate::db::{SQLTable, SQLiteConnectionPool, SQLiteSchema};
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub struct App {
    pub title: String,
    pub is_running: bool,
    pub pool: SQLiteConnectionPool,
    pub sql_table: SQLTable,
    pub tabs: Vec<String>,
    pub tab_idx: usize,
    pub scroll: usize,
    pub rows: Vec<String>,
}

impl App {
    pub fn new(title: String, pool: SQLiteConnectionPool, tables: Vec<SQLiteSchema>) -> App {
        let sql_table = SQLTable::new(pool.clone());
        App {
            title,
            is_running: true,
            pool,
            sql_table,
            tabs: tables.iter().map(|table| table.tbl_name.clone()).collect(),
            tab_idx: 0,
            scroll: 0,
            rows: Vec::new(),
        }
    }

    pub fn on_quit(&mut self) {
        self.is_running = false;
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'a' => self.title = "ABC".to_string(),
            _ => self.title = "Test TUI".to_string(),
        }
    }

    pub fn on_up(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn on_down(&mut self) {
        if self.scroll < (self.rows.len() - 1) {
            self.scroll += 1;
        }
    }

    pub fn on_left(&mut self) {
        if self.tab_idx > 0 {
            self.tab_idx -= 1;
            self.scroll = 0;
        }
    }

    pub fn on_right(&mut self) {
        if self.tab_idx < (self.tabs.len() - 1) {
            self.tab_idx += 1;
            self.scroll = 0;
        }
    }

    pub fn on_tick(&mut self) {
        let current_tab = &self.tabs[self.tab_idx];
        self.rows = self.sql_table.get_rows(current_tab);
    }
}

pub fn run(
    tick_rate: Duration,
    pool: SQLiteConnectionPool,
    tables: Vec<SQLiteSchema>,
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new("Test TUI".to_string(), pool, tables);
    let result = run_app(&mut terminal, app, tick_rate);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => app.on_quit(),
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Down => app.on_down(),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Right => app.on_right(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.is_running {
            return Ok(());
        }
    }
}
