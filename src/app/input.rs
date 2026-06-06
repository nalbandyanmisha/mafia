#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,  // modal commands (n, s, v, ...)
    Command, // ':' command line
    Popup { title: String, kind: PopupKind },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopupKind {
    Join,
    Leave,
    Nominate,
    Shoot,
    Check,
    Warn,
    Pardon,
    Vote,
    Guess,
    Help,
}

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
