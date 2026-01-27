use ratatui::style::Color;

use crate::domain::{Day, Position, Status};
use crate::tui::view::PlayerView;

#[derive(Debug, Clone)]
pub struct ChairView {
    pub position: Position,
    pub state: ChairState,
    pub player: Option<PlayerView>,
    pub highlight: bool,
    pub border_style: Color,
}

#[derive(Debug, Clone)]
pub enum ChairState {
    Empty,
    Alive,
    Dead,
    Eliminated,
    Removed,

    RoleAssignment,
    Speaking,
    Muted,
    Candidate,
}

impl ChairView {
    pub fn from_snapshot(position: Position, app: &crate::snapshot::App) -> Self {
        use Day::*;
        let player = app
            .engine
            .game
            .players
            .iter()
            .find(|p| p.position == Some(position));

        let border_style = match app.engine.phase.unwrap().daytime() {
            Night => Color::Magenta,
            Morning => Color::Cyan,
            Noon => Color::Yellow,
            Evening => Color::Blue,
        };

        let player_view = player.map(|_| PlayerView::from_snapshot(position, app));

        let state = match &player_view {
            None => ChairState::Empty,
            Some(view) => {
                // 1️⃣ Terminal states FIRST
                match view.status {
                    Status::Dead => ChairState::Dead,
                    Status::Eliminated => ChairState::Eliminated,
                    Status::Removed => ChairState::Removed,
                    Status::Alive => {
                        // 2️⃣ Warnings / silence
                        if view.warnings == 3 && view.is_silenced {
                            ChairState::Muted
                        }
                        // 3️⃣ Active player states
                        else if app.engine.actor == Some(position) {
                            match app.engine.phase.unwrap().daytime() {
                                Night => ChairState::RoleAssignment,
                                Morning | Noon => ChairState::Speaking,
                                Evening => ChairState::Candidate,
                            }
                        }
                        // 4️⃣ Default alive
                        else {
                            ChairState::Alive
                        }
                    }
                }
            }
        };

        let highlight = app.engine.actor == Some(position);

        Self {
            position,
            state,
            player: player_view,
            highlight,
            border_style,
        }
    }
}
