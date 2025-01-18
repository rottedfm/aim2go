use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::event::Event;
use winapi::um::winuser::{VK_ESCAPE, VK_SHIFT};


/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_global_events(event: Event, app: &mut App) -> AppResult<()> {
    match event {
        Event::GlobalKey(key_code) => {
            if key_code == VK_ESCAPE as u32 {
                // Check if Shift is also pressed
                let shift_pressed = unsafe { winapi::um::winuser::GetAsyncKeyState(VK_SHIFT) & 0x8000u16 as i16 != 0 };
                if shift_pressed {
                    // Custom handling for ESC + Shift
                    app.quit(); // Or add any other logic for this combination
                }
            }
        }
        _ => {}
    }
    Ok(())
}