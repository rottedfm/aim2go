use std::error::Error;
use ratatui::widgets::ListState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// UI mode
    pub mode: Mode,
    /// Game name
    pub game: String,
    /// Game window
    pub game_window: String,
    /// Logo gradient position
    pub logo_gradient: usize,
    /// Logo
    pub logo: String,
    /// Main menu state
    pub menu_state: ListState,
    /// Menu items
    pub menu_items: Vec<String>,
    /// Execute log
    pub log: Vec<String>,
}

#[derive(Debug)]
pub enum Mode {
    Execute,
    Model,
    Config,
    #[cfg(feature = "paid")]
    Community,
    #[cfg(feature = "paid")]
    Leaderboard,
    Settings,
    Quit,
    Menu,
}


impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, game_window: &str, logo: &str) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));  // Start with the first item selected

        // Feature-based menu item selection
        #[cfg(feature = "free")]
        let menu_items = Self::get_free_menu_items();
        #[cfg(feature = "paid")]
        let menu_items = Self::get_paid_menu_items();

        // Ensure gradient calculation doesn't panic if logo is empty
        let max_logo_length = logo.lines().map(|line| line.len()).max().unwrap_or(1);

        Self {
            running: true,
            game: game.to_string(),
            game_window: game_window.to_string(),
            logo_gradient: max_logo_length,
            logo: logo.to_string(),
            menu_state: list_state,
            menu_items,
            mode: Mode::Menu,
        }
    }

    /// Handles the tick event of the terminal asynchronously.
    pub async fn tick(&mut self) {
        match &self.mode {
            Mode::Menu => {        
                self.increment_gradient();
            }
            _ => {}
        }
    }
    /// Increments the gradient and resets after a delay for the shimmer effect
    fn increment_gradient(&mut self) {
        let max_length = self.logo.lines().map(|line| line.len()).max().unwrap_or(1);
        if self.logo_gradient < max_length {
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

    #[cfg(feature = "free")]
    #[allow(dead_code)]
    fn get_free_menu_items() -> Vec<String> {
        vec![
            "Execute".to_string(),
            "Model".to_string(),
            "Config".to_string(),
            "Settings".to_string(),
            "Quit".to_string(),
        ]
    }

    #[cfg(feature = "paid")]
    fn get_paid_menu_items() -> Vec<String> {
        vec![
            "Execute".to_string(),
            "Model".to_string(),
            "Config".to_string(),
            "Community".to_string(),
            "Leaderboard".to_string(),
            "Settings".to_string(),
            "Quit".to_string(),
        ]
    }

    /// Switches the application mode based on the selected menu item.
    pub fn select_mode(&mut self) {
        if let Some(selected) = self.menu_state.selected() {
            let selected_item = &self.menu_items[selected];

            match selected_item.as_str() {
                "Execute" => self.mode = Mode::Execute,
                "Model" => self.mode = Mode::Model,
                "Config" => self.mode = Mode::Config,
                "Settings" => self.mode = Mode::Settings,
                "Quit" => self.mode = Mode::Quit,
                #[cfg(feature = "paid")]
                "Community" => self.mode = Mode::Community,
                #[cfg(feature = "paid")]
                "Leaderboard" => self.mode = Mode::Leaderboard,
                _ => self.mode = Mode::Menu,  // Default to MainMenu if no match
            }
        }
    }
}
