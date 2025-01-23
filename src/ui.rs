use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Block, Borders, BorderType, block::Position, List,  ListItem},
    layout::{Alignment, Constraint, Direction, Layout},
    Frame,
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
        let diagonal_position = row_index + col_index;
        let distance = diagonal_position as isize - center as isize;

        let color = if distance >= 0 && (distance as usize) < gradient_len {
            gradient[distance as usize]
        } else {
            Color::Magenta
        };

        spans.push(Span::styled(
            ch.to_string(),
            Style::default().fg(color),
        ));
    }

    Line::from(spans)
}

/// Checks if the terminal size is sufficient, otherwise shows a resize popup.
pub fn check_terminal_size(frame: &mut Frame, required_height: usize, required_width: usize) -> bool {
    let terminal_size = frame.area();
    if (terminal_size.height as usize) < required_height || (terminal_size.width as usize) < required_width {
        let popup = Paragraph::new("Please resize the terminal to view the full content.")
            .alignment(Alignment::Center)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title("Warning")
                .title_position(Position::Top)
                .title_alignment(Alignment::Center)
                .style(Style::default().fg(Color::Magenta)));
        
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
    let required_height = app.logo.lines().count() + 10;
    let required_width = app.logo.lines().map(|line| line.len()).max().unwrap_or(0) + 10;

    if !check_terminal_size(frame, required_height, required_width) {
        return;
    }

    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(5),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
            Constraint::Percentage(5),
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

    let gradient_lines: Vec<Line> = app.logo
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            gradient_line(line, row_index, app.logo_gradient, &gradient)
        })
        .collect();

    let ascii = Paragraph::new(gradient_lines)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Black));

    frame.render_widget(ascii, outer_layout[1]);

    let items = &app.menu_items;
    
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(Line::from(item.as_str())))
        .collect();
    
    let list = List::new(list_items)
        .style(Style::default().fg(Color::Magenta))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::LightMagenta))
        .highlight_symbol("/");

    frame.render_stateful_widget(list, inner_layout[1], &mut app.menu_state);}
