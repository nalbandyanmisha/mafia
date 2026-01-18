pub mod footer;
pub mod header;
pub mod main;

use std::collections::HashMap;

pub use footer::Footer;
pub use header::Header;
use main::Actor;
pub use main::Main;

use crate::domain::{
    Activity, Day, EveningActivity, MorningActivity, NightActivity, NoonActivity, Position, Role,
    Status,
};
use crate::snapshot::{self, Check, Player, Voting};
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
        let engine = &app.engine;
        let phase = engine.phase.expect("phase must exist");

        let (title, title_style) = match phase.daytime() {
            Day::Night => (format!("🌙 Night · {}", engine.day), Color::Magenta),
            Day::Morning => (format!("☀ Morning · {}", engine.day), Color::Cyan),
            Day::Noon => (format!("☀ Day · {}", engine.day), Color::Yellow),
            Day::Evening => (format!("🌆 Evening · {}", engine.day), Color::Blue),
        };

        let host_text = Text::new(
            phase,
            engine.actor,
            &engine.game.players,
            app.current_timer,
            engine.day,
            engine.game.check.clone(),
            engine.game.guess.clone(),
            &engine
                .game
                .voting
                .get(&engine.day)
                .unwrap_or(&Voting::default())
                .nominations,
            &engine
                .game
                .voting
                .get(&engine.day)
                .unwrap_or(&Voting::default())
                .nominees,
            &engine
                .game
                .tie_voting
                .get(&engine.day)
                .unwrap_or(&Voting::default())
                .nominees,
            &engine
                .game
                .voting
                .get(&engine.day)
                .unwrap_or(&Voting::default())
                .votes,
            &engine
                .game
                .tie_voting
                .get(&engine.day)
                .unwrap_or(&Voting::default())
                .votes,
            engine
                .game
                .final_voting
                .get(&engine.day)
                .unwrap_or(&Vec::new()),
            engine
                .game
                .eliminated
                .get(&engine.day)
                .unwrap_or(&Vec::new()),
        );
        let in_p_c = engine
            .game
            .players
            .iter()
            .fold(0, |c, p| if p.status == Status::Alive { c + 1 } else { c });
        let out_p_c = 10 - in_p_c;

        let main = if engine.actor.is_some() {
            Main::Actor(Actor::new(
                host_text.actor.expect("actor should exist at this point"),
                host_text.timer,
                host_text.result,
            ))
        } else {
            Main::Description(host_text.description.expect("description should exist"))
        };

        Self {
            title,
            title_style,
            header: Header::new(in_p_c, out_p_c, host_text.title, host_text.subtitle),
            main,
            footer: Footer::new(&host_text.commands),
        }
    }
}

pub struct Text {
    // Header
    pub title: String,
    pub subtitle: Option<String>,

    // Main content
    pub description: Option<String>,
    pub actor: Option<String>,
    pub timer: Option<u64>, // future, keep optional
    pub result: Option<String>,

    //Footer
    pub commands: Vec<String>,
}

impl Text {
    fn header(title: String, subtitle: Option<String>) -> Self {
        Self {
            title,
            subtitle,
            description: None,
            actor: None,
            timer: None,
            result: None,
            commands: Vec::new(),
        }
    }

