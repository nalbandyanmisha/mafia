use crate::snapshot::{ChairData, Snapshot};

use super::table::chair::Chair;

#[derive(Debug, Clone)]
pub struct Actor {
    start: Chair,
    current: Option<Chair>,
    completed: bool,
}

impl Snapshot for Actor {
    type Output = Option<ChairData>;

    fn snapshot(&self) -> Self::Output {
        self.current.map(|c| c.snapshot())
    }
}

impl Actor {
    pub fn new(start: Chair) -> Self {
        Self {
            start,
            current: None,
            completed: false,
        }
    }

    pub fn reset(&mut self) {
        self.current = None;
        self.completed = false;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn current(&self) -> Option<Chair> {
        self.current
    }

    pub fn start(&self) -> Chair {
        self.start
    }

    pub fn set_current(&mut self, chair: Option<Chair>) {
        self.current = chair;
    }

    pub fn set_completed(&mut self, completed: bool) {
        self.completed = completed;
    }
}
