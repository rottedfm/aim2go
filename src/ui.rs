use ratatui::{
    style::{Color, Style},
    text::{Text, Line, Span},
    widgets::{Paragraph, Block, Borders, BorderType, block::Position, List,  ListItem},
    layout::{Alignment, Constraint, Direction, Layout},
    Frame,
};
use crate::app::{App, Mode};
use crate::config::load_game_config;

/// Returns the corresponding theme color from config
fn get_theme_color(game: &str) -> Color {
    let config = load_game_config(game).unwrap_or_else(|_| panic!("Failed to load config"));

    match config.app.theme.as_str() {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "yellow" => Color::Yellow,
        _ => Color::Magenta,
    }
}

/// Generates a single diagonal gradient across the entire ASCII art.
pub fn gradient_line(
    text: &str,
    row_index: usize,
    center: usize,
    game: &str,
    gradient: &[Color],
) -> Line<'static> {
    let gradient_len = gradient.len();
    let mut spans = Vec::with_capacity(text.len());

    let theme_color = get_theme_color(&game);

    for (col_index, ch) in text.chars().enumerate() {
        let diagonal_position = row_index + col_index;
        let distance = diagonal_position as isize - center as isize;

        let color = if distance >= 0 && (distance as usize) < gradient_len {
            gradient[distance as usize]
        } else {
            theme_color
        };

        spans.push(Span::styled(
            ch.to_string(),
            Style::default().fg(color),
        ));
    }

    Line::from(spans)
}

/// Checks if the terminal size is sufficient, otherwise shows a resize popup.
pub fn check_terminal_size(frame: &mut Frame, game: &str, required_height: usize, required_width: usize) -> bool {
    let terminal_size = frame.area();
    let color = get_theme_color(game);
    if (terminal_size.height as usize) < required_height || (terminal_size.width as usize) < required_width {
        let popup = Paragraph::new("Please resize the terminal to view the full content.")
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Warning")
                .title_position(Position::Top)
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(color)));
        
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
            .split(frame.area());
        
        frame.render_widget(popup, popup_layout[1]);
        return false;
    }
    true
}

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    match &app.mode {
        Mode::Menu => {
            let required_logo_height = app.logo.lines().count();
            let required_select_height = app.menu_items.len();
            let required_height = required_logo_height + required_select_height + 35;
            let required_width = app.logo.lines().map(|line| line.len()).max().unwrap_or(0) + 10;

            if !check_terminal_size(frame, &app.game, required_height, required_width) {
                return;
            }

            let outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Min(required_logo_height.try_into().unwrap()),
                    Constraint::Max(20),
                    Constraint::Min(required_select_height.try_into().unwrap()),
                    Constraint::Max(10),
                    Constraint::Max(5),
                ])
                .split(frame.area());

            let inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(50),
                    Constraint::Percentage(25),
                ])
                .split(outer_layout[2]);

            let theme_color = get_theme_color(&app.game);
    
            let gradient = vec![
                theme_color,
                theme_color,
                theme_color,
                theme_color,
                theme_color,
                theme_color,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
                Color::White,
            ];

            let gradient_lines: Vec<Line> = app.logo
                .lines()
                .enumerate()
                .map(|(row_index, line)| {
                    gradient_line(line, row_index, app.logo_gradient, &app.game, &gradient)
                })
                .collect();

            let ascii = Paragraph::new(gradient_lines)
                .alignment(Alignment::Center)
                .style(Style::default().bg(Color::Black));

            frame.render_widget(ascii, outer_layout[0]);


            let hint = Paragraph::new("Version: 0.1.0 \nMaintained by rottedfm \nAre you a rust developer and want get paid to contribute to aim2go? \nPlease visit: https://aim2go.xyz/contribute for more information!").alignment(Alignment::Center).style(Style::default().fg(theme_color));

            frame.render_widget(hint, outer_layout[4]);

            let items = &app.menu_items;
    
            let list_items: Vec<ListItem> = items
                .iter()
                .map(|item| ListItem::new(Line::from(item.as_str())))
                .collect();
    
            let list = List::new(list_items)
                .style(Style::default().fg(theme_color))
                .highlight_style(Style::default().fg(Color::Black).bg(theme_color))
                .highlight_symbol("/");

            frame.render_stateful_widget(list, inner_layout[1], &mut app.menu_state);      
        }
        Mode::Execute => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Max(97),
                    Constraint::Max(3),
                ])
                .split(frame.area());

            let theme_color = get_theme_color(&app.game);

            let log_lines: Vec<Line> = app
                .log
                .iter()
                .map(|entry| Line::from(Span::styled(entry, Style::default().fg(theme_color))))
                .collect();

            let log = Paragraph::new(Text::from(log_lines))
                .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("[Execution Log]").title_alignment(Alignment::Center).border_style(Style::default().fg(theme_color)));

            frame.render_widget(log, layout[0]);

            let input = Paragraph::new("/help or 'h' for keybind/commands").block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).border_style(Style::default().fg(theme_color))).style(Style::default().fg(theme_color));

            frame.render_widget(input, layout[1]);

        }
        _ => {}
    }
}
