use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aim2go")]
#[command(version = "0.1")]
#[command(about = r#"
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
                                                        ^"===*"`"#, long_about = None)]

pub struct Cli {
	#[command(subcommand)]
	pub command: Option<Commands>,

	/// Lists avalible games
	#[arg(short, long)]
	pub list: bool,

}

#[derive(Subcommand)]
pub enum Commands {
	/// Creates a new config for a game
	New {
	   game: String,
        },

	/// Removes a games config
	Remove {
	   game: String,
        },

	/// Attaches to a game
	Attach {
	   game: String,
	}, 
}