use super::actor::Actor;
use crate::domain::position::Position;

use crate::domain::{DayPhase, NightPhase, Phase, RoundId, VotingPhase};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnContext {
    RoleAssignment,
    DayDiscussion,
    VotingDiscussion,
    VoteCasting,
    FinalSpeech(Position), // Single actor turn
    SheriffCheck(Position),
    DonCheck(Position),
}

pub trait Turn {
    /// Advance the actor to the next eligible Chair
    fn next_actor<F>(&self, actor: &mut Actor, is_eligible: F) -> Option<Position>
    where
        F: Fn(Position) -> bool;

    fn turn_context(&self, round: RoundId, phase: Phase, actor: &Actor) -> Option<TurnContext>;
}
