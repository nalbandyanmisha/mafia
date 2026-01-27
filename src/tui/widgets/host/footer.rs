use crate::tui::{layout, view};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Stylize},
    widgets::Paragraph,
};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::Footer,
    view: &view::host::Footer,
) -> anyhow::Result<()> {
    let paragraph = Paragraph::new(view.info.clone())
        .alignment(Alignment::Center)
        .add_modifier(Modifier::ITALIC);

    frame.render_widget(paragraph, layout.area);
    Ok(())
}
