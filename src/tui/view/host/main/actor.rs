use crate::domain::Position;

#[derive(Debug, Clone)]
pub struct Actor {
    pub position: Position,     // who is acting now
    pub instructions: String,   // host instructions if no active actor
    pub timer: Option<u16>,     // time left for actor
    pub result: Option<String>, // results of the activity
}

impl Actor {
    pub fn new(
        position: Position,
        instructions: String,
        timer: Option<u16>,
        result: Option<String>,
    ) -> Self {
        Self {
            position,
            instructions,
            timer,
            result,
        }
    }
}
