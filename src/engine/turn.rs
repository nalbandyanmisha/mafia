use super::actor::Actor;
use crate::domain::position::Position;

pub trait Turn {
    /// Advance the actor to the next eligible Chair
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Position>
    where
        F: Fn(Position) -> bool;
}
