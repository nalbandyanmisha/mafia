#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Role {
    #[default]
    Citizen,
    Mafia,
    Don,
    Sheriff,
}
