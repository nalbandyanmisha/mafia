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
}
