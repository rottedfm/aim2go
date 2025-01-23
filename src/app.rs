use std::error;
use ratatui::widgets::ListState;

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
    /// Logo gradient position
    pub logo_gradient: usize,
    /// Logo
    pub logo: String,
    /// Main menu state,
    pub menu_state: ListState,
    /// Menu items
    pub menu_items: Vec<String>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, game_window: &str, logo: &str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));  // Start with the first item selected
        
        Self {
            running: true,
            game: game.to_string(),
            game_window: game_window.to_string(),
            logo_gradient: logo.lines().map(|line| line.len()).max().unwrap_or(0),
            logo: logo.to_string(),
            menu_state: list_state, 
            menu_items: vec![
              "Execute".to_string(),
              "Account".to_string(),
              "Leaderboard".to_string(), 
              "Settings".to_string(),
            ],
        }
    }

    /// Handles the tick event of the terminal asynchronously.
    pub async fn tick(&mut self) {
        self.increment_gradient();
    }

    /// Increments the gradient and resets after a delay for the shimmer effect
    fn increment_gradient(&mut self) {
        if self.logo_gradient <  self.logo.lines().map(|line| line.len()).max().unwrap_or(0) {
                self.logo_gradient += 1;
        } else {
                self.logo_gradient = 0;
        }
    }

    
    /// Updates the menu state by moving the selection up or down.
    pub fn update_menu_state(&mut self, direction: i8, item_count: usize) {
        let selected = self.menu_state.selected().unwrap_or(0);
        let new_selected = if direction > 0 {
            (selected + 1) % item_count
        } else if selected == 0 {
            item_count - 1
        } else {
            selected - 1
        };
        self.menu_state.select(Some(new_selected));
    }    
    
    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
