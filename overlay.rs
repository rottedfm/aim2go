use eframe::{egui, App, NativeOptions};
use egui::{Color32, Pos2, Stroke, Vec2};
use crate::config::{load_game_config, CrosshairType};

/// Overlay Application using `egui`
struct OverlayApp {
    crosshair_type: CrosshairType,
    crosshair_size: f32,
    color: Color32,
}

impl OverlayApp {
    fn new(game: &str) -> Self {
        // Load the game configuration
        let config = load_game_config(game).expect("Failed to load game config");

        // Extract overlay settings
        let overlay_config = config.game.overlay;
        
        let color = Color32::from_rgb(255, 0, 255); // Default color (Magenta)
        
        Self {
            crosshair_type: overlay_config.crosshair_type,
            crosshair_size: overlay_config.crosshair_size as f32,
            color,
        }
    }
}

impl App for OverlayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (rect, _) = ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());

            // Calculate center of the overlay
            let center = rect.center();

            // Draw the selected crosshair type
            let painter = ui.painter();
            match self.crosshair_type {
                CrosshairType::Dot => {
                    painter.circle_filled(center, self.crosshair_size, self.color);
                }
                CrosshairType::Cross => {
                    let size = self.crosshair_size;
                    painter.line_segment(
                        [Pos2::new(center.x - size, center.y), Pos2::new(center.x + size, center.y)],
                        Stroke::new(2.0, self.color),
                    );
                    painter.line_segment(
                        [Pos2::new(center.x, center.y - size), Pos2::new(center.x, center.y + size)],
                        Stroke::new(2.0, self.color),
                    );
                }
            }
        });
    }
}

/// Function to create an overlay using `eframe` and `egui`
pub fn create_overlay(game: &str) {
    let options = NativeOptions {
        decorated: false,         // Removes window decorations (title bar, borders)
        transparent: true,        // Enables transparency
        always_on_top: true,      // Keeps the overlay above all other windows
        fullscreen: false,        // Not fullscreen, just a floating overlay
        maximized: false,
        resizable: false,         // Prevents resizing
        ..Default::default()
    };

    eframe::run_native(
        "Game Overlay",
        options,
        Box::new(|_cc| Box::new(OverlayApp::new(game))),
    )
    .expect("Failed to start overlay");
}
