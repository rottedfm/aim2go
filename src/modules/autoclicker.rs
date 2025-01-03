use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::id,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
enigo::{
    Enigo,
}

pub struct AutoClickerProcess {
    cps: i8,
    range: Option<i8>,
    button: String,
    config: Option<PathBuf>,
    pid_file: String,
    running: Arc<Mutex<bool>>,
}

impl AutoClickerProcess {
    pub fn new(cps: i8, range: Option<i8>, button: &str, config: Option<PathBuf>) {
        Self {
            range,
            cps,
            button: button.to_string(),
            pid_file: "/tmp/autoclicker_process.pid".to_string(),
            running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start(&self) {
        if Path::new(&self.pid_file).exists() {
            println!("AutoClicker is already running. PID file exits.");
            return;
        }

        let pid = id();
        let mut file = File::create(&self.pid_file).expect("Failed to create PID file");
        writeln!(file, "{}", pid).expect("Failed to write PID to file");
        println!("Starting AutoClicker with PID: {}", pid);

        let running_clone = Arc::clone(&self.running);
        thread::spawn(move || {
            *running_clone.lock().unwrap() = true;
            while *running_clone.lock().unwrap() {
                // Autoclicker logic
                let mut enigo = Enigo::new(&Settings::default()).unwrap();

                // Make auto clicker here
                // if config use config settings else
                // use cps to set the clicks persecond
                // else if the range value is persent use it to make a random range to make autoclicking look more human
                // else if button match RMB or LMB for mouse buttons 
            }
            println!("AutoClicker stopped.")
        });
    }

    pub fn stop(&self) {
        if !Path::new(&self.pid_file).exists() {
            println!("AutoClicker is not running. No PID file found.");
            return;
        }

        println!("Stopping AutoClicker...");
        *self.running.lock().unwrap() = false;

        fs::remove_file(&self.pid_file).expect("Failed to remove PID file");
        println!("AutoClicker stoped and PID file removed.");
    }

    pub fn is_running(&self) -> bool {
        Path::new(&self.pid_file).exists()
    }
}
