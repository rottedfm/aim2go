use ratatui::{
    prelude::*,
    widgets::{List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(60),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
        ])
        .split(frame.area());

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(vertical_layout[2]);

    let logo = Paragraph::new(
        r#"
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
                                                        ^"===*"`                                       ~`                  
                                        "#,
    )
    .alignment(Alignment::Center).style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

    frame.render_widget(logo, vertical_layout[0]);

    let items: Vec<ListItem> = app
        .menu_options
        .iter()
        .map(|item| {
            ListItem::new(Span::styled(
                item.clone(),
                Style::default().fg(Color::White),
            ))
        })
        .collect();

    let menu = List::new(items)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
        .highlight_symbol("/");
    frame.render_stateful_widget(menu, horizontal_layout[1], &mut app.menu_state);

    let hint = Paragraph::new(
        " Keybinds: <J/Down> Scroll Down <K/Up> Scroll Up <Enter> Select <Escape> Back <Q> Quit ",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::White))
    .italic();

    frame.render_widget(hint, vertical_layout[1]);
}
