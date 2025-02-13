use tokio::fs;
use tokio::io;
use std::path::Path;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{GetWindowTextW, IsWindowVisible, GetWindowTextLengthW, EnumWindows};
use cliclack::{select, intro, outro, log::info, clear_screen, set_theme, Theme, ThemeState};
use console::Style;

pub mod config;

use crate::config::{create_default_game_config, load_game_config};

#[derive(Debug)]
struct DynamicTheme {
    pub color: Style,
}

impl DynamicTheme {
    pub fn from_config(game:&str) -> Self {
        let config = load_game_config(game).expect("Failed to load config");

        let theme_color = match config.app.theme.as_str() {
            "red" => Style::new().red(),
            "green" => Style::new().green(),
            "blue" => Style::new().blue(),
            "yellow" => Style::new().yellow(),
            _ => Style::new().magenta(),
        };

        Self { color: theme_color }
        
    }
}
impl Clone for DynamicTheme {
    fn clone(&self) -> Self {
         Self {
             color: self.color.clone(),
         }
    }
}
impl Theme for DynamicTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => self.color.clone().bright(),
            ThemeState::Error(_) => Style::new().red(),
            _ => self.color.clone().dim(),
        }
    }

    fn state_symbol_color(&self, _state: &ThemeState) -> Style {
        self.color.clone()
    }

    fn info_symbol(&self) -> String {
        format!("{}", self.color.apply_to("?"))
    }

    fn format_select_item(
        &self,
        state: &ThemeState,
        selected: bool,
        label: &str,
        hint: &str,
    ) -> String {
        // Trim whitespace from the label
        let trimmed_label = label.trim();

        // Handle submit and cancel states to optionally hide non-selected items
        if matches!(state, ThemeState::Submit | ThemeState::Cancel) && !selected {
            return String::new();
        }

        let label_style = if selected {
            self.color.clone()
        } else {
            Style::new().white()
        };

        let hint_style = Style::new().white();

        let pointer = if selected {
            format!("{}", self.color.apply_to(">>"))
        } else {
            "  ".to_string()
        }; // Use an arrow for the selected item

        let spacing = "  "; // Align items for a clean vertical list

        // Show hint only if the item is selected
        let hint_display = if selected { format!(" {}", hint_style.apply_to(hint)) } else { "".to_string() };

        format!(
            "{}{} {}{}\n",
            pointer,
            spacing,
            label_style.apply_to(trimmed_label),
            hint_display
        )
    }
}

/// Creates the specified directory structure.
pub async fn create_directory(dir_name: &str) -> io::Result<()> {
    let base_path = Path::new(dir_name);

    // Create the main directory
    fs::create_dir_all(base_path).await?;
    println!("Directory '{}' created successfully!", dir_name);

    // Create 'dataset' and 'model' subdirectories
    for sub_dir in ["dataset", "model"] {
        let sub_dir_path = base_path.join(sub_dir);
        fs::create_dir_all(&sub_dir_path).await?;
        println!("Subdirectory '{}' created successfully!", sub_dir_path.display());
    }

    let _ = create_default_game_config(dir_name);
    println!("File 'config.yaml' created successfully!");
    
    Ok(())
}

/// Prompts the user to select a visible window and returns the selected window's `HWND`.
pub fn select_window(game: &str) -> Option<HWND> {
    let mut windows: Vec<(HWND, String)> = Vec::new();

    // Load the theme dynamically
    let theme = DynamicTheme::from_config(game);

    // Callback to collect window handles and titles
    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: isize) -> i32 {
        let windows = &mut *(lparam as *mut Vec<(HWND, String)>);

        if IsWindowVisible(hwnd) != 0 {
            let length = GetWindowTextLengthW(hwnd) as usize;
            if length > 0 {
                let mut buffer = vec![0u16; length + 1];
                GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
                let title = String::from_utf16_lossy(&buffer[..length]);
                windows.push((hwnd, title));
            }
        }
        1 // Continue enumeration
    }

    // Enumerate all visible windows
    unsafe {
        EnumWindows(Some(enum_windows_callback), &mut windows as *mut _ as isize);
    }

    if windows.is_empty() {
        let _ = info("No visible windows found.");
        return None;
    }

    let _ = clear_screen();

    // Apply the dynamic theme
    set_theme(theme.clone());

    let _ = intro(format!("{}", theme.color.apply_to(" Please select a window to attach to! ")));

    // Create a `cliclack::Select` prompt
    let mut selector = select(format!("{}", theme.color.apply_to("Select a window:")));

    for (index, (_, title)) in windows.iter().enumerate() {
        selector = selector.item(index, title, "(Window)"); // Add each window with index as the key and "Window" as description
    }

    let _ = outro(format!(
        "Problems? {}",
        theme.color.apply_to("https://example.com/issues").underlined()
    ));

    // Show the selection menu and get the selected index
    match selector.interact() {
        Ok(selected_index) => {
            if let Some((selected_hwnd, selected_title)) = windows.get(selected_index) {
                let _ = info(format!(
                    "Attached to window: {}",
                    theme.color.apply_to(selected_title)
                ));
                Some(*selected_hwnd) // Return the HWND of the selected window
            } else {
                let _ = info("Invalid selection.");
                None
            }
        }
        Err(_) => {
            let _ = info("No selection made or operation canceled.");
            None
        }
    }
}

/// Checks if the required directory structure exists.
pub fn check_requirements(dir_name: &str) -> bool {
    let base_path = Path::new(dir_name);
    base_path.is_dir()
        && base_path.join("dataset").is_dir()
        && base_path.join("model").is_dir()
        && base_path.join("config.yaml").is_file()
}

/// Deletes a directory if all required items are present.
pub async fn check_and_delete_directory(dir_name: &str) -> io::Result<()> {
    let base_path = Path::new(dir_name);

    if check_requirements(dir_name) {
        println!("All required items found in '{}'. Deleting the parent directory...", dir_name);
        fs::remove_dir_all(base_path).await?;
        println!("Directory '{}' deleted successfully!", dir_name);
    } else {
        println!("Directory '{}' does not contain all required items. No action taken.", dir_name);
    }

    Ok(())
}
