use std::error::Error;
use ratatui::widgets::ListState;
use std::time::{Duration, Instant};
use winapi::shared::windef::HWND;

use crate::thread::{ThreadHandler, ThreadType};

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
    pub game_window: HWND,
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
    /// Thread handler
    pub thread_handler: ThreadHandler,
    /// Is game overlay active?
    pub is_overlay_active: bool,
    /// Was the game overlay previously active?
    pub overlay_was_active: bool,
    /// Last overlay check
    pub last_overlay_check: Instant,
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
    pub fn new(game: &str, game_window: HWND, logo: &str) -> Self {
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
            game_window: game_window,
            logo_gradient: max_logo_length,
            logo: logo.to_string(),
            menu_state: list_state,
            menu_items,             mode: Mode::Menu,
            log: Vec::new(),
            thread_handler: ThreadHandler::new(),
            is_overlay_active: true,
            overlay_was_active: false,
            last_overlay_check: Instant::now(),
        }
    }

    /// Handles the tick event of the terminal asynchronously.
    pub async fn tick(&mut self) {
        match &self.mode {
            Mode::Menu => {        
                self.increment_gradient();
            }
            Mode::Execute => {
                let now = Instant::now();
                if now.duration_since(self.last_overlay_check) >= Duration::from_millis(500) {
                    self.last_overlay_check = now;
                    self.check_overlay_status().await;
                }
            }
            Mode::Quit => {
                self.quit();
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

    /// Logs a message to the execution log
    pub fn log_message(&mut self, message: &str) {
        self.log.push(message.to_string());
    }

    pub async fn check_overlay_status(&mut self) {
        if self.is_overlay_active {
            if !self.overlay_was_active {
                self.thread_handler.start_thread(self.game_window, ThreadType::GameOverlay).await;
                self.overlay_was_active = true;
                self.log_message("[PROCESS] Overlay started!");
            }
        } else {
            if self.overlay_was_active {
                self.thread_handler.stop_thread(&ThreadType::GameOverlay).await;
                self.overlay_was_active = false;
                self.log_message("[PROCESS] Game overlay stopped.");
            }
        }
    }
}
