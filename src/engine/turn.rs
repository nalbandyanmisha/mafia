use super::state::{actor::Actor, table::chair::Chair};

pub trait Turn {
    /// Advance the actor to the next eligible Chair
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Chair>
    where
        F: Fn(Chair) -> bool;
}
