use std::fmt;

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
    HasPosition,
    #[error("Cannot assign role; already assigned")]
    HasRole,
    #[error("Cannot add warning; player is removed")]
    MaxWarningsReached,
    #[error("Cannot remove warning; player has zero warnings")]
    NoWarnings,
}

#[derive(Debug, Clone)]
pub enum Event {
    PositionAssigned {
        name: String,
        position: Position,
    },
    PositionRevoked {
        name: String,
        position: Position,
    },
    RoleAssigned {
        name: String,
        position: Position,
        role: Role,
    },
    RoleRevoked {
        name: String,
        position: Position,
        role: Role,
    },

    Warned {
        name: String,
        position: Position,
        total: u8,
    },
    Pardoned {
        name: String,
        position: Position,
        total: u8,
    },

    Silenced {
        name: String,
        position: Position,
    },
    Unsilenced {
        name: String,
        position: Position,
    },

    Revived {
        name: String,
        position: Position,
    },
    Removed {
        name: String,
        position: Position,
    },
    Died {
        name: String,
        position: Position,
    },
    Eliminated {
        name: String,
        position: Position,
    },
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::PositionAssigned { name, position } => {
                write!(f, "Player {name} got assigned to position {position}")
            }
            Event::PositionRevoked { name, position } => {
                write!(f, "Player {name} was unassinged from position {position}")
            }
            Event::RoleAssigned {
                name,
                position,
                role,
            } => {
                write!(
                    f,
                    "Player {name} at position {position} got the role {role}"
                )
            }
            Event::RoleRevoked {
                name,
                position,
                role,
            } => {
                write!(
                    f,
                    "Player {name} at position {position} was unassigned from role {role}"
                )
            }
            Event::Warned {
                name,
                position,
                total,
            } => {
                write!(
                    f,
                    "Player {name} at position {position} was warned, currently has {total}"
                )
            }
            Event::Pardoned {
                name,
                position,
                total,
            } => {
                write!(
                    f,
                    "Player {name} at position {position} was pardoned, currently has {total}"
                )
            }
            Event::Silenced { name, position } => {
                write!(f, "Player {name} at position {position} is Silenced")
            }
            Event::Unsilenced { name, position } => {
                write!(f, "Player {name} at position {position} is Unsilenced")
            }
            Event::Revived { name, position } => {
                write!(f, "Player {name} at position {position} is Revived")
            }
            Event::Removed { name, position } => {
                write!(f, "Player {name} at position {position} is Removed")
            }
            Event::Died { name, position } => {
                write!(f, "Player {name} at position {position} is Dead")
            }
            Event::Eliminated { name, position } => {
                write!(f, "Player {name} at position {position} is Eliminated")
            }
        }
    }
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

    pub fn assign_position(&mut self, position: Position) -> Result<Vec<Event>, Error> {
        if self.position.is_some() {
            return Err(Error::HasPosition);
        }
        self.position = Some(position);
        Ok(vec![Event::PositionAssigned {
            name: self.name.clone(),
            position,
        }])
    }

    pub fn revoke_position(&mut self) -> Result<Vec<Event>, Error> {
        let mut events = Vec::new();
        if self.position().is_some() {
            events.push(Event::PositionRevoked {
                name: self.name.clone(),
                position: self.position().unwrap(),
            });
            self.position = None;
        }
        Ok(events)
    }

    /// Assign a role (immutable once set)
    pub fn assign_role(&mut self, role: Role) -> Result<Vec<Event>, Error> {
        if self.role.is_some() {
            return Err(Error::HasRole);
        }
        self.role = Some(role);
        Ok(vec![Event::RoleAssigned {
            name: self.name.clone(),
            position: self.position.unwrap(),
            role,
        }])
    }

    pub fn revoke_role(&mut self) -> Result<Vec<Event>, Error> {
        let mut events = Vec::new();
        if self.role().is_some() {
            events.push(Event::RoleRevoked {
                name: self.name.clone(),
                position: self.position.unwrap(),
                role: self.role.unwrap(),
            });
            self.role = None;
        }
        Ok(events)
    }

    // ----------- Warnings ----------
    pub fn warn(&mut self) -> Result<Vec<Event>, Error> {
        if self.status == Status::Removed {
            return Err(Error::MaxWarningsReached);
        }

        let mut events = Vec::new();
        events.push(Event::Warned {
            name: self.name.clone(),
            position: self.position.unwrap(),
            total: self.warnings,
        });

        self.warnings += 1;

        match self.warnings {
            3 => {
                self.penalty.silenced = true;
                events.push(Event::Silenced {
                    name: self.name.clone(),
                    position: self.position.unwrap(),
                });
            }
            4 => {
                self.mark_removed()?;
            }
            _ => {}
        }

        Ok(events)
    }

    pub fn pardon(&mut self) -> Result<Vec<Event>, Error> {
        if self.warnings == 0 {
            return Err(Error::NoWarnings);
        }

        let mut events = Vec::new();

        self.warnings -= 1;
        events.push(Event::Pardoned {
            name: self.name.clone(),
            position: self.position.unwrap(),
            total: self.warnings,
        });

        if self.warnings < 3 && self.penalty.silenced {
            self.penalty.silenced = false;
            events.push(Event::Unsilenced {
                name: self.name.clone(),
                position: self.position.unwrap(),
            });
        }

        if self.status == Status::Removed && self.warnings < 4 {
            self.restore_alive()?;
        }

        Ok(events)
    }

    // ----------- Life transitions ----------
    pub fn mark_dead(&mut self) -> Result<Vec<Event>, Error> {
        self.status = Status::Dead;
        Ok(vec![Event::Died {
            name: self.name.clone(),
            position: self.position.unwrap(),
        }])
    }
    pub fn mark_eliminated(&mut self) -> Result<Vec<Event>, Error> {
        self.status = Status::Eliminated;

        Ok(vec![Event::Eliminated {
            name: self.name.clone(),
            position: self.position.unwrap(),
        }])
    }
    pub fn mark_removed(&mut self) -> Result<Vec<Event>, Error> {
        self.status = Status::Removed;
        self.penalty.silenced = false;
        Ok(vec![Event::Removed {
            name: self.name.clone(),
            position: self.position.unwrap(),
        }])
    }
    pub fn restore_alive(&mut self) -> Result<Vec<Event>, Error> {
        self.status = Status::Alive;
        Ok(vec![Event::Revived {
            name: self.name.clone(),
            position: self.position.unwrap(),
        }])
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
    // pub fn status(&self) -> Status {
    //     self.status
    // }
    // pub fn warnings(&self) -> u8 {
    //     self.warnings
    // }
    // pub fn is_silenced(&self) -> bool {
    //     self.penalty.silenced
    // }

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
    // pub fn is_removed(&self) -> bool {
    //     self.status == Status::Removed
    // }
    pub fn is_eliminated(&self) -> bool {
        self.status == Status::Eliminated
    }

    // pub fn is_mafia(&self) -> bool {
    //     matches!(self.role, Some(Role::Don) | Some(Role::Mafia))
    // }

    pub fn is_sheriff(&self) -> bool {
        matches!(self.role, Some(Role::Sheriff))
    }

    pub fn is_don(&self) -> bool {
        matches!(self.role, Some(Role::Don))
    }
}
