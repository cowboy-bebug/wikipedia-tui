mod app;
mod request;
mod ui;
mod util;

use crate::app::{App, Mode};
use crate::util::{Event, Events, Key};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::Write;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup event handlers
    let events = Events::new();

    // Initialise app
    let mut app = App::new("wikipedia-tui");

    loop {
        terminal.draw(|mut f| ui::draw(&mut f, &mut app))?;

        // Handle mode
        match app.mode {
            Mode::Search => {
                terminal.show_cursor()?;
                terminal.set_cursor(app.search_cursor_x, app.search_cursor_y)?;
            }
            Mode::Browse => terminal.hide_cursor()?,
            Mode::Read => terminal.hide_cursor()?,
        }

        // Handle key event received
        if let Event::Input(key) = events.next()? {
            match key {
                Key::Char('q') | Key::Char('Q') => break,
                Key::Esc => app.on_escape(),
                Key::Enter => app.on_enter(),
                Key::Up => app.on_up(),
                Key::Down => app.on_down(),
                Key::Left => app.on_left(),
                Key::Right => app.on_right(),
                Key::Char(key) => app.on_key(key),
                Key::Backspace => app.on_backspace(),
                Key::Alt(_) => {}
                _ => {}
            }
        }
    }

    // Close terminal
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
