use ratatui::widgets::ListState;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Menu state.
    pub menu_state: ListState,
    /// Menu options.
    pub menu_options: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        Self {
            running: true,
            menu_state,
            menu_options: vec![
                "Execute ".to_string(),
                "Config ".to_string(),
                "Forum 󰊌".to_string(),
                "Help 󰞋".to_string(),
            ],
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Menu select down
    pub fn menu_down(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= self.menu_options.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }
    /// Menu select up
    pub fn menu_up(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.menu_options.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }
}
