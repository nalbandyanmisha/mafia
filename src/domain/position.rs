use std::fmt;

use crate::snapshot::Snapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position(u8);

impl Position {
    /// Creates a position.
    ///
    /// This constructor is intentionally infallible.
    /// Validity is enforced by the game rules, not by Position itself.
    pub fn new(value: u8) -> Self {
        Position(value)
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

impl Snapshot for Position {
    type Output = Position;

    fn snapshot(&self) -> Self::Output {
        *self
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u8> for Position {
    fn from(value: u8) -> Self {
        Position::new(value)
    }
}

impl From<Position> for u8 {
    fn from(pos: Position) -> Self {
        pos.0
    }
}
