use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Configuration structure for both app-wide and game-specific settings.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: AppConfig,
    pub game: GameConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub tick_rate: u64,
    pub frame_rate: u64,
    pub theme: String,
    pub ascii_art: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameConfig {
    pub overlay: OverlayConfig,
    pub keybinds: KeybindConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OverlayConfig {
    pub render: Vec<String>,
    pub crosshair_type: CrosshairType,
    pub crosshair_size: u8,
    pub class_radial_size: u16,
    pub class_radial_position: HashMap<u16, u16>,
    pub hud_ascii: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CrosshairType {
    Dot,
    Cross,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct KeybindConfig {
    pub keyboard: HashMap<String, Actions>,
    pub mouse: HashMap<String, Actions>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Actions {
    ClickCapture,
    ClassCaptureWheel,
    SnapAim,
    CloseOverlay,
    Quit,
}

/// Returns the directory path for a specific game: `./<game>/`
fn get_game_config_dir(game: &str) -> PathBuf {
    PathBuf::from(".").join(game)
}

/// Returns the path to a game's `config.yaml` file inside its directory.
fn get_game_config_file(game: &str) -> PathBuf {
    get_game_config_dir(game).join("config.yaml")
}

/// Loads the configuration for a specific game.
/// If the game directory or config file is missing, it creates them with defaults.
pub fn load_game_config(game: &str) -> io::Result<Config> {
    let config_file = get_game_config_file(game);

    if !config_file.exists() {
        create_default_game_config(game)?;
    }

    let config_content = fs::read_to_string(&config_file)?;
    let config: Config = serde_yaml::from_str(&config_content)
        .unwrap_or_else(|_| panic!("Failed to parse config file: {:?}", config_file));

    Ok(config)
}

/// Saves the given configuration back to the game's `config.yaml` file.
pub fn save_game_config(game: &str, config: &Config) -> io::Result<()> {
    let config_file = get_game_config_file(game);

    if let Some(parent_dir) = config_file.parent() {
        fs::create_dir_all(parent_dir)?;
    }

    let yaml_data = serde_yaml::to_string(config).expect("Failed to serialize config");
    let mut file = fs::File::create(&config_file)?;
    file.write_all(yaml_data.as_bytes())?;
    Ok(())
}

/// Creates a default configuration file for a new game.
pub fn create_default_game_config(game: &str) -> io::Result<()> {
    let game_dir = get_game_config_dir(game);

    if !game_dir.exists() {
        fs::create_dir_all(&game_dir)?;
    }

    let default_config = Config {
        app: AppConfig {
            tick_rate: 60,
            frame_rate: 144,
            theme: "magenta".to_string(),
            ascii_art: r#"
                                ____      ,----,                      
               ,--,           ,'  , `.  .'   .' \                     
             ,--.'|        ,-+-,.' _ |,----,'    |            ,---.  
             |  |,      ,-+-. ;   , |||    :  .  ;,----._,.  '   ,'\ 
   ,--.--.   `--'_     ,--.'|'   |  ||;    |.'  //   /  ' / /   /   |
  /       \  ,' ,'|   |   |  ,', |  |,`----'/  ;|   :     |.   ; ,. :
 .--.  .-. | '  | |   |   | /  | |--'   /  ;  / |   | .\  .'   | |: :
  \__\/: . . |  | :   |   : |  | ,     ;  /  /-,.   ; ';  |'   | .; :
  ," .--.; | '  : |__ |   : |  |/     /  /  /.`|'   .   . ||   :    |
 /  /  ,.  | |  | '.'||   | |`-'    ./__;      : `---`-'| | \   \  / 
;  :   .'   \;  :    ;|   ;/        |   :    .'  .'__/\_: |  `----'  
|  ,     .-./|  ,   / '---'         ;   | .'     |   :    :          
 `--`---'     ---`-'                `---'         \   \  /           
                                                   `--`-'            "#.to_string(),
        },
        game: GameConfig {
            overlay: OverlayConfig {
                render: vec![
                    "HUD".to_string(),
                    "Crosshair".to_string(),
                    "ESP".to_string(),
                    "ActiveRadius".to_string(),
                ],
                crosshair_type: CrosshairType::Dot,
                crosshair_size: 5,
                class_radial_size: 100,
                class_radial_position: HashMap::new(),
                hud_ascii: "default".to_string(),
            },
            keybinds: KeybindConfig {
                keyboard: HashMap::from([
                    ("CTRL+Q".to_string(), Actions::ClassCaptureWheel),
                    ("CTRL+ESC".to_string(), Actions::CloseOverlay),
                    ("CTRL+SHIFT+ESC".to_string(), Actions::Quit),
                ]),
                mouse: HashMap::from([                    
                    ("LMB".to_string(), Actions::ClickCapture),
                    ("RMB".to_string(), Actions::SnapAim),
                ]),
            },
        },
    };

    save_game_config(game, &default_config)
}

/// Updates a game's configuration.
pub fn update_game_config(game: &str, new_config: GameConfig) -> io::Result<()> {
    let mut config = load_game_config(game)?;
    config.game = new_config;
    save_game_config(game, &config)
}
