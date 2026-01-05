use crate::{
    domain::position::Position,
    snapshot::{self, Snapshot},
};

impl Snapshot for Check {
    type Output = snapshot::Check;

    fn snapshot(&self) -> Self::Output {
        snapshot::Check {
            sheriff: self.sheriff.map(|c| c.snapshot()),
            don: self.don.map(|c| c.snapshot()),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Check {
    sheriff: Option<Position>,
    don: Option<Position>,
}

impl Check {
    pub fn new() -> Self {
        Check {
            sheriff: None,
            don: None,
        }
    }

    pub fn record_sheriff_check(&mut self, chair: Position) {
        self.sheriff = Some(chair);
    }

    pub fn record_don_check(&mut self, chair: Position) {
        self.don = Some(chair);
    }

    pub fn sheriff(&self) -> Option<Position> {
        self.sheriff
    }

    pub fn don(&self) -> Option<Position> {
        self.don
    }
}
