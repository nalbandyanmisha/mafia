pub mod footer;
pub mod header;
pub mod main;

pub use footer::Footer;
pub use header::Header;
use main::Actor;
pub use main::Main;

use crate::domain::{
    Activity, Day, EveningActivity, MorningActivity, NightActivity, NoonActivity, Status,
};
use crate::snapshot;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct HostView {
    pub title: String, // 🌙 NIGHT · Day 2
    pub title_style: Color,
    pub header: Header, // activity name + phase info (nominees/checks)
    pub main: Main,     // active actor / instructions + timer + results
    pub footer: Footer, // list of host commands
}

impl HostView {
    pub fn from_snapshot(app: &snapshot::App) -> Self {
        use Activity::*;
        use EveningActivity::*;
        use MorningActivity::*;
        use NightActivity::*;
        use NoonActivity::*;
        let engine = &app.engine;
        let phase = engine.phase.expect("phase must exist");

        let (title, title_style) = match phase.daytime() {
            Day::Night => (format!("🌙 Night · {}", engine.day), Color::Magenta),
            Day::Morning => (format!("☀ Morning · {}", engine.day), Color::Cyan),
            Day::Noon => (format!("☀ Day · {}", engine.day), Color::Yellow),
            Day::Evening => (format!("🌆 Evening · {}", engine.day), Color::Blue),
        };

        let (activity, activity_info) = match phase {
            Night(activity) => match activity {
                RoleAssignment => {
                    let player_left_to_be_assinged_role = engine
                        .game
                        .players
                        .iter()
                        .fold(0, |c, p| if p.role.is_none() { c + 1 } else { c });
                    (
                        activity.to_string(),
                        format!("Players left: {player_left_to_be_assinged_role}"),
                    )
                }
                SheriffReveal => todo!(),
                DonReveal => todo!(),
                MafiaBriefing => todo!(),
                MafiaShooting => todo!(),
                SheriffCheck => todo!(),
                DonCheck => todo!(),
            },

            Morning(activity) => match activity {
                Guessing => todo!(),
                DeathSpeech => todo!(),
            },

            Noon(activity) => match activity {
                Discussion => todo!(),
            },

            Evening(activity) => match activity {
                NominationAnnouncement => todo!(),
                Voting => todo!(),
                TieDiscussion => todo!(),
                TieVoting => todo!(),
                FinalVoting => todo!(),
                FinalSpeech => todo!(),
            },
        };

        let in_p_c = engine
            .game
            .players
            .iter()
            .fold(0, |c, p| if p.status == Status::Alive { c + 1 } else { c });
        let out_p_c = 10 - in_p_c;

        let main = if let Some(actor) = engine.actor {
            Main::Actor(Actor {
                position: actor,
                instructions: "instructions".to_string(),
                timer: Some(30),
                result: Some("result".to_string()),
            })
        } else {
            Main::Description("Description".to_string())
        };

        Self {
            title,
            title_style,
            header: Header::new(in_p_c, out_p_c, activity, Some(activity_info)),
            main,
            footer: Footer::new(&["next", "warn"]),
        }
    }
}
