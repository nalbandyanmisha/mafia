use crate::tui::{layout, view::host::HostView};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, host: &layout::Host, view: &HostView) -> Result<(), anyhow::Error> {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(view.title.clone())
            .title_alignment(Alignment::Center)
            .style(view.title_style),
        host.area,
    );

    frame.render_widget(
        Paragraph::new(view.header.text.clone())
            .alignment(Alignment::Center)
            .style(view.header.style),
        host.header,
    );

    draw_main(frame, host.body, view)?;
    draw_footer(frame, host.footer, view)?;
    Ok(())
}
fn draw_main(frame: &mut Frame, area: Rect, view: &HostView) -> Result<(), anyhow::Error> {
    let mut lines = vec![
        Line::from(view.main.title.clone()).style(Style::default().add_modifier(Modifier::BOLD)),
    ];

    if let Some(sub) = &view.main.subtitle {
        lines.push(Line::from(sub.clone()));
    }

    let text = Text::from(lines);
    let centered = centered_area(area, text.height() as u16);

    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered,
    );

    Ok(())
}

fn draw_footer(frame: &mut Frame, area: Rect, view: &HostView) -> Result<(), anyhow::Error> {
    frame.render_widget(
        Paragraph::new(view.footer.text.clone())
            .alignment(Alignment::Center)
            .style(view.footer.style),
        area,
    );
    Ok(())
}

fn centered_area(area: Rect, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    vertical[1]
}
