use std::io::{Stdout, stdout};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use color_eyre::{Result};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init() -> std::io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore() -> Result<()> {
    execute!(stdout(),  LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}