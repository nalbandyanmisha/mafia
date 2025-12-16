use super::role::Role;
use std::fmt;

/// Player status at the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Alive,
    Killed,
    Eliminated,
    Removed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Alive
    }
}

/// Warnings and penalties applied to a player.
#[derive(Debug, Clone, Default)]
pub struct WarningPenalty {
    pub pending_silence: bool,
}

/// Representation of a player.
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    role: Role,
    warnings: u8,
    penalty: WarningPenalty,
    status: Status,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::new(),
            role: Role::Citizen, // default safe role
            warnings: 0,
            penalty: WarningPenalty::default(),
            status: Status::default(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PlayerError {
    #[error("Cannot add more warnings; player is already removed")]
    MaxWarningsReached,
    #[error("Cannot remove warning; player has zero warnings")]
    NoWarnings,
}

impl Player {
    /// Create a new player with a name and role
    pub fn new(name: String, role: Role) -> Self {
        Player {
            name,
            role,
            warnings: 0,
            penalty: WarningPenalty::default(),
            status: Status::Alive,
        }
    }

    /// Player's display name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Player's role
    pub fn role(&self) -> Role {
        self.role
    }

    /// Current number of warnings
    pub fn warnings(&self) -> u8 {
        self.warnings
    }

    /// Current status
    pub fn status(&self) -> Status {
        self.status
    }

    /// Check if the player is alive
    pub fn is_alive(&self) -> bool {
        self.status == Status::Alive
    }

    /// Add a warning to the player, updating penalty/status if necessary
    pub fn add_warning(&mut self) -> Result<(), PlayerError> {
        if self.status == Status::Removed {
            return Err(PlayerError::MaxWarningsReached);
        }

        self.warnings += 1;

        match self.warnings {
            3 => self.penalty.pending_silence = true,
            4 => self.status = Status::Removed,
            _ => {}
        }

        Ok(())
    }

    /// Remove a warning from the player
    pub fn remove_warning(&mut self) -> Result<(), PlayerError> {
        if self.warnings == 0 {
            return Err(PlayerError::NoWarnings);
        }

        self.warnings -= 1;

        // Adjust penalties
        if self.warnings < 3 {
            self.penalty.pending_silence = false;
        }
        if self.status == Status::Removed && self.warnings < 4 {
            self.status = Status::Alive;
        }

        Ok(())
    }

    /// Reset all warnings and penalties
    pub fn reset_warnings(&mut self) {
        self.warnings = 0;
        self.penalty.pending_silence = false;
        if self.status == Status::Removed {
            self.status = Status::Alive;
        }
    }

    /// Manually kill the player
    pub fn kill(&mut self) {
        self.status = Status::Killed;
    }

    /// Manually eliminate the player (e.g., voted out)
    pub fn eliminate(&mut self) {
        self.status = Status::Eliminated;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:?}) – Status: {:?}, Warnings: {}{}",
            self.name,
            self.role,
            self.status,
            self.warnings,
            if self.penalty.pending_silence {
                ", Silenced"
            } else {
                ""
            }
        )
    }
}
