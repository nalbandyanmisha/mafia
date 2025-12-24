use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn draw_layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
    let screen = frame.area();

    // 1️⃣ Split screen: left (table+command) | right (event log)
    let [left, event_log] =
        Layout::horizontal([Constraint::Percentage(75), Constraint::Percentage(25)]).areas(screen);

    // 2️⃣ Split left side: table | command palette
    let [table, command] =
        Layout::vertical([Constraint::Min(10), Constraint::Length(3)]).areas(left);

    // 3️⃣ Render blocks
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" TABLE ")
            .style(Style::default().fg(Color::Green)),
        table,
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" COMMAND ")
            .style(Style::default().fg(Color::Cyan)),
        command,
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" EVENTS ")
            .style(Style::default().fg(Color::Magenta)),
        event_log,
    );

    (table, event_log, command)
}
