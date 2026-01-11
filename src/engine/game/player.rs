use crate::domain::{position::Position, role::Role, status::Status};
use crate::snapshot::{self, Snapshot};

#[derive(Debug, Clone, Default)]
struct Penalty {
    pub silenced: bool,
}

/// Representation of a player
#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    role: Option<Role>,
    position: Option<Position>,
    warnings: u8,
    penalty: Penalty,
    status: Status,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot assign position; already assigned")]
    PositionAlreadyAssigned,
    #[error("Cannot assign role; already assigned")]
    RoleAlreadyAssigned,
    #[error("Cannot add warning; player is removed")]
    MaxWarningsReached,
    #[error("Cannot remove warning; player has zero warnings")]
    NoWarnings,
}

impl Snapshot for Player {
    type Output = snapshot::Player;

    fn snapshot(&self) -> Self::Output {
        snapshot::Player {
            name: self.name.to_string(),
            position: self.position,
            role: self.role,
            warnings: self.warnings,
            status: self.status,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::new(),
            role: None,
            position: None,
            warnings: 0,
            penalty: Penalty::default(),
            status: Status::Alive,
        }
    }
}

impl Player {
    /// Create a new player with a name
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    /// Assign a position (immutable once set)
    pub fn assign_position(&mut self, pos: Position) -> Result<(), Error> {
        if self.position.is_some() {
            return Err(Error::PositionAlreadyAssigned);
        }
        self.position = Some(pos);
        Ok(())
    }

    pub fn clear_position(&mut self) {
        self.position = None;
    }

    /// Assign a role (immutable once set)
    pub fn assign_role(&mut self, role: Role) -> Result<(), Error> {
        if self.role.is_some() {
            return Err(Error::RoleAlreadyAssigned);
        }
        self.role = Some(role);
        Ok(())
    }

    pub fn clear_role(&mut self) {
        self.role = None;
    }

    // ----------- Accessors ----------
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn role(&self) -> Option<Role> {
        self.role
    }
    pub fn position(&self) -> Option<Position> {
        self.position
    }
    pub fn status(&self) -> Status {
        self.status
    }
    pub fn warnings(&self) -> u8 {
        self.warnings
    }
    pub fn is_silenced(&self) -> bool {
        self.penalty.silenced
    }

    // ----------- Life transitions ----------
    pub fn mark_dead(&mut self) {
        self.status = Status::Dead;
    }
    pub fn mark_eliminated(&mut self) {
        self.status = Status::Eliminated;
    }
    pub fn mark_removed(&mut self) {
        self.status = Status::Removed;
        self.penalty.silenced = false;
    }
    pub fn restore_alive(&mut self) {
        self.status = Status::Alive;
    }

    // ----------- Warnings ----------
    pub fn add_warning(&mut self) -> Result<(), Error> {
        if self.status == Status::Removed {
            return Err(Error::MaxWarningsReached);
        }

        self.warnings += 1;

        match self.warnings {
            3 => self.penalty.silenced = true,
            4 => self.mark_removed(),
            _ => {}
        }

        Ok(())
    }

    pub fn remove_warning(&mut self) -> Result<(), Error> {
        if self.warnings == 0 {
            return Err(Error::NoWarnings);
        }

        self.warnings -= 1;

        if self.warnings < 3 {
            self.penalty.silenced = false;
        }
        if self.status == Status::Removed && self.warnings < 4 {
            self.restore_alive();
        }

        Ok(())
    }

    // ----------- Queries ----------
    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }
    pub fn is_alive(&self) -> bool {
        self.status == Status::Alive
    }
    pub fn is_dead(&self) -> bool {
        matches!(self.status, Status::Dead | Status::Eliminated)
    }
    pub fn is_removed(&self) -> bool {
        self.status == Status::Removed
    }
    pub fn is_eliminated(&self) -> bool {
        self.status == Status::Eliminated
    }

    pub fn is_mafia(&self) -> bool {
        matches!(self.role, Some(Role::Don) | Some(Role::Mafia))
    }

    pub fn is_sheriff(&self) -> bool {
        matches!(self.role, Some(Role::Sheriff))
    }

    pub fn is_don(&self) -> bool {
        matches!(self.role, Some(Role::Don))
    }
}
