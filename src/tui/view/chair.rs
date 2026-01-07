use crate::domain::{Position, Status};
use crate::tui::view::PlayerView;

#[derive(Debug, Clone)]
pub struct ChairView {
    pub position: Position,
    pub state: ChairState,
    pub player: Option<PlayerView>,
    pub highlight: bool,
}

#[derive(Debug, Clone)]
pub enum ChairState {
    Empty,
    Alive,
    Dead,
    Eliminated,
    Removed,

    Speaking,
    Muted,
    Candidate,
}

impl ChairView {
    pub fn from_snapshot(position: Position, app: &crate::snapshot::App) -> Self {
        let player = app
            .engine
            .game
            .players
            .iter()
            .find(|p| p.position == Some(position));

        let player_view = player.map(|_| PlayerView::from_snapshot(position, app));

        let state = match &player_view {
            None => ChairState::Empty,
            Some(view) => match view.status {
                Status::Alive => ChairState::Alive,
                Status::Dead => ChairState::Dead,
                Status::Eliminated => ChairState::Eliminated,
                Status::Removed => ChairState::Removed,
            },
        };

        let highlight = app.engine.actor == Some(position);

        Self {
            position,
            state,
            player: player_view,
            highlight,
        }
    }
}
