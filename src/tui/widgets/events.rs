// use ratatui::{
//     Frame,
//     style::{Color, Style},
//     widgets::{Block, Borders, Paragraph},
// };
//
// use crate::{snapshot::App, tui::layout};
//
// pub fn draw(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
//     let layout = layout::Events::new(area);
//
//     frame.render_widget(
//         Block::default()
//             .borders(Borders::ALL)
//             .title(" EVENTS ")
//             .style(Style::default().fg(Color::Magenta)),
//         layout.area,
//     );
//
//     // placeholder for now
//     frame.render_widget(Paragraph::new("No events yet"), layout.content);
// }

use ratatui::{
    Frame,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{tui::layout, tui::view::events::EventsView};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::Events,
    view: &EventsView,
    app: &crate::snapshot::App,
) {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title(" EVENTS ")
            .style(Style::default().fg(Color::Magenta)),
        layout.area,
    );

    let lines: Vec<Line> = if view.messages.is_empty() {
        vec![Line::from("No events yet")]
    } else {
        view.messages
            .iter()
            .map(|m| Line::from(m.clone()))
            .collect()
    };

    frame.render_widget(Paragraph::new(lines), layout.content);
}
