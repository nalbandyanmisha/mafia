#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Copy, PartialOrd, Ord)]
pub struct Chair {
    pub position: u8,
}

impl Chair {
    pub fn new(position: u8) -> Self {
        Chair { position }
    }
}
