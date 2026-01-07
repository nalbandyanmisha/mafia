use crate::{
    domain::{Activity, DayActivity, EveningActivity, MorningActivity, NightActivity, Time},
    snapshot::{self, App, Check},
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

        let (title, title_style) = match engine.phase.unwrap().time() {
            Time::Night => (
                format!("Night · {}", engine.round),
                Style::default().fg(Color::Magenta),
            ),
            Time::Morning => (
                format!("Morning · {}", engine.round),
                Style::default().fg(Color::Cyan),
            ),
            Time::Day => (
                format!("Day · {}", engine.round),
                Style::default().fg(Color::Yellow),
            ),
            Time::Evening => (
                format!("Evening · {}", engine.round),
                Style::default().fg(Color::Green),
            ),
        };

        Self {
            title,
            title_style,
            header: Some(build_header(engine.phase.unwrap())),
            main: build_main(app),
            footer: build_footer(app),
        }
    }
}

/* ---------- builders ---------- */

fn build_header(phase: Activity) -> HostHeader {
    use Activity::*;
    use DayActivity::*;
    use EveningActivity::*;
    use NightActivity::*;
    let (text, style) = match phase {
        Night(night_activity) => match night_activity {
            RoleAssignment => ("Role Assignment", Color::Magenta),
            // SheriffReveal => self.game.sheriff().map(|p| p.position().unwrap()),
            SheriffReveal => ("Sheriff Reveal", Color::Magenta),
            DonReveal => ("Don Reveal", Color::Magenta),
            MafiaBriefing => ("Mafia Briefing", Color::Magenta),
            MafiaShooting => ("Mafia Shooting", Color::Magenta),
            SheriffCheck => ("Sheriff Check", Color::Magenta),
            DonCheck => ("Don Check", Color::Magenta),
        },

        Morning(morning_activity) => match morning_activity {
            MorningActivity::Guessing => ("Guessing", Color::Cyan),
            MorningActivity::FinalSpeech => ("R.I.P. Speech", Color::Cyan),
        },

        Day(Discussion) => ("Discussion", Color::Yellow),

        Evening(NominationAnnouncement) => ("Nomination Announcment", Color::Green),
        Evening(Voting) => ("Voting", Color::Green),
        Evening(TieDiscussion) => ("Tie Discussion", Color::Green),
        Evening(TieVoting) => ("Tie Voting", Color::Green),
        Evening(FinalVoting) => ("Final Voting", Color::Green),
        Evening(FinalSpeech) => ("Final Speech", Color::Green),
    };

    HostHeader {
        text: text.into(),
        style: Style::default().fg(style),
    }
}

fn build_main(app: &App) -> HostMain {
    use Activity::*;
    use DayActivity::*;
    use EveningActivity::*;
    use NightActivity::*;
    let engine = &app.engine;

    match engine.phase.unwrap() {
        Night(night_activity) => match night_activity {
            RoleAssignment => HostMain {
                title: "RoleAssignment".into(),
                subtitle: None,
                highlight_actor: false,
            },
            // SheriffReveal => self.game.sheriff().map(|p| p.position().unwrap()),
            SheriffReveal => HostMain {
                title: "SheriffReveal".into(),
                subtitle: None,
                highlight_actor: true,
            },
            DonReveal => HostMain {
                title: "DonReveal".into(),
                subtitle: None,
                highlight_actor: true,
            },
            MafiaBriefing => HostMain {
                title: "DonReveal".into(),
                subtitle: None,
                highlight_actor: true,
            },
            MafiaShooting => HostMain {
                title: "MAFIA SHOOTING".into(),
                subtitle: engine
                    .game
                    .kill
                    .get(&engine.round)
                    .map(|c| format!("🎯 Chair {}", c.value())),
                highlight_actor: true,
            },
            SheriffCheck => HostMain {
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

            DonCheck => HostMain {
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
        },

        Morning(morning_activity) => match morning_activity {
            MorningActivity::Guessing => HostMain {
                title: "Guessing".into(),
                subtitle: None,
                highlight_actor: false,
            },

            MorningActivity::FinalSpeech => HostMain {
                title: "Guessing".into(),
                subtitle: None,
                highlight_actor: false,
            },
        },

        Day(Discussion) => HostMain {
            title: "DISCUSSION".into(),
            subtitle: engine.actor.map(|c| format!("🗣️ Chair {}", c.value())),
            highlight_actor: true,
        },

        Evening(NominationAnnouncement) => {
            let nominees = engine
                .game
                .voting
                .get(&engine.round)
                .cloned()
                .unwrap_or_else(snapshot::Voting::default)
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
        Evening(Voting) => HostMain {
            title: "CAST YOUR VOTE".into(),
            subtitle: engine.actor.map(|c| format!("🎯 Chair {}", c.value())),
            highlight_actor: true,
        },

        Evening(TieDiscussion) => HostMain {
            title: "TieDiscussion".into(),
            subtitle: None,
            highlight_actor: false,
        },
        Evening(TieVoting) => HostMain {
            title: "TieVoting".into(),
            subtitle: None,
            highlight_actor: false,
        },
        Evening(FinalVoting) => HostMain {
            title: "FinalVoting".into(),
            subtitle: None,
            highlight_actor: false,
        },
        Evening(FinalSpeech) => HostMain {
            title: "FinalVoting".into(),
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
