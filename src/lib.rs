use tokio::fs::{self, File};
use tokio::io::{self, AsyncWriteExt};
use std::path::Path;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{GetWindowTextW, IsWindowVisible, GetWindowTextLengthW};
use dialoguer::{theme::ColorfulTheme, Select};
use dialoguer::console::Term;


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
pub fn select_window() -> Option<String> {
    let mut windows: Vec<(HWND, String)> = Vec::new();

    // Callback to collect window handles
    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: isize) -> i32 {
        let windows = &mut *(lparam as *mut Vec<(HWND, String)>);

        if unsafe { IsWindowVisible(hwnd) } != 0 {
            let length = unsafe { GetWindowTextLengthW(hwnd) as usize };
            if length > 0 {
                let mut buffer = vec![0u16; length + 1];
                unsafe {
                    GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
                }
                let title = String::from_utf16_lossy(&buffer[..length]);
                windows.push((hwnd, title));
            }
        }
        1 // Continue enumeration
    }

    unsafe {
        winapi::um::winuser::EnumWindows(Some(enum_windows_callback), &mut windows as *mut _ as isize);
    }

    if windows.is_empty() {
        let _ = write_output("No visible windows found.");
        return None;
    }

    let titles: Vec<String> = windows.iter().map(|(_, title)| title.clone()).collect();

    // Create a custom terminal for dialoguer to ensure UTF-8 compatibility
    let term = Term::stdout();

    // Display a selection menu using dialoguer
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a window to attach to:")
        .items(&titles)
        .default(0)
        .interact_on_opt(&term)
        .expect("Failed to interact with user");

    selection.map(|index| titles[index].clone())
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
