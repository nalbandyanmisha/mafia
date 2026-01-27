#[derive(Debug, Clone)]
pub struct Actor {
    pub actor: String,          // who is acting now
    pub timer: Option<u64>,     // time left for actor
    pub result: Option<String>, // results of the activity
}

impl Actor {
    pub fn new(actor: String, timer: Option<u64>, result: Option<String>) -> Self {
        Self {
            actor,
            timer,
            result,
        }
    }
}
