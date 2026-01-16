use ratatui::{
    Frame,
    layout::Alignment,
    text::Text,
    widgets::{Paragraph, Wrap},
};

use crate::tui::{layout, util::centered_area};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::main::Description,
    text: &str,
) -> anyhow::Result<()> {
    let text = Text::from(text.to_string());
    let centered = centered_area(layout.area, text.height() as u16);
    frame.render_widget(
        Paragraph::new(text.to_string())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        centered,
    );

    Ok(())
}
