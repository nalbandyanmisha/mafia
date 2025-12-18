use super::super::table::chair::Chair;
#[derive(Debug, Clone)]
pub struct Vote {
    voter: Chair,
    nominee: Chair,
}

impl Vote {
    pub fn new(voter: Chair, nominee: Chair) -> Self {
        Vote { voter, nominee }
    }

    pub fn voter(&self) -> Chair {
        self.voter
    }

    pub fn nominee(&self) -> Chair {
        self.nominee
    }
}
