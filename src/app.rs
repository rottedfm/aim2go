use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use xcap::Window;
use std::fs;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// Game
    pub game: String,

    // Game Window
    pub game_window: String,

    /// Are we editing a config?
    pub editing: bool,

    /// Thread handle for screen capture
    pub capture_thread: Option<thread::JoinHandle<()>>,

    /// Shared state for controlling the capture thread
    pub capturing: Arc<Mutex<bool>>,

    /// Frames per second for screen capture
    pub fps: u32,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(game: &str, editing: bool, game_window: &str, fps: u32) -> Self {
        Self {
            running: true,
            game: game.to_string(),
            game_window: game_window.to_string(),
            editing,
            capture_thread: None,
            capturing: Arc::new(Mutex::new(false)),
            fps,
        }
    }

    /// Starts the screen capture thread for the game window.
    pub fn start_screen_capture(&mut self) -> AppResult<()> {
        let capturing = self.capturing.clone();
        let game_window = self.game_window.clone();
        let game = self.game.clone();
        let fps = self.fps;
        let frame_duration = Duration::from_secs_f32(1.0 / fps as f32);

        // Ensure the dataset directory exists
        let dataset_path = format!("games/{}/dataset", game);
        fs::create_dir_all(&dataset_path)?;

        *capturing.lock().unwrap() = true;

        let handle = thread::spawn(move || {
            while *capturing.lock().unwrap() {
                let start_time = Instant::now();

                let windows = Window::all().unwrap_or_else(|_| vec![]);

                for window in windows {
                    if window.title().contains(&game_window) && !window.is_minimized() {
                        if let Ok(image) = window.capture_image() {
                            let filename = format!("{}/{}-{}.png",
                                dataset_path,
                                window.title(),
                                chrono::Utc::now().format("%Y%m%d%H%M%S%3f")
                            );
                            if let Err(e) = image.save(&filename) {
                                eprintln!("Failed to save capture: {}", e);
                            } else {
                                println!("Captured and saved: {}", filename);
                            }
                        }
                    }
                }

                let elapsed = start_time.elapsed();
                if elapsed < frame_duration {
                    thread::sleep(frame_duration - elapsed);
                }
            }

            println!("Screen capture thread stopped.");
        });

        self.capture_thread = Some(handle);
        println!("Screen capture thread started at {} FPS.", fps);
        Ok(())
    }

    /// Stops the screen capture thread.
    pub fn stop_screen_capture(&mut self) -> AppResult<()> {
        if let Some(handle) = self.capture_thread.take() {
            *self.capturing.lock().unwrap() = false;
            handle.join().expect("Failed to join screen capture thread");
            println!("Screen capture thread successfully joined.");
        } else {
            println!("No active screen capture thread to stop.");
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;

        // Ensure the screen capture thread is stopped
        if let Err(e) = self.stop_screen_capture() {
            eprintln!("Error while stopping screen capture: {}", e);
        }
    }
}
