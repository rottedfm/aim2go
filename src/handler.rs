use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }

        KeyCode::Up | KeyCode::Char('k') => {
            app.menu_up();
        }

        KeyCode::Down | KeyCode::Char('j') => {
            app.menu_down();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
