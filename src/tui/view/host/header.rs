use crate::{domain::Activity, snapshot};

#[derive(Debug, Clone)]
pub struct Header {
    pub in_players: usize,
    pub out_players: usize,
    pub activity: String,              // current activity, e.g. Discussion
    pub activity_info: Option<String>, // optional second line: nominees, checks
}

impl Header {
    pub fn new(
        in_players: usize,
        out_players: usize,
        activity: String,
        activity_info: Option<String>,
    ) -> Self {
        Self {
            in_players,
            out_players,
            activity,
            activity_info,
        }
    }
}
