use crate::app::{App, AppResult, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }

        }
        KeyCode::Char('j') | KeyCode::Char('J') | KeyCode::Down => {
            match &app.mode {
                Mode::Menu => app.update_menu_state(1, app.menu_items.len()),
                _ => {}
            }
        }
        KeyCode::Char('k') | KeyCode::Char('K') | KeyCode::Up => {
            match &app.mode {
                Mode::Menu => app.update_menu_state(-1, app.menu_items.len()),
                _ => {}
            }
        }

        KeyCode::Enter => {
            match &app.mode {
                Mode::Menu => app.select_mode(),
                _ => {}
            }
        }

        KeyCode::Esc => {
            app.mode = Mode::Menu;
        }

        
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
