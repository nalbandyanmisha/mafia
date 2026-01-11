use crate::domain::{Activity, Day, EveningActivity, MorningActivity, NightActivity, NoonActivity};
use crate::snapshot;
use crate::tui::view::host_narration::build_host_narration;
use ratatui::style::{Color, Modifier, Style};

use super::HostNarration;

#[derive(Debug, Clone)]
pub struct HostView {
    pub narration: HostNarration,
    pub title: String,
    pub title_style: Style,
    pub header: HostHeader,
    pub main: HostMain,
    pub footer: HostFooter,
}

#[derive(Debug, Clone)]
pub struct HostHeader {
    pub text: String,
    pub style: Style,
}

#[derive(Debug, Clone)]
pub struct HostMain {
    pub title: String,
    pub subtitle: Option<String>,
    pub highlight_actor: bool,
}

#[derive(Debug, Clone)]
pub struct HostFooter {
    pub text: String,
    pub style: Style,
}

fn format_votes(voting: &snapshot::Voting) -> String {
    let mut rows: Vec<_> = voting.votes.iter().collect();
    rows.sort_by_key(|(nominee, _)| nominee.value());

    rows.iter()
        .map(|(nominee, voters)| format!("🪑{}: {}", nominee.value(), voters.len()))
        .collect::<Vec<_>>()
        .join("  ")
}

fn format_votes_verbose(voting: &snapshot::Voting) -> String {
    let mut rows: Vec<_> = voting.votes.iter().collect();
    rows.sort_by_key(|(nominee, _)| nominee.value());

    rows.into_iter()
        .map(|(nominee, voters)| {
            let mut voters = voters.clone();
            voters.sort_by_key(|p| p.value());

            let voters_str = voters
                .iter()
                .map(|p| format!("🪑{}", p.value()))
                .collect::<Vec<_>>()
                .join(", ");

            format!("🪑{} ← {} ({})", nominee.value(), voters.len(), voters_str)
        })
        .collect::<Vec<_>>()
        .join("  |  ")
}

impl HostView {
    pub fn from_snapshot(app: &snapshot::App) -> Self {
        use Day::*;
        let narration = build_host_narration(app);
        let engine = &app.engine;
        let phase = engine.phase.expect("phase must exist");

        let (title, title_style) = match phase.daytime() {
            Night => (format!("🌙 Night · {}", engine.day), Color::Magenta),
            Morning => (format!("☀ Morning · {}", engine.day), Color::Cyan),
            Noon => (format!("☀ Day · {}", engine.day), Color::Yellow),
            Evening => (format!("🌆 Evening · {}", engine.day), Color::Blue),
        };

        Self {
            narration,
            title,
            title_style: Style::default().fg(title_style),
            header: build_header(phase),
            main: build_main(app),
            footer: build_footer(app),
        }
    }
}

/* ---------------- Header ---------------- */

fn build_header(activity: Activity) -> HostHeader {
    use Activity::*;
    use EveningActivity::*;
    use MorningActivity::*;
    use NightActivity::*;
    use NoonActivity::*;

    let (text, color) = match activity {
        Night(a) => match a {
            RoleAssignment => ("Role assignment", Color::Magenta),
            SheriffReveal => ("Sheriff reveal", Color::Magenta),
            DonReveal => ("Don reveal", Color::Magenta),
            MafiaBriefing => ("Mafia briefing", Color::Magenta),
            MafiaShooting => ("Mafia shooting", Color::Magenta),
            SheriffCheck => ("Sheriff check", Color::Magenta),
            DonCheck => ("Don check", Color::Magenta),
        },

        Morning(a) => match a {
            DeathSpeech => ("Death speech", Color::Cyan),
            Guessing => ("Guessing", Color::Cyan),
        },

        Noon(Discussion) => ("Discussion", Color::Yellow),

        Evening(a) => match a {
            NominationAnnouncement => ("Nominations", Color::Blue),
            Voting => ("Voting", Color::Blue),
            TieDiscussion => ("Tie discussion", Color::Blue),
            TieVoting => ("Tie voting", Color::Blue),
            FinalVoting => ("Final voting", Color::Blue),
            FinalSpeech => ("Final speech", Color::Blue),
        },
    };

    HostHeader {
        text: text.into(),
        style: Style::default().fg(color),
    }
}

