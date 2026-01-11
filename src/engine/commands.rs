use crate::domain::position::Position;

#[derive(Debug)]
pub enum Command {
    Join { name: String },
    Leave { name: String },
    Start,
    Advance,
    AssignRole,
    RevokeRole,
    Warn { target: Position },
    Pardon { target: Position },
    Nominate { target: Position },
    Vote { targets: Vec<Position> },
    Shoot { target: Position },
    Check { target: Position },
}
