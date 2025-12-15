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
pub struct Player {
    pub name: String,
    pub role: Role,
    pub warnings: u8,
    pub status: Status,
    // index acts as a round in the game
    pub is_nominee: Vec<bool>,
    pub nominated: Vec<Option<u8>>,
}

impl Player {
    pub fn new(name: String, role: Role) -> Self {
        Player {
            name,
            role,
            warnings: 0,
            status: Status::Alive,
            is_nominee: vec![],
            nominated: vec![],
        }
    }
    pub fn add_warning(&mut self) {
        self.warnings += 1;
        if self.warnings == 4 {
            self.status = Status::Removed;
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

    pub fn withdraw(&mut self) {
        self.nominated.pop();
    }

    pub fn nominate(&mut self, position: Option<u8>) {
        self.nominated.push(position);
    }
}
