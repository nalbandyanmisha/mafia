use super::role::Role;
use std::fmt;

/// Player status at the table.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LifeStatus {
    #[default]
    Alive,
    Killed,
    Eliminated,
    Removed,
}

/// Warnings and penalties applied to a player.
#[derive(Debug, Clone, Default)]
struct WarningPenalty {
    pub pending_silence: bool,
}

/// Representation of a player.
#[derive(Debug, Clone, Default)]
pub struct Player {
    name: String,
    role: Role,
    warnings: u8,
    penalty: WarningPenalty,
    life_status: LifeStatus,
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
            life_status: LifeStatus::Alive,
        }
    }

    // identity
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn role(&self) -> Role {
        self.role
    }

    // observation
    pub fn life_status(&self) -> LifeStatus {
        self.life_status
    }

    pub fn is_alive(&self) -> bool {
        matches!(self.life_status, LifeStatus::Alive)
    }

    pub fn is_removed(&self) -> bool {
        matches!(self.life_status, LifeStatus::Removed)
    }

    pub fn is_dead(&self) -> bool {
        matches!(
            self.life_status,
            LifeStatus::Killed | LifeStatus::Eliminated
        )
    }

    pub fn is_eliminated(&self) -> bool {
        matches!(self.life_status, LifeStatus::Eliminated)
    }

    // warnings
    pub fn warnings(&self) -> u8 {
        self.warnings
    }
    pub fn has_warnings(&self) -> bool {
        self.warnings > 0
    }
    pub fn is_silenced(&self) -> bool {
        self.penalty.pending_silence
    }
    pub fn increment_warning(&mut self) -> Result<(), PlayerError> {
        if self.life_status == LifeStatus::Removed {
            return Err(PlayerError::MaxWarningsReached);
        }

        self.warnings += 1;

        match self.warnings {
            3 => self.penalty.pending_silence = true,
            4 => self.life_status = LifeStatus::Removed,
            _ => {}
        }

        Ok(())
    }
    pub fn deincrement_warning(&mut self) -> Result<(), PlayerError> {
        if self.warnings == 0 {
            return Err(PlayerError::NoWarnings);
        }

        self.warnings -= 1;

        // Adjust penalties
        if self.warnings < 3 {
            self.penalty.pending_silence = false;
        }
        if self.life_status == LifeStatus::Removed && self.warnings < 4 {
            self.life_status = LifeStatus::Alive;
        }

        Ok(())
    }

    // explicit transitions
    pub fn mark_killed(&mut self) {
        self.life_status = LifeStatus::Killed;
    }

    pub fn mark_eliminated(&mut self) {
        self.life_status = LifeStatus::Eliminated;
    }

    pub fn mark_removed(&mut self) {
        self.life_status = LifeStatus::Removed;
    }

    pub fn restore_alive(&mut self) {
        self.life_status = LifeStatus::Alive;
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:?}) – Status: {:?}, Warnings: {}{}",
            self.name,
            self.role,
            self.life_status,
            self.warnings,
            if self.penalty.pending_silence {
                ", Silenced"
            } else {
                ""
            }
        )
    }
}

impl fmt::Display for LifeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status_str = match self {
            LifeStatus::Alive => "Alive",
            LifeStatus::Killed => "Killed",
            LifeStatus::Eliminated => "Eliminated",
            LifeStatus::Removed => "Removed",
        };
        write!(f, "{}", status_str)
    }
}
