use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::tui::app::App;

mod buttons {
    use ratatui::style::{Color, Modifier, Style};
    pub const NAVIGATION_KEY: Style = Style::new()
        .on_black()
        .fg(Color::LightRed)
        .add_modifier(Modifier::BOLD);
    pub const MODIFIER_KEY: Style = Style::new()
        .on_black()
        .fg(Color::LightGreen)
        .add_modifier(Modifier::BOLD);
    pub const NAVIGATION_HELP_STRING: Style = Style::new().fg(Color::White);
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_window_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(10), Constraint::Length(3)])
        .split(frame.area());

    // Navigation Bar Controls
    let wrap_or_unwrap = if app.wrap_output { "unwrap" } else { "wrap" };
    let main_navigation = vec![
        Span::styled(" ESC ", buttons::NAVIGATION_KEY),
        Span::styled(" quit | ", buttons::NAVIGATION_HELP_STRING),
        Span::styled(" CTRL + ", buttons::MODIFIER_KEY),
        Span::styled(" < ", buttons::NAVIGATION_HELP_STRING),
        Span::styled(" R ", buttons::NAVIGATION_KEY),
        Span::styled(" run | ", buttons::NAVIGATION_HELP_STRING),
        Span::styled(" W ", buttons::NAVIGATION_KEY),
        Span::styled(
            format!(" {} output | ", wrap_or_unwrap),
            buttons::NAVIGATION_HELP_STRING,
        ),
        Span::styled("     ", buttons::NAVIGATION_KEY),
        Span::styled(" or ", buttons::NAVIGATION_HELP_STRING),
        Span::styled(" k j h l ", buttons::NAVIGATION_KEY),
        Span::styled(" scroll table ", buttons::NAVIGATION_HELP_STRING),
        Span::styled(" > ", buttons::NAVIGATION_HELP_STRING),
    ];
    let main_footer =
        Paragraph::new(Line::from(main_navigation)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(main_footer, main_window_chunks[1]);

    // Query vs. Results Chunks
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_window_chunks[0]);

    // Query Block UI
    app.query_editor.set_block(
        Block::bordered()
            .title_top(" Query Editor ")
            .title_alignment(ratatui::layout::HorizontalAlignment::Center),
    );
    frame.render_widget(&app.query_editor, horizontal_chunks[0]);
    let (output_header, output_content) = app.output_header_and_content();
    let output_header_length = output_header.lines().count().clamp(0, u16::MAX as usize) as u16;

    // Query Results UI
    let query_viewer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(output_header_length + 1),
            Constraint::Min(1),
        ])
        .split(horizontal_chunks[1]);
    let result_table_header_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
        .title_top(" Output ")
        .title_alignment(ratatui::layout::HorizontalAlignment::Center)
        .style(Style::default());
    let result_table_header = Paragraph::new(Text::styled(
        output_header,
        Style::default().fg(Color::Green),
    ))
    .scroll((0, app.scroll_coord.1))
    .block(result_table_header_block);
    frame.render_widget(result_table_header, query_viewer_chunks[0]);
    app.set_viewport_dims(
        horizontal_chunks[1].height - 2,
        horizontal_chunks[1].width - 2,
    );
    let result_table_content_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .title_alignment(ratatui::layout::HorizontalAlignment::Center)
        .title_bottom(" Limited to 2048 rows ")
        .style(Style::default());
    let result_table_content = Paragraph::new(Text::styled(
        output_content,
        Style::default().fg(Color::Green),
    ))
    .scroll(app.scroll_coord)
    .block(result_table_content_block);
    frame.render_widget(result_table_content, query_viewer_chunks[1]);
}
