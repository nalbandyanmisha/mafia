#[derive(Debug, Clone)]
pub struct Header {
    pub in_players: usize,
    pub out_players: usize,
    pub activity: String, // current activity, e.g. Discussion
}

impl Header {
    pub fn new(in_players: usize, out_players: usize, activity: String) -> Self {
        Self {
            in_players,
            out_players,
            activity,
        }
    }
}
