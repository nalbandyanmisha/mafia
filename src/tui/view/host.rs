use crate::{
    domain::{Activity, DayActivity, EveningActivity, MorningActivity, NightActivity, Time},
    snapshot::{self, App, Check},
};
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct HostView {
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
    pub fn from_snapshot(app: &App) -> Self {
        let engine = &app.engine;
        let phase = engine.phase.expect("phase must exist");

        let (title, title_style) = match phase.time() {
            Time::Night => (format!("🌙 Night · {}", engine.round), Color::Magenta),
            Time::Morning => (format!("☀ Morning · {}", engine.round), Color::Cyan),
            Time::Day => (format!("☀ Day · {}", engine.round), Color::Yellow),
            Time::Evening => (format!("🌆 Evening · {}", engine.round), Color::Blue),
        };

        Self {
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

    let (text, color) = match activity {
        Night(a) => match a {
            NightActivity::RoleAssignment => ("Role assignment", Color::Magenta),
            NightActivity::SheriffReveal => ("Sheriff reveal", Color::Magenta),
            NightActivity::DonReveal => ("Don reveal", Color::Magenta),
            NightActivity::MafiaBriefing => ("Mafia briefing", Color::Magenta),
            NightActivity::MafiaShooting => ("Mafia shooting", Color::Magenta),
            NightActivity::SheriffCheck => ("Sheriff check", Color::Magenta),
            NightActivity::DonCheck => ("Don check", Color::Magenta),
        },

        Morning(a) => match a {
            MorningActivity::FinalSpeech => ("Final speech", Color::Cyan),
            MorningActivity::Guessing => ("Guessing", Color::Cyan),
        },

        Day(DayActivity::Discussion) => ("Discussion", Color::Yellow),

        Evening(a) => match a {
            EveningActivity::NominationAnnouncement => ("Nominations", Color::Blue),
            EveningActivity::Voting => ("Voting", Color::Blue),
            EveningActivity::TieDiscussion => ("Tie discussion", Color::Blue),
            EveningActivity::TieVoting => ("Tie voting", Color::Blue),
            EveningActivity::FinalVoting => ("Final voting", Color::Blue),
            EveningActivity::FinalSpeech => ("Final speech", Color::Blue),
        },
    };

    HostHeader {
        text: text.into(),
        style: Style::default().fg(color),
    }
}

/* ---------------- Main ---------------- */

fn build_main(app: &App) -> HostMain {
    use Activity::*;

    let engine = &app.engine;
    let phase = engine.phase.unwrap();

    match phase {
        Night(a) => match a {
            NightActivity::MafiaShooting => HostMain {
                title: "Mafia is choosing a victim".into(),
                subtitle: engine
                    .game
                    .kill
                    .get(&engine.round)
                    .map(|c| format!("🎯 Chair {}", c.value())),
                highlight_actor: true,
            },

            NightActivity::SheriffCheck => HostMain {
                title: "Sheriff is checking".into(),
                subtitle: engine
                    .game
                    .check
                    .get(&engine.round)
                    .and_then(|c| c.sheriff)
                    .map(|p| format!("🔍 Checking Chair {}", p.value())),
                highlight_actor: true,
            },

            NightActivity::DonCheck => HostMain {
                title: "Don is checking".into(),
                subtitle: engine
                    .game
                    .check
                    .get(&engine.round)
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
            MorningActivity::FinalSpeech => HostMain {
                title: "Final words".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },

            MorningActivity::Guessing => HostMain {
                title: "Guess mafia".into(),
                subtitle: engine.actor.map(|p| format!("🎯 Chair {}", p.value())),
                highlight_actor: true,
            },
        },

        Day(DayActivity::Discussion) => HostMain {
            title: "Discussion".into(),
            subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
            highlight_actor: true,
        },

        Evening(a) => match a {
            EveningActivity::NominationAnnouncement => {
                let nominees = engine.game.voting.get(&engine.round).map(|v| {
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

            EveningActivity::Voting | EveningActivity::TieVoting | EveningActivity::FinalVoting => {
                let voting = engine
                    .game
                    .voting
                    .get(&engine.round)
                    .cloned()
                    .unwrap_or_else(snapshot::Voting::default);

                HostMain {
                    title: "CAST YOUR VOTE".into(),
                    subtitle: Some(format_votes_verbose(&voting)),
                    highlight_actor: true,
                }
            }

            EveningActivity::TieDiscussion => HostMain {
                title: "Tie discussion".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },

            EveningActivity::FinalSpeech => HostMain {
                title: "Final speech".into(),
                subtitle: engine.actor.map(|p| format!("🗣 Chair {}", p.value())),
                highlight_actor: true,
            },
        },
    }
}

/* ---------------- Footer ---------------- */

fn build_footer(app: &App) -> HostFooter {
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
