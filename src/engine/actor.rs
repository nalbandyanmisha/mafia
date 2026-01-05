use crate::domain::position::Position;
use crate::snapshot::Snapshot;

#[derive(Debug, Clone)]
pub struct Actor {
    start: Position,
    current: Option<Position>,
    completed: bool,
}

impl Snapshot for Actor {
    type Output = Option<Position>;

    fn snapshot(&self) -> Self::Output {
        self.current()
    }
}

impl Actor {
    pub fn new(start: Position) -> Self {
        Self {
            start,
            current: None,
            completed: false,
        }
    }

    pub fn reset(&mut self, start: Position) {
        self.start = start;
        self.current = None;
        self.completed = false;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn current(&self) -> Option<Position> {
        self.current
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn set_current(&mut self, position: Option<Position>) {
        self.current = position;
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }

    pub fn set_start(&mut self, position: Position) {
        self.start = position;
    }
}
