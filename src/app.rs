use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Game
    pub game: String,

    /// Are we editing a config?
    pub editing: bool,
}


impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, editing: bool) -> Self {
        Self {
		running: true,
		game: game.to_string(),
		editing,
	}
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
