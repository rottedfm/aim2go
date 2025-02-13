use std::io;
use clap::Parser;
use ratatui::{backend::CrosstermBackend, Terminal};
use aim2go::{check_and_delete_directory, create_directory, check_requirements, select_window};
use crate::{
    app::{App, AppResult},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
    cli::{Cli, Commands},
    config::load_game_config,
};

pub mod cli;
pub mod app;
pub mod event;
pub mod handler;
pub mod thread;
pub mod tui;
pub mod ui;
pub mod overlay;
pub mod config;

#[tokio::main] async fn main() -> AppResult<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    if cli.list {
        println!("Feature '--list' is not implemented yet.");
    }

    match &cli.command {
        Some(Commands::New { game }) => {
            if let Err(e) = create_directory(&game).await {
                eprintln!("Error creating directory '{}': {}", game, e);
            }
        }
        Some(Commands::Remove { game }) => {
            if let Err(e) = check_and_delete_directory(&game).await {
                eprintln!("Error removing directory '{}': {}", game, e);
            }
        }
         Some(Commands::Attach { game }) => {
            if check_requirements(&game) {
                println!("Select a window to attach to...");

            
                let selected_window = select_window(&game);
                if selected_window.is_none() {
                    eprintln!("No window selected. Exiting...");
                    return Ok(());
                }

                let selected_window = selected_window.unwrap(); // Safe because we checked

                let mut app = App::new(&game, selected_window);
        
                let config = load_game_config(&game)?;

                let tick_rate = config.app.tick_rate;
                
                let stdout = io::stdout();
                let backend = CrosstermBackend::new(stdout);
                let terminal = Terminal::new(backend)?;
                let events = EventHandler::new(tick_rate);
                let mut tui = Tui::new(terminal, events);

                // Initialize TUI
                if let Err(e) = tui.init() {
                    eprintln!("Failed to initialize TUI: {}", e);
                    return Err(e);
                }

                // Start main loop
                let result = run_tui(&mut tui, &mut app).await;

                // Exit TUI gracefully
                if let Err(e) = tui.exit() {
                    eprintln!("Failed to exit TUI: {}", e);
                }

                result?;
            } else {
                println!("The specified game is missing required components. Please check and try again.");
            }
        }
        None => {
            println!("No command provided. Use '--help' to see available options.");
        }
    }

    Ok(())
}

/// Runs the TUI main loop.
async fn run_tui<B: ratatui::backend::Backend>(
    tui: &mut Tui<B>,
    app: &mut App,
) -> AppResult<()> {
    while app.running {
        // Render the user interface.
        tui.draw(app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick().await,
            Event::Key(key_event) => handle_key_events(key_event, app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    Ok(())
}
