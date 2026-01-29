use ratatui::style::Color;

use crate::domain::{Activity, Day, EveningActivity, NightActivity, Position, Role, Status};
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

        let phase = app.engine.phase.expect("Phase must exist");

        let mut highlight = false;
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

        // Active actor is always highlighted
        if app.engine.actor == Some(position) {
            highlight = true;
        }

        // Mafia briefing: highlight all mafia
        let is_mafia_briefing = matches!(phase, Activity::Night(NightActivity::MafiaBriefing));
        let is_final_voting = matches!(phase, Activity::Evening(EveningActivity::FinalVoting));

        if is_mafia_briefing && app.engine.actor.is_some() {
            if let Some(view) = &player_view {
                if view.role.expect("Role must exist") == Role::Mafia {
                    highlight = true;
                }
            }
        }

        if is_final_voting {
            if let Some(view) = &player_view {
                if app
                    .engine
                    .game
                    .tie_voting
                    .get(&app.engine.day)
                    .map_or(false, |voting| voting.nominees.contains(&position))
                {
                    highlight = true;
                }
            }
        }

        Self {
            position,
            state,
            player: player_view,
            highlight,
            border_style,
        }
    }
}
