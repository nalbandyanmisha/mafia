pub mod footer;
pub mod header;
pub mod main;

use crate::{
    app::input::InputMode,
    tui::{layout, util::centered_area, view},
};

use ratatui::{
    Frame,
    layout::Alignment,
    style::Color,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};

pub fn draw(frame: &mut Frame, host: &layout::Host, view: &view::HostView) -> anyhow::Result<()> {
    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(view.title.clone())
            .title_alignment(Alignment::Center)
            .style(view.title_style),
        host.area,
    );

    header::draw(frame, &host.header, &view.header)?;
    main::draw(frame, &host.body, &view.body)?;
    footer::draw(frame, &host.footer, &view.footer)?;

    if let InputMode::Popup { title, kind } = &view.input_mode {
        let (content, area, alignment) = match kind {
            _ => {
                let popup_area = centered_area(host.area, 3);
                (view.input.as_str(), popup_area, Alignment::Center)
            }
        };

        frame.render_widget(Clear, area);

        let popup_block = Block::default()
            .title(title.as_str())
            .borders(Borders::ALL)
            .border_style(Color::White)
            .border_type(BorderType::Thick);

        let paragraph = Paragraph::new(content)
            .block(popup_block)
            .alignment(alignment)
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    }

    Ok(())
}
