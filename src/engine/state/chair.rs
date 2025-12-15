use std::fmt::Display;
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
pub struct Chair {
    pub position: u8,
}

impl Chair {
    pub fn new(position: u8) -> Self {
        Chair { position }
    }
}

impl From<u8> for Chair {
    fn from(position: u8) -> Self {
        Chair { position }
    }
}

impl From<&str> for Chair {
    fn from(s: &str) -> Self {
        let position = s.parse::<u8>().unwrap_or(1);
        Chair { position }
    }
}

impl Display for Chair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)
    }
}
