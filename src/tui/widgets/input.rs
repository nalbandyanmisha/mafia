use crate::app::input::PopupKind;
use ratatui::{
    layout::Alignment,
    text::Text,
    widgets::{Paragraph, Wrap},
};

pub fn widget(content: &str) -> Paragraph<'_> {
    Paragraph::new(Text::from(content))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false })
}

/// Returns (width_pct, height) for an input popup of the given kind.
pub fn size(kind: PopupKind) -> (u16, u16) {
    let width = match kind {
        PopupKind::Join | PopupKind::Leave => 40,
        PopupKind::Nominate
        | PopupKind::Shoot
        | PopupKind::Check
        | PopupKind::Warn
        | PopupKind::Pardon => 35,
        PopupKind::Vote | PopupKind::Guess => 50,
        PopupKind::Help => 100,
    };
    (width, 3)
}
