use ratatui::{
    layout::{Alignment, Rect},
    text::Text,
    widgets::{Paragraph, Wrap},
};

pub fn help_text() -> &'static str {
    "NORMAL MODE
  :          Enter command mode
  j          Join player (popup)
  l          Leave player (popup)
  n          Next phase
  b          Start game
  w          Warn player (popup)
  p          Pardon player (popup)
  o          Nominate player (popup)
  c          Check player (popup)
  g          Guess targets (popup)
  v          Vote (popup)
  s          Shoot player (popup)
  h          Show help dialog
  Esc        Quit game

COMMAND MODE
  Enter      Execute command
  Esc        Return to normal mode
  Backspace  Delete character

POPUP MODE
  Enter      Confirm popup action
  Esc        Close popup
  Backspace  Delete character
  Any char   Type input"
}

pub fn widget() -> Paragraph<'static> {
    Paragraph::new(Text::from(help_text()))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
}

/// Returns (width_pct, height) for the help popup.
pub fn size(area: Rect) -> (u16, u16) {
    let lines = help_text().lines().count() as u16;
    let max = area.height.saturating_sub(4);
    (100, lines.min(max))
}
