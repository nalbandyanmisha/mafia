use super::super::table::chair::Chair;

#[derive(Debug, Clone)]
pub struct Nomination {
    nominator: Chair,
    nominee: Chair,
}

impl Nomination {
    pub fn new(nominator: Chair, nominee: Chair) -> Self {
        Nomination { nominator, nominee }
    }

    pub fn nominator(&self) -> Chair {
        self.nominator
    }

    pub fn nominee(&self) -> Chair {
        self.nominee
    }
}
