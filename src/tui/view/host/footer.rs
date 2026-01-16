#[derive(Debug, Clone)]
pub struct Footer {
    pub commands: String, // list of host commands
}

impl Footer {
    pub fn new(commands: &[&str]) -> Self {
        Self {
            commands: commands.join(" | "),
        }
    }
}
