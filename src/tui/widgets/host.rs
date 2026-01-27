pub mod footer;
pub mod header;
pub mod main;

use crate::tui::{layout, view};

use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, BorderType, Borders},
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
    Ok(())
}