    fn role_assignment(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Role Assignment".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(
                    "Roles will be assigned to players one by one.\nRun `next` to select the player to assign a role".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is waiting",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = player
                    .role
                    .map(|role| format!("Player has been assigned {role}"));
                text.commands.push("next".to_string());
                text.commands.push("assign".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn sheriff_reveal(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let mut text = Self::header("Sheriff Reveal".to_string(), Some(String::new()));

        match actor {
            None => {
                text.description = Some(
                    "Woke up the sheriff to make sure players got his role\n has 5s to look at the city\nRun `next` to select the sheriff and to start timer".to_string(),
                );

                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.role.expect("Player should have position") == Role::Sheriff)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is the Sheriff",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn don_reveal(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let mut text = Self::header("Don Reveal".to_string(), Some(String::new()));

        match actor {
            None => {
                text.description = Some(
                    "Woke up the don to make sure player got his role\n has 5s to look at the city\nRun `next` to select the don and to start timer".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.role.expect("Player should have position") == Role::Don)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is the Don",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
        }
    }

    fn mafia_briefing(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let mafia = &players
            .iter()
            .filter(|p| {
                p.role.expect("Player should have role at this point") == Role::Don
                    || p.role.expect("Player should have role at this point") == Role::Mafia
            })
            .map(|p| {
                p.position
                    .expect("Player should have position at this point")
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");
        let mut text = Self::header("Mafia Briefing".to_string(), Some(mafia.clone()));

        match actor {
            None => {
                text.description = Some(
                    "Woke up the don and 2 mafia players\n They have 60s to come up with strategy\nRun `next` to start timer".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(_) => {
                text.actor = None;
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
        }
    }

    fn mafia_shooting(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let mafia = &players
            .iter()
            .filter(|p| {
                p.role.expect("Player should have role at this point") == Role::Don
                    || p.role.expect("Player should have role at this point") == Role::Mafia
            })
            .map(|p| {
                p.position
                    .expect("Player should have position at this point")
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join(", ");
        let mut text = Self::header("Mafia Shooting".to_string(), Some(mafia.clone()));

        match actor {
            None => {
                text.description = Some(
                    "Name the players in order in case all mafia players shoot one target record it\n Run 'shoot n' to record or \nRun `next` to move forward and record miss".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("shoot".to_string());

                text.commands.push("warn".to_string());

                text
            }
            Some(_) => {
                text.actor = None;
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("shoot".to_string());
                text.commands.push("warn".to_string());

                text
            }
        }
    }

    fn sheriff_check(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        day: usize,
        checks: HashMap<usize, Check>,
    ) -> Self {
        let subtitle = if day > 1 {
            let mut prev_checks = Vec::new();
            for d in 1..=day {
                prev_checks.push(checks.get(&d))
            }

            prev_checks
                .iter()
                .map(|c| {
                    if c.is_some() {
                        c.unwrap()
                            .sheriff
                            .expect("position should exist")
                            .value()
                            .to_string()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            String::new()
        };
        let mut text = Self::header("Sheriff Check".to_string(), Some(subtitle));

        match actor {
            None => {
                text.description = Some(
                    "Woke up the sheriff \n has 10ss to check one player\nRun `next` to select the sheriff and to start timer".to_string(),
                );

                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let sheriff = &players
                    .iter()
                    .find(|p| p.role.expect("Player should have position") == Role::Sheriff)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is the Sheriff",
                    sheriff.name.clone()
                ));
                text.timer = timer;
                text.result = checks.get(&day).map(|check| {
                    format!(
                        "Sheriff performed check of player at position {}\n Found: {}",
                        check.sheriff.expect("target position should exist"),
                        players
                            .iter()
                            .find(|p| p.position == check.sheriff)
                            .expect("Target player should exist")
                            .role
                            .expect("Player shoud have role")
                    )
                });
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text.commands.push("check".to_string());
                text
            }
        }
    }

    fn don_check(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        day: usize,
        checks: HashMap<usize, Check>,
    ) -> Self {
        let subtitle = if day > 1 {
            let mut prev_checks = Vec::new();
            for d in 1..=day {
                prev_checks.push(checks.get(&d))
            }

            prev_checks
                .iter()
                .map(|c| {
                    if c.is_some() {
                        c.unwrap()
                            .sheriff
                            .expect("position should exist")
                            .value()
                            .to_string()
                    } else {
                        String::new()
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            String::new()
        };
        let mut text = Self::header("Don Check".to_string(), Some(subtitle));

        match actor {
            None => {
                text.description = Some(
                    "Woke up the don \n has 10ss to check one player\nRun `next` to select the don and to start timer".to_string(),
                );

                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let don = &players
                    .iter()
                    .find(|p| p.role.expect("Player should have position") == Role::Don)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is the Don",
                    don.name.clone()
                ));
                text.timer = timer;

                text.result = checks.get(&day).map(|check| match check.don {
                    Some(position) => {
                        format!(
                            "Don performed check of player at position {}\n Found:  {}",
                            position,
                            players
                                .iter()
                                .find(|p| p.position == check.don)
                                .expect("Target player should exist")
                                .role
                                .expect("Player shoud have role")
                        )
                    }
                    None => "".to_string(),
                });
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text.commands.push("check".to_string());
                text
            }
        }
    }

    fn guesing(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        guesses: Vec<Position>,
    ) -> Self {
        let mut text = Self::header(
            "Mafia guessing".to_string(),
            Some("Just to fix".to_string()),
        );

        match actor {
            None => {
                text.description = Some(
                    "Woke up killed player \n has 10s to name 3 players as mafia\nRun `next` to select the player and to start timer".to_string(),
                );

                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let dead_p = &players
                    .iter()
                    .find(|p| p.status == Status::Dead)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} was been killed",
                    dead_p.name.clone()
                ));
                text.timer = timer;

                if guesses.is_empty() {
                    text.result = None;
                } else {
                    text.result = Some(format!(
                        "{} {} {} players have been recorded as guesses",
                        guesses[0], guesses[1], guesses[2]
                    ));
                }

                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text.commands.push("check".to_string());
                text
            }
        }
    }

    fn death_speech(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let mut text = Self::header(
            "Final Speech".to_string(),
            Some("Final thoughts of killed player".to_string()),
        );

        match actor {
            None => {
                text.description = Some(
                    "Final speech, player has 60s to talk\nRun `next` to select the player and to start timer".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is giving a final speach",
                    player.name.clone()
                ));
                text.timer = timer;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn discussion(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominations: &HashMap<Position, Position>,
    ) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Discussion".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(
                    "Players would be given 60s to talk\nRun `next` to select the player and to start timer".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is speaking",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = nominations
                    .get(&position)
                    .map(|pos| format!("Player  has nominated {pos}"));
                text.commands.push("next".to_string());
                text.commands.push("nominate".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn voting(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        votes: &HashMap<Position, Vec<Position>>,
    ) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Voting".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(format!(
                    "Players {} have been nominated\nRun `next` to select the nominee \nand record votes by issuing vote <positions> command",
                    nominees
                        .iter()
                        .map(|p| p.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Cast votes for Player {} at position {position}",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = votes.get(&position).map(|votes| {
                    format!(
                        "Players {}  has voted",
                        votes
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                });
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn tie_discussion(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Tie Discussion".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(
                    "Players would be given 30s to talk\nRun `next` to select the player and to start timer".to_string(),
                );
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Player {} at position {position} is speaking",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn tie_voting(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        votes: &HashMap<Position, Vec<Position>>,
    ) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Tie Voting".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(format!(
                    "Players {} have been nominated\nRun `next` to select the nominee \nand record votes by issuing vote <positions> command",
                    nominees
                        .iter()
                        .map(|p| p.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(position) => {
                let player = &players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                text.actor = Some(format!(
                    "Cast votes for Player {} at position {position}",
                    player.name.clone()
                ));
                text.timer = timer;
                text.result = votes.get(&position).map(|votes| {
                    format!(
                        "Players {}  has voted",
                        votes
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                });
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn final_voting(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        final_votes: &[Position],
    ) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Final Voting".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(format!(
                    "Players {} have been in tie voting\nRun vote <positions> \n and list positions who voted yes and 0 if none",
                    nominees
                        .iter()
                        .map(|p| p.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(_) => {
                text.actor = Some("Cast votes ".to_string());
                text.timer = timer;
                text.result = if final_votes.is_empty() {
                    None
                } else {
                    Some(format!(
                        "Players {}  has voted",
                        final_votes
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                    ))
                };

                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    fn final_speech(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        eliminations: &[Position],
    ) -> Self {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut text = Self::header(
            "Final Speech".to_string(),
            Some(format!("Remaining: {remaining}")),
        );

        match actor {
            None => {
                text.description = Some(format!(
                    "Players {} have been eliminated\nRun next to choose eliminated player and to start timer",
                    eliminations
                        .iter()
                        .map(|p| p.value().to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
                text.commands.push("next".to_string());
                text.commands.push("vote".to_string());
                text.commands.push("warn".to_string());

                text
            }
            Some(_) => {
                text.actor = Some("Cast votes ".to_string());
                text.timer = timer;
                text.result = None;
                text.commands.push("next".to_string());
                text.commands.push("warn".to_string());
                text
            }
        }
    }

    pub fn new(
        pahse: Activity,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        day: usize,
        checks: HashMap<usize, Check>,
        guesses: Vec<Position>,
        nominations: &HashMap<Position, Position>,
        nominees: &[Position],
        tie_nominees: &[Position],
        votes: &HashMap<Position, Vec<Position>>,
        tie_votes: &HashMap<Position, Vec<Position>>,
        final_votes: &[Position],
        eliminations: &[Position],
    ) -> Self {
        use Activity::*;
        use EveningActivity::*;
        use MorningActivity::*;
        use NightActivity::*;
        use NoonActivity::*;
        match pahse {
            Night(activity) => match activity {
                RoleAssignment => Self::role_assignment(actor, players, timer),
                SheriffReveal => Self::sheriff_reveal(actor, players, timer),
                DonReveal => Self::don_reveal(actor, players, timer),
                MafiaBriefing => Self::mafia_briefing(actor, players, timer),
                MafiaShooting => Self::mafia_shooting(actor, players, timer),
                SheriffCheck => Self::sheriff_check(actor, players, timer, day, checks),
                DonCheck => Self::don_check(actor, players, timer, day, checks),
            },

            Morning(activity) => match activity {
                Guessing => Self::guesing(actor, players, timer, guesses),
                DeathSpeech => Self::death_speech(actor, players, timer),
            },

            Noon(activity) => match activity {
                Discussion => Self::discussion(actor, players, timer, nominations),
            },

            Evening(activity) => match activity {
                NominationAnnouncement => Self::role_assignment(actor, players, timer),
                Voting => Self::voting(actor, players, timer, nominees, votes),
                TieDiscussion => Self::tie_discussion(actor, players, timer),
                TieVoting => Self::tie_voting(actor, players, timer, tie_nominees, tie_votes),
                FinalVoting => Self::final_voting(actor, players, timer, tie_nominees, final_votes),
                FinalSpeech => Self::final_speech(actor, players, timer, eliminations),
            },
        }
    }
}
