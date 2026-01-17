use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{stdout, Stdout};

pub type CrosstermTerminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

/// Terminal wrapper for setup and cleanup
pub struct Terminal {
    terminal: CrosstermTerminal,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let terminal = Self::setup()?;
        Ok(Self { terminal })
    }

    fn setup() -> Result<CrosstermTerminal> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = ratatui::Terminal::new(backend)?;
        Ok(terminal)
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        if let Err(e) = self.restore() {
            eprintln!("Failed to restore terminal: {}", e);
        }
    }
}
