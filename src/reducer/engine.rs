use crate::action::engine::EngineAction;
use crate::domain::{EngineState, LobbyStatus};
use crate::engine::Engine;
use crate::engine::EngineEvent;
use crate::engine::game;
use anyhow::Error;

pub fn reduce(state: &mut Engine, action: EngineAction) -> Result<Vec<EngineEvent>, Error> {
    match action {
        EngineAction::Join { name } => join(state, &name),
        EngineAction::Leave { name } => leave(state, &name),
    }
}

fn join(state: &mut Engine, name: &str) -> Result<Vec<EngineEvent>, Error> {
    // TODO: ensure lobby state
    let mut events = Vec::new();
    events.extend(state.game.add_player(name)?);

    Ok(events.into_iter().map(EngineEvent::Game).collect())
}

fn leave(state: &mut Engine, name: &str) -> Result<Vec<EngineEvent>, Error> {
    // TODO: ensure lobby state
    let mut events = Vec::new();
    events.extend(revoke_position(state, name)?);

    state.game.remove_player(name)?;

    if state.state == EngineState::Lobby(LobbyStatus::Ready)
        && state.game.available_positions().len() == 1
    {
        state.state = EngineState::Lobby(LobbyStatus::Waiting);
    }

    Ok(vec![EngineEvent::Game(game::Event::PlayerLeft {
        name: name.to_string(),
    })])
}

fn revoke_position(state: &mut Engine, name: &str) -> Result<Vec<EngineEvent>, Error> {
    // TODO: ensure lobby state
    let position = state
        .game
        .player_by_name(name)
        .ok_or_else(|| anyhow::anyhow!("Player not found"))?
        .position()
        .ok_or_else(|| anyhow::anyhow!("Player {name} does not have an assigned position"))?;

    let player = state
        .game
        .player_by_name_mut(name)
        .ok_or_else(|| anyhow::anyhow!("Player not found"))?;

    let events = player.revoke_position()?;
    state.game.return_position(position);
    Ok(events
        .into_iter()
        .map(game::Event::Player)
        .map(EngineEvent::Game)
        .collect())
}
