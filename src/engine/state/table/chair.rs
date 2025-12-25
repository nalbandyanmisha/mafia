use std::fmt;

use crate::snapshot::{ChairData, Snapshot};

/// Logical seat at the table.
///
/// - Created only by `Table`
/// - Used as an identity / key
/// - Immutable and copyable
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Chair(u8);

impl Snapshot for Chair {
    type Output = ChairData;

    fn snapshot(&self) -> Self::Output {
        ChairData {
            position: self.0 as usize,
        }
    }
}

impl Chair {
    /// Create a chair at a given position.
    ///
    /// Visibility:
    /// - Only accessible by the parent `table` module
    /// - Prevents chair creation outside of `Table`
    pub(super) fn new(position: u8) -> Self {
        debug_assert!(position > 0, "chair position must be >= 1");
        Chair(position)
    }

    /// 1-based chair position at the table.
    pub fn position(&self) -> u8 {
        self.0
    }
}

impl fmt::Display for Chair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ChairError {
    #[error("invalid chair position {0}")]
    InvalidPosition(u8),
}
