use crate::{
    domain::{Activity, NightActivity, Position, Role},
    snapshot,
};

/// HostNarration structure
#[derive(Debug, Clone)]
pub struct HostNarration {
    pub title: String,
    pub body: Vec<String>,
}

/// Build narration for the host panel directly from snapshot::App
pub fn build_host_narration(app: &snapshot::App) -> HostNarration {
    // Collect all active chairs
    let active_players: Vec<_> = app
        .engine
        .game
        .players
        .iter()
        .filter(|c| c.position == app.engine.actor)
        .collect();
    let mafia: Vec<_> = app
        .engine
        .game
        .players
        .iter()
        .filter(|p| {
            p.role.is_some_and(|r| r == Role::Mafia) || p.role.is_some_and(|r| r == Role::Don)
        })
        .collect();
    use Activity::*;
    use NightActivity::*;
    match app.engine.phase.expect("phase should exist") {
        Night(RoleAssignment) => {
            if let Some(player) = active_players.first() {
                match player.role {
                    Some(role) => HostNarration {
                        title: "Role Assigned".into(),
                        body: vec![format!(
                            "Chair {} has been assigned role '{}'.",
                            player.position.expect("Player must have position"),
                            role
                        )],
                    },
                    None => HostNarration {
                        title: "Role Assignment".into(),
                        body: vec![format!(
                            "Chair {} is active, awaiting role.",
                            player.position.expect("Player must have position")
                        )],
                    },
                }
            } else {
                HostNarration {
                    title: "Role Assignment".into(),
                    body: vec!["No chairs active.".into()],
                }
            }
        }

        Night(SheriffReveal) => {
            if let Some(player) = active_players.first() {
                HostNarration {
                    title: "Sheriff Reveal".into(),
                    body: vec![format!(
                        "Chair {} is revealed as Sheriff.",
                        player.position.expect("Player must have position")
                    )],
                }
            } else {
                HostNarration {
                    title: "Sheriff Reveal".into(),
                    body: vec!["No Sheriff active.".into()],
                }
            }
        }

        Night(DonReveal) => {
            if let Some(chair) = active_players.first() {
                HostNarration {
                    title: "Don Reveal".into(),
                    body: vec![format!(
                        "Chair {} is revealed as Don.",
                        chair.position.expect("Player must have position")
                    )],
                }
            } else {
                HostNarration {
                    title: "Don Reveal".into(),
                    body: vec!["No Don active.".into()],
                }
            }
        }

        Night(MafiaBriefing) => {
            if !mafia.is_empty() {
                let positions: Vec<_> = mafia.iter().map(|c| c.position).collect();
                HostNarration {
                    title: "Mafia Briefing".into(),
                    body: vec![format!(
                        "Mafia Chairs: {:?}",
                        format_positions(positions.as_slice())
                    )],
                }
            } else {
                HostNarration {
                    title: "Mafia Briefing".into(),
                    body: vec!["No Mafia active.".into()],
                }
            }
        }

        // Default fallback for other activities
        _ => HostNarration {
            title: "Waiting…".into(),
            body: vec!["No active activity.".into()],
        },
    }
}

fn format_positions(positions: &[Option<Position>]) -> String {
    positions
        .iter()
        .map(|p| p.unwrap().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
