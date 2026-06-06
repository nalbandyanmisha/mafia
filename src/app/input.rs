#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,   // modal commands (h, j, l, n, :, etc.)
    Command,  // ':' command line
    Input,    // typing text (popup or command line)
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