/* ---------------- Main ---------------- */

fn build_main(app: &snapshot::App) -> HostMain {
    use Activity::*;
    use EveningActivity::*;
    use MorningActivity::*;
    use NightActivity::*;
    use NoonActivity::*;

    let engine = &app.engine;
    let phase = engine.phase.expect("phase must exist");

    match phase {
        Night(a) => match a {
            MafiaShooting => HostMain {
                title: "Mafia is choosing a victim".into(),
                subtitle: engine
                    .game
                    .kill
                    .get(&engine.day)
                    .map(|c| format!("🎯 Chair {} was killed", c.value())),
                highlight_actor: true,
            },

            SheriffCheck => HostMain {
                title: "Sheriff is checking".into(),
                subtitle: engine
                    .game
                    .check
                    .get(&engine.day)
                    .and_then(|c| c.sheriff)
                    .map(|p| {
                        format!(
                            "🔍 Sheriff checked player at position {} and found",
                            p.value(),
                        )
                    }),
                highlight_actor: true,
            },

            DonCheck => HostMain {
                title: "Don is checking".into(),
                subtitle: engine
                    .game
                    .check
                    .get(&engine.day)
                    .and_then(|c| c.don)
                    .map(|p| format!("🕵️ Checking Chair {}", p.value())),
                highlight_actor: true,
            },

            _ => HostMain {
                title: "Night phase".into(),
                subtitle: None,
                highlight_actor: false,
            },
        },

        Morning(a) => match a {
            DeathSpeech => HostMain {
                title: "Final words".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },

            Guessing => HostMain {
                title: "Guess mafia".into(),
                subtitle: engine.actor.map(|p| format!("🎯 Chair {}", p.value())),
                highlight_actor: true,
            },
        },

        Noon(Discussion) => HostMain {
            title: "Discussion".into(),
            subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
            highlight_actor: true,
        },

        Evening(a) => match a {
            NominationAnnouncement => {
                let nominees = engine.game.voting.get(&engine.day).map(|v| {
                    v.nominees
                        .iter()
                        .map(|p| format!("🪑{}", p.value()))
                        .collect::<Vec<_>>()
                        .join(", ")
                });

                HostMain {
                    title: "Nominations".into(),
                    subtitle: nominees.map(|n| format!("Nominated: {n}")),
                    highlight_actor: false,
                }
            }

            Voting => {
                let voting = engine
                    .game
                    .voting
                    .get(&engine.day)
                    .cloned()
                    .unwrap_or_else(snapshot::Voting::default);

                HostMain {
                    title: "CAST YOUR VOTE".into(),
                    subtitle: Some(format_votes_verbose(&voting)),
                    highlight_actor: true,
                }
            }

            TieDiscussion => HostMain {
                title: "Tie discussion".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },

            TieVoting => {
                let voting = engine
                    .game
                    .tie_voting
                    .get(&engine.day)
                    .cloned()
                    .unwrap_or_else(snapshot::Voting::default);
                HostMain {
                    title: "CAST YOUR VOTE".into(),
                    subtitle: Some(format_votes_verbose(&voting)),
                    highlight_actor: true,
                }
            }

            FinalVoting => {
                let final_votes = engine
                    .game
                    .final_voting
                    .get(&engine.day)
                    .cloned()
                    .unwrap_or_default();

                let votes_str = final_votes
                    .iter()
                    .map(|p| format!("🪑{}", p.value()))
                    .collect::<Vec<_>>()
                    .join(", ");

                HostMain {
                    title: "FINAL VOTE".into(),
                    subtitle: Some(format!("Votes: {votes_str}")),
                    highlight_actor: true,
                }
            }

            FinalSpeech => HostMain {
                title: "Final speech".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },
        },
    }
}

/* ---------------- Footer ---------------- */

fn build_footer(app: &snapshot::App) -> HostFooter {
    match app.current_timer {
        Some(sec) => {
            let style = if sec <= 10 {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            HostFooter {
                text: format!("⏳ {:02}:{:02}", sec / 60, sec % 60),
                style,
            }
        }
        None => HostFooter {
            text: "NO TIMER".into(),
            style: Style::default().fg(Color::Gray),
        },
    }
}
