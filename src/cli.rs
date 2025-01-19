use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aim2go")]
#[command(version = "0.1")]
#[command(about = r#"
               .                                                                                                           
              @88>                        .--~*teu.                                                  ..                    
              %8P      ..    .     :     dF     988Nx                     u.             uL   ..    @L               ..    
      u        .     .888: x888  x888.  d888b   `8888>      uL      ...ue888b          .@88b  @88R 9888i   .dL     .@88i   
   us888u.   .@88u  ~`8888~'888X`?888f` ?8888>  98888F  .ue888Nc..  888R Y888r        '"Y888k/"*P  `Y888k:*888.   ""%888>  
.@88 "8888" ''888E`   X888  888X '888>   "**"  x88888~ d88E`"888E`  888R I888>           Y888L       888E  888I     '88%   
9888  9888    888E    X888  888X '888>        d8888*`  888E  888E   888R I888>            8888       888E  888I   ..dILr~` 
9888  9888    888E    X888  888X '888>      z8**"`   : 888E  888E   888R I888>            `888N      888E  888I  '".-%88b  
9888  9888    888E    X888  888X '888>    :?.....  ..F 888E  888E  u8888cJ888     .    .u./"888&     888E  888I   @  '888k 
9888  9888    888&   "*88%""*88" '888!`  <""888888888~ 888& .888E   "*888*P"    .@8c  d888" Y888*"  x888N><888'  8F   8888 
"888*""888"   R888"    `~    "    `"`    8:  "888888*  *888" 888&     'Y"      '%888" ` "Y   Y"      "88"  888  '8    8888 
 ^Y"   ^Y'     ""                        ""    "**"`    `"   "888E               ^*                        88F  '8    888F 
                                                       .dWi   `88E                                        98"    %k  <88F  
                                                       4888~  J8%                                       ./"       "+:*%`   
                                                        ^"===*"`                                       ~`                  "#, long_about = None)]

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