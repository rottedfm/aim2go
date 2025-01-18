use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};
use std::sync::mpsc::channel;
use std::path::Path;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{GetWindowTextW, IsWindowVisible, GetWindowTextLengthW, EnumWindows};
use cliclack::{select, intro, outro, log::info, clear_screen, set_theme, Theme, ThemeState};
use ctrlc;
use console::{style, Style};

struct MagentaTheme;

impl Theme for MagentaTheme {
    fn bar_color(&self, state: &ThemeState) -> Style {
        match state {
            ThemeState::Active => Style::new().magenta(),
            ThemeState::Error(_) => Style::new().red(),
            _ => Style::new().magenta(),
        }
    }

    fn state_symbol_color(&self, _state: &ThemeState) -> Style {
        Style::new().magenta()
    }

    fn info_symbol(&self) -> String {
        "[INFO]".into()
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

    // Create a blank 'config.yaml'
    let config_path = base_path.join("config.yaml");
    let mut file = File::create(&config_path).await?;
    file.write_all(b"# Config").await?;
    println!("File '{}' created successfully!", config_path.display());

    Ok(())
}

/// Writes UTF-8 output to the standard output asynchronously.
async fn write_output(message: &str) -> io::Result<()> {
    let mut stdout = tokio::io::stdout();
    stdout.write_all(message.as_bytes()).await?;
    stdout.write_all(b"\n").await?;
    stdout.flush().await?;
    Ok(())
}

/// Prompts the user to select a visible window.
/// Prompts the user to select a visible window.
pub fn select_window() -> Option<String> {
    let mut windows: Vec<(HWND, String)> = Vec::new();

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
        info("No visible windows found.");
        return None;
    }

    // Extract window titles for selection
    let titles: Vec<String> = windows.iter().map(|(_, title)| title.clone()).collect();

    ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");

    clear_screen();

    set_theme(MagentaTheme);

    intro(style(" Please select a window to attach to! ").on_magenta().black());

   // Create a `cliclack::Select` prompt

    let mut selector = select(style("Select a window:").on_magenta().black());

    for (index, (_, title)) in windows.iter().enumerate() {
        selector = selector.item(index, title, "Window"); // Add each window with index as the key and "Window" as description
    }

    outro(format!(
        "Problems? {}\n",
        style("https://example.com/issues").cyan().underlined()
    ));

    // Show the selection menu and get the selected index
    match selector.interact() {
        Ok(selected_index) => {
            if let Some((_, selected_title)) = windows.get(selected_index) {
                info(format!("Selected: {}", selected_title));
                Some(selected_title.clone())
            } else {
                info("Invalid selection.");
                None
            }
        }
        Err(_) => {
            info(style("No selection made or operation canceled."));
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