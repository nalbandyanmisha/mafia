use std::fmt;

use crate::{
    domain::position::Position,
    snapshot::{self, Snapshot},
};
use thiserror::Error;

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

#[derive(Debug, Clone)]
pub enum Event {
    SheriffChecked { chair: Position },
    DonChecked { chair: Position },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::SheriffChecked { chair } => {
                write!(f, "Sheriff checked {chair}")
            }
            Event::DonChecked { chair } => {
                write!(f, "Don checked {chair}")
            }
        }
    }
}

/// Errors that can occur during checks
#[derive(Debug, Error)]
pub enum Error {
    #[error("Sheriff has already checked a chair")]
    SheriffAlreadyChecked,

    #[error("Don has already checked a chair")]
    DonAlreadyChecked,
}

impl Check {
    pub fn record_sheriff_check(&mut self, chair: Position) -> Result<Vec<Event>, Error> {
        if self.sheriff.is_some() {
            return Err(Error::SheriffAlreadyChecked);
        }

        self.sheriff = Some(chair);
        Ok(vec![Event::SheriffChecked { chair }])
    }

    /// Record Don's check. Returns an event or an error if already checked.
    pub fn record_don_check(&mut self, chair: Position) -> Result<Vec<Event>, Error> {
        if self.don.is_some() {
            return Err(Error::DonAlreadyChecked);
        }

        self.don = Some(chair);
        Ok(vec![Event::DonChecked { chair }])
    }
}
