use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Block, Borders, BorderType, block::Position},
    layout::Alignment,
    Frame,
    prelude::*,
};
use crate::app::App;

/// Generates a single diagonal gradient across the entire ASCII art.
pub fn gradient_line(
    text: &str,
    row_index: usize,
    center: usize,
    gradient: &[Color],
) -> Line<'static> {
    let gradient_len = gradient.len();
    let mut spans = Vec::with_capacity(text.len());

    for (col_index, ch) in text.chars().enumerate() {
        // Calculate a diagonal position based on row and column
        let diagonal_position = row_index + col_index;

        // Determine the distance from the gradient center
        let distance = diagonal_position as isize - center as isize;

        // If within gradient bounds, use a gradient color; otherwise, reset
        let color = if distance >= 0 && (distance as usize) < gradient_len {
            gradient[distance as usize]
        } else {
            Color::Magenta // Default color for out-of-bounds
        };

        spans.push(Span::styled(
            ch.to_string(),
            Style::default().fg(color),
        ));
    }

    Line::from(spans)
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {

    let outer_layout = Layout::default()
	.direction(Direction::Vertical)
	.constraints(vec![
		Constraint::Percentage(30),
		Constraint::Percentage(55),
		Constraint::Percentage(15),
	])
	.split(frame.area());

     let inner_layout = Layout::default()
	.direction(Direction::Horizontal)
	.constraints(vec![
		Constraint::Percentage(10),
		Constraint::Percentage(90),
	])
	.split(outer_layout[1]);

    // animated logo

    let ascii_art = r#"
               .                                                                                                           
              @88>                        .--~*teu.                                                  ..                    
              %8P      ..    .     :     dF     988Nx                     u.             uL   ..    @L               ..    
      u        .     .888: x888  x888.  d888b   `8888>      uL      ...ue888b          .@88b  @88R 9888i   .dL     .@88i   
   us888u.   .@88u  ~`8888~'888X`?888f` ?8888>  98888F  .ue888Nc..  888R Y888r        '"Y888k/"*P  `Y888k:*888.   ""%888>  
.@88 "8888" ''888E`   X888  888X '888>   "**"  x88888~ d88E`"888E`  888R I888>           Y888L       888E  888I     '88%   
9888  9888    888E    X888  888X '888>        d8888*`  888E  888E   888R I888>            8888       888E  888I   ..dILr~` 
9888  9888    888E    X888  888X '888>      z8**"`   : 888E  888E   888R I888>            `888N      888E  888I  '".-%88b  
9888  9888    888E    X888  888X '888>    :?.....  ..F 888E  888E  u8888cJ888     .    .u./"888&     888E  888I   @  '888k 
9888  9888    888&   "*88%""*88" '888!`  <""888888888~ 888& .888E   "*888*P"    .@8c  d888" Y888*"  x888N><888' 8F   8888 
"888*""888"   R888"    `~    "    `"`    8:  "888888*  *888" 888&     'Y"      '%888" ` "Y   Y"      "88"  888  '8    8888 
 ^Y"   ^Y'     ""                        ""    "**"`    `"   "888E               ^*                        88F  '8    888F 
                                                       .dWi   `88E                                        98"    %k  <88F  
                                                       4888~  J8%                                       ./"       "+:*%`   
                                                        ^"===*"`                                       ~`                  "#;

   // Gradient colors for the shimmer effect
    let gradient = vec![
	Color::LightMagenta,
	Color::LightMagenta,
	Color::LightMagenta,
	Color::LightMagenta,
	Color::LightMagenta,
	Color::LightMagenta,
	Color::White,
	Color::White,
	Color::White,
	Color::White,
	Color::White,
	Color::White,
	 
    ];	

    let gradient_lines: Vec<Line> = ascii_art
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            gradient_line(line, row_index, app.logo_gradiant, &gradient)
        })
        .collect();

    let  ascii = Paragraph::new(gradient_lines)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black))
	.block(Block::default().borders(Borders::ALL).border_type(BorderType::Plain).title(app.game_window.clone()).title_position(Position::Bottom).title_alignment(Alignment::Center).title_style(Style::default().light_magenta()).style(Style::default().magenta()));


    frame.render_widget(ascii, outer_layout[0]);

    let test = Paragraph::new("Test").block(Block::default().borders(Borders::ALL).border_type(BorderType::Plain).style(Style::new().magenta()));

    frame.render_widget(&test, inner_layout[0]);
    frame.render_widget(&test, inner_layout[1]);
    frame.render_widget(&test, outer_layout[2]);
	

    
    

}
