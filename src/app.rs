use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Game name
    pub game: String,
    /// Game window
    pub game_window: String,
    // Logo gradiant
    pub logo_gradiant: usize,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, game_window: &str) -> Self {
        Self {
            running: true,
            game: game.to_string(),
	    game_window: game_window.to_string(),
	    logo_gradiant: 0,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
	 if self.logo_gradiant < 122 {
            self.logo_gradiant += 1;
        } else {
            self.logo_gradiant = 0;
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}