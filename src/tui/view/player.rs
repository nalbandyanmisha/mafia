// use ratatui::{
//     text::{Line, Text},
//     widgets::Paragraph,
// };
//
// use super::View;
use crate::domain::{position::Position, role::Role, status::Status};
use crate::snapshot::{App, Player as SnapPlayer};
#[derive(Debug, Clone)]
pub struct PlayerView {
    pub name: String,
    pub position: Option<Position>,
    pub role: Option<Role>, // maybe hide if secret
    pub warnings: u8,
    pub is_silenced: bool,
    pub status: Status,

    // UI / round derived info
    pub is_current_speaker: bool,
    pub is_nominated: bool,
    pub who_nominated: Option<Position>,
    pub voted_for: Option<Position>,
    pub votes_received: Vec<Position>,
}

// impl View for PlayerView {
//     type Widget = Paragraph<'static>;
//     fn widget(&self) -> Self::Widget {
//         let name = Line::from(self.name.clone());
//         let position = Line::from(
//             self.position
//                 .as_ref()
//                 .map_or("Unknown".to_string(), |p| p.to_string()),
//         );
//         let role = Line::from(
//             self.role
//                 .as_ref()
//                 .map_or("??????".to_string(), |role| role.to_string()),
//         );
//         let warnings = Line::from(format!("Warnings: {}", self.warnings));
//         let status = Line::from(format!("Status: {}", self.status));
//         let is_silenced = Line::from(format!("Silenced: {}", self.is_silenced));
//         let is_nominated = Line::from(format!("Nominated: {}", self.is_nominated));
//         let who_nominated = Line::from(format!(
//             "Nominated by: {}",
//             self.who_nominated
//                 .as_ref()
//                 .map_or("N/A".to_string(), |p| p.to_string())
//         ));
//         let voted_for = Line::from(format!(
//             "Voted for: {}",
//             self.voted_for
//                 .as_ref()
//                 .map_or("N/A".to_string(), |p| p.to_string())
//         ));
//         let votes_received = Line::from(format!(
//             "Votes received: {}",
//             if self.votes_received.is_empty() {
//                 "None".to_string()
//             } else {
//                 self.votes_received
//                     .iter()
//                     .map(|p| p.to_string())
//                     .collect::<Vec<String>>()
//                     .join(", ")
//             }
//         ));
//         let mut player = Text::default();
//         player.extend(vec![
//             name,
//             position,
//             role,
//             warnings,
//             status,
//             is_silenced,
//             is_nominated,
//             who_nominated,
//             voted_for,
//             votes_received,
//         ]);
//
//         Paragraph::new(player)
//     }
// }

impl PlayerView {
    pub fn from_snapshot(player: &SnapPlayer, app: &App) -> Self {
        let pos = player.position;

        let round = &app.engine.game.round;

        let is_current_speaker = app.engine.actor == pos;

        let (is_nominated, who_nominated, votes_received, voted_for) = if let Some(p) = pos {
            (
                round.is_nominated(p),
                round.nominated_by(p),
                round.votes_received(p),
                round.voted_for(p),
            )
        } else {
            (false, None, vec![], None)
        };

        Self {
            name: player.name.clone(),
            position: player.position,
            role: player.role,
            warnings: player.warnings,
            is_silenced: player.is_silenced,
            status: player.status,

            is_current_speaker,
            is_nominated,
            who_nominated,
            voted_for,
            votes_received,
        }
    }
}
