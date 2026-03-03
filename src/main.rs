mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind}, 
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{Terminal, backend, prelude::CrosstermBackend};
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("Could not enable alternate screen");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal  = Terminal::new(backend)?;
    let mut app = ui::App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match app.current_screen {
                    ui::CurrentScreen::MainMenu => { app.current_screen = ui::CurrentScreen::Game; }             
                    ui::CurrentScreen::Game => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('e') => app.current_screen = ui::CurrentScreen::GameOver,
                            _ => {}
                        }
                    },
                    ui::CurrentScreen::GameOver => {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => app.current_screen = ui::CurrentScreen::MainMenu,
                            _ => {}
                        }
                    },
                }

            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    return Ok(());
}