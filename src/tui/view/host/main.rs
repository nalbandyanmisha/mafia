mod actor;
pub use actor::Actor;

#[derive(Debug, Clone)]
pub enum Main {
    Actor(Actor),
    Description(String),
}
