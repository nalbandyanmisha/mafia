use super::table::chair::Chair;
#[derive(Debug, Clone)]
pub struct Vote {
    pub voter: Chair,
    pub target: Chair,
}
