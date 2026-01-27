#[derive(Debug, Clone)]
pub struct Footer {
    pub info: String, // list of host commands
}

impl Footer {
    pub fn new(info: &str) -> Self {
        Self {
            info: info.to_string(),
        }
    }
}
