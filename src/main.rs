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
};

pub mod cli;
pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> AppResult<()> {
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
               
		    if let Some(selected_window) = select_window() {

		    let logo = r#"
		                   .                                                               
              @88>                        .--~*teu.                            
              %8P      ..    .     :     dF     988Nx                     u.   
      u        .     .888: x888  x888.  d888b   `8888>      uL      ...ue888b  
   us888u.   .@88u  ~`8888~'888X`?888f` ?8888>  98888F  .ue888Nc..  888R Y888r 
.@88 "8888" ''888E`   X888  888X '888>   "**"  x88888~ d88E`"888E`  888R I888> 
9888  9888    888E    X888  888X '888>        d8888*`  888E  888E   888R I888> 
9888  9888    888E    X888  888X '888>      z8**"`   : 888E  888E   888R I888> 
9888  9888    888E    X888  888X '888>    :?.....  ..F 888E  888E  u8888cJ888  
9888  9888    888&   "*88%""*88" '888!`  <""888888888~ 888& .888E   "*888*P"   
"888*""888"   R888"    `~    "    `"`    8:  "888888*  *888" 888&     'Y"      
 ^Y"   ^Y'     ""                        ""    "**"`    `"   "888E             
                                                       .dWi   `88E             
                                                       4888~  J8%              
                                                        ^"===*"`               "#;

            let mut app = App::new(&game, &selected_window, logo);
			
			let stdout = io::stdout();
			let backend = CrosstermBackend::new(stdout);
			let terminal = Terminal::new(backend)?;
			let events = EventHandler::new(16);
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
		}

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
            Event::Key(key_event) => handle_key_events(key_event, app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }
    Ok(())
}
