use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game name
    game: String,
    /// Game window
    game_window: String,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, game_window: &str) -> Self {
        Self {
            running: true,
            game: game.to_string(),
	    game_window: game_window.to_string(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}