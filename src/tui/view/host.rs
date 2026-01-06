use crate::{
    domain::phase::{CheckPhase, DayPhase, NightPhase, Phase, VotingPhase},
    snapshot::{App, Check, Voting},
};
use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct HostView {
    pub title: String,
    pub title_style: Style,

    pub header: Option<HostHeader>,
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

impl HostView {
    pub fn from_snapshot(app: &App) -> Self {
        let engine = &app.engine;

        let (title, title_style) = match engine.phase {
            Phase::Day(_) => (
                format!("Day · {}", engine.round),
                Style::default().fg(Color::Yellow),
            ),
            Phase::Night(_) => (
                format!("Night · {}", engine.round),
                Style::default().fg(Color::Magenta),
            ),
        };

        Self {
            title,
            title_style,
            header: Some(build_header(engine.phase)),
            main: build_main(app),
            footer: build_footer(app),
        }
    }
}

/* ---------- builders ---------- */

fn build_header(phase: Phase) -> HostHeader {
    let (text, style) = match phase {
        Phase::Day(DayPhase::Morning) => ("Morning", Color::Yellow),
        Phase::Day(DayPhase::Discussion) => ("Discussion", Color::Yellow),
        Phase::Day(DayPhase::Voting(_)) => ("Voting", Color::Yellow),

        Phase::Night(NightPhase::RoleAssignment) => ("Role Assignment", Color::Magenta),
        Phase::Night(NightPhase::SheriffReveal) => ("Sheriff Reveal", Color::Magenta),
        Phase::Night(NightPhase::MafiaBriefing) => ("Mafia Briefing", Color::Magenta),
        Phase::Night(NightPhase::MafiaShoot) => ("Mafia Shooting", Color::Magenta),

        Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => {
            ("Sheriff Checking", Color::Magenta)
        }
        Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => {
            ("Don Checking", Color::Magenta)
        }
    };

    HostHeader {
        text: text.into(),
        style: Style::default().fg(style),
    }
}

fn build_main(app: &App) -> HostMain {
    let engine = &app.engine;

    match engine.phase {
        Phase::Day(DayPhase::Morning) => HostMain {
            title: "MORNING".into(),
            subtitle: None,
            highlight_actor: false,
        },

        Phase::Day(DayPhase::Discussion) => HostMain {
            title: "DISCUSSION".into(),
            subtitle: engine.actor.map(|c| format!("🗣️ Chair {}", c.value())),
            highlight_actor: true,
        },

        Phase::Day(DayPhase::Voting(VotingPhase::Nomination)) => {
            let nominees = engine
                .game
                .voting
                .get(&engine.round)
                .cloned()
                .unwrap_or_else(Voting::default)
                .nominees
                .iter()
                .map(|c| format!("🪑{}", c.value()))
                .collect::<Vec<_>>()
                .join(", ");

            HostMain {
                title: "NOMINATIONS".into(),
                subtitle: Some(format!("Nominated: {nominees}")),
                highlight_actor: true,
            }
        }

        Phase::Day(DayPhase::Voting(VotingPhase::VoteCast)) => HostMain {
            title: "CAST YOUR VOTE".into(),
            subtitle: engine.actor.map(|c| format!("🎯 Chair {}", c.value())),
            highlight_actor: true,
        },

        Phase::Night(NightPhase::RoleAssignment) => HostMain {
            title: "ROLE REVEAL".into(),
            subtitle: engine.actor.map(|c| format!("🎭 Chair {}", c.value())),
            highlight_actor: true,
        },

        Phase::Night(NightPhase::MafiaShoot) => HostMain {
            title: "MAFIA SHOOTING".into(),
            subtitle: engine
                .game
                .kill
                .get(&engine.round)
                .map(|c| format!("🎯 Chair {}", c.value())),
            highlight_actor: true,
        },

        Phase::Night(NightPhase::Investigation(CheckPhase::Sheriff)) => HostMain {
            title: "SHERIFF CHECKING".into(),
            subtitle: engine
                .game
                .check
                .get(&engine.round)
                .cloned()
                .unwrap_or_else(Check::default)
                .sheriff
                .map(|c| format!("🎯 Chair {}", c.value())),
            highlight_actor: true,
        },

        Phase::Night(NightPhase::Investigation(CheckPhase::Don)) => HostMain {
            title: "DON CHECKING".into(),
            subtitle: engine
                .game
                .check
                .get(&engine.round)
                .cloned()
                .unwrap_or_else(Check::default)
                .don
                .map(|c| format!("🎯 Chair {}", c.value())),
            highlight_actor: true,
        },
        _ => HostMain {
            title: "UNKNOWN PHASE".into(),
            subtitle: None,
            highlight_actor: false,
        },
    }
}

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
