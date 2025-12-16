use super::table::chair::Chair;

#[derive(Debug, Clone)]
pub struct Nomination {
    pub by: Chair,
    pub target: Chair,
}
