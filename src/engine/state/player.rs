use crate::engine::state::role::Role;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Status {
    #[default]
    Alive,
    Killed,
    Eliminated,
    Removed,
}

#[derive(Debug, Default, Clone)]
pub struct WarningPenalty {
    pending_silence: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub name: String,
    pub role: Role,
    pub warnings: u8,
    pub penalty: WarningPenalty,
    pub status: Status,
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Player {
            name,
            role,
            warnings: 0,
            penalty: WarningPenalty::default(),
            status: Status::Alive,
        }
    }
    pub fn add_warning(&mut self) {
        self.warnings += 1;
        if self.warnings == 4 {
            self.status = Status::Removed;
        }

        if self.warnings == 3 {
            self.penalty.pending_silence = true;
        }
    }

    pub fn reset_warnings(&mut self) {
        self.warnings = 0;
    }

    pub fn remove_warning(&mut self) {
        if self.warnings > 0 {
            self.warnings -= 1;
        }
    }
}
