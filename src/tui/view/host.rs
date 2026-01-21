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
use crate::snapshot::{self, Check, Player};
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

        let host_text = Text::new(app);
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
    pub context: Option<String>,
    pub actor: Option<String>,
    pub timer: Option<u64>, // future, keep optional
    pub result: Option<String>,

    //Footer
    pub commands: Vec<String>,
}

pub struct TextBuilder {
    text: Text,
}

impl TextBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            text: Text {
                title: title.into(),
                subtitle: None,
                description: None,
                context: None,
                actor: None,
                timer: None,
                result: None,
                commands: Vec::new(),
            },
        }
    }

    pub fn subtitle(mut self, s: impl Into<String>) -> Self {
        self.text.subtitle = Some(s.into());
        self
    }

    pub fn description(mut self, s: impl Into<String>) -> Self {
        self.text.description = Some(s.into());
        self
    }

    pub fn context(mut self, s: impl Into<String>) -> Self {
        self.text.context = Some(s.into());
        self
    }

    pub fn actor(mut self, s: impl Into<String>) -> Self {
        self.text.actor = Some(s.into());
        self
    }

    pub fn timer(mut self, t: Option<u64>) -> Self {
        self.text.timer = t;
        self
    }

    pub fn result(mut self, s: impl Into<String>) -> Self {
        self.text.result = Some(s.into());
        self
    }

    pub fn command(mut self, c: impl Into<String>) -> Self {
        self.text.commands.push(c.into());
        self
    }

    pub fn commands(mut self, cmds: &[&str]) -> Self {
        self.text.commands = cmds.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(self) -> Text {
        self.text
    }
}

impl Text {
    pub fn new(app: &snapshot::App) -> Self {
        use Activity::*;
        let phase = app.engine.phase.expect("phase must exist");
        let day = app.engine.day;
        let timer = app.current_timer;
        let checks = app.engine.game.check.clone();
        let actor = app.engine.actor;
        let players = app.engine.game.players.as_slice();
        let guesses = app.engine.game.guess.as_slice();
        let voting = app
            .engine
            .game
            .voting
            .get(&day)
            .unwrap_or(&snapshot::Voting::default())
            .clone();
        let tie_voting = app
            .engine
            .game
            .tie_voting
            .get(&day)
            .unwrap_or(&snapshot::Voting::default())
            .clone();
        let final_votes = app
            .engine
            .game
            .final_voting
            .get(&day)
            .unwrap_or(&Vec::new())
            .clone();
        let eliminations = app
            .engine
            .game
            .eliminated
            .get(&day)
            .unwrap_or(&Vec::new())
            .clone();

        match phase {
            Night(activity) => Self::night(activity, day, actor, players, timer, checks),
            Morning(activity) => Self::morning(activity, actor, players, timer, guesses),
            Noon(activity) => Self::noon(
                activity,
                actor,
                players,
                timer,
                &voting.nominees,
                &voting.nominations,
            ),
            Evening(activity) => Self::evening(
                activity,
                actor,
                players,
                timer,
                voting,
                tie_voting,
                final_votes.as_slice(),
                eliminations.as_slice(),
            ),
        }
    }

    fn night(
        activity: NightActivity,
        day: usize,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        checks: HashMap<usize, Check>,
    ) -> Self {
        use NightActivity::*;
        match activity {
            RoleAssignment => Self::role_assignment(actor, players, timer),

            SheriffReveal => {
                Self::role_reveal("Sheriff Reveal", Role::Sheriff, actor, players, timer)
            }

            DonReveal => Self::role_reveal("Don Reveal", Role::Don, actor, players, timer),

            MafiaBriefing => Self::mafia_briefing(actor, players, timer),

            MafiaShooting => Self::mafia_shooting(actor, players, timer),

            SheriffCheck => Self::check(day, Role::Sheriff, actor, players, timer, checks),

            DonCheck => Self::check(day, Role::Don, actor, players, timer, checks),
        }
    }

    fn morning(
        activity: MorningActivity,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        guesses: &[Position],
    ) -> Self {
        use MorningActivity::*;

        match activity {
            Guessing => Self::guessing(actor, players, timer, guesses),
            DeathSpeech => Self::death_speech(actor, players, timer),
        }
    }
    fn noon(
        activity: NoonActivity,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        nominations: &HashMap<Position, Position>,
    ) -> Self {
        use NoonActivity::*;

        match activity {
            Discussion => Self::discussion(actor, players, timer, nominees, nominations),
        }
    }
    fn evening(
        activity: EveningActivity,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        voting: snapshot::Voting,
        tie_voting: snapshot::Voting,
        final_votes: &[Position],
        eliminations: &[Position],
    ) -> Self {
        use EveningActivity::*;
        let nominees = voting.nominees.as_slice();
        let votes = &voting.votes;
        let tie_nominees = tie_voting.nominees.as_slice();
        let tie_votes = &tie_voting.votes;

        match activity {
            Voting => Self::voting(actor, players, timer, nominees, votes),
            TieDiscussion => Self::tie_discussion(actor, players, timer, &tie_voting.nominees),
            TieVoting => Self::tie_voting(actor, players, timer, tie_nominees, tie_votes),
            FinalVoting => Self::final_voting(actor, timer, tie_nominees, final_votes),
            FinalSpeech => Self::final_speech(actor, players, timer, eliminations),
        }
    }

    fn role_assignment(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Text {
        let remaining = players.iter().filter(|p| p.role.is_none()).count();
        let mut builder =
            TextBuilder::new("Role Assignment").subtitle(format!("Remaining: {remaining}"));

        match actor {
            None => {
                // No active actor yet – show initial description
                builder = builder
                    .description(
                        "Assign roles to players one by one.\n\
                     Run `next` to select the next unassigned player.\n\
                     Use `assign` to give them a role.",
                    )
                    .commands(&["next", "warn"])
            }
            Some(position) => {
                let active_player = players
                    .iter()
                    .find(|p| p.position.expect("Player should have position") == position)
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is waiting.\n\
                        Run `assign` to reveal their role.",
                        active_player.name
                    ))
                    .commands(&["assign", "warn"])
                    .timer(timer);

                if let Some(role) = active_player.role {
                    builder = builder
                        .actor(format!(
                            "Player {} at {position} received their role.",
                            active_player.name
                        ))
                        .result(format!("Role assigned: {role}"))
                        .commands(&["next", "warn"]);
                }
            }
        }
        builder.build()
    }

    fn role_reveal(
        title: &str,
        role: Role,
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
    ) -> Text {
        let mut builder = TextBuilder::new(title).subtitle("Confirming identity");

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "Wake the {role} so they can confirm their role.\n\
                     The {role} will have a few seconds to look around.\n\
                     Run `next` to begin."
                    ))
                    .commands(&["next", "warn"]);
            }
            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.role == Some(role))
                    .expect("Required role must exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is the {role}.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "warn"]);
            }
        }

        builder.build()
    }

    fn mafia_briefing(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Text {
        // Collect positions of Don and Mafia members
        let mafia_positions = players
            .iter()
            .filter(|p| matches!(p.role, Some(Role::Don) | Some(Role::Mafia)))
            .map(|p| p.position.expect("Player should have position").to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let mut builder = TextBuilder::new("Mafia Briefing")
            .subtitle(format!("Members at positions: {mafia_positions}"));

        match actor {
            None => {
                // No active actor yet – description for host
                builder = builder
                    .description(
                        "Wake the Don and Mafia players.\n\
                     They have 60s to discuss strategy.\n\
                     Run `next` to start the timer.",
                    )
                    .commands(&["next", "warn"]);
            }
            Some(_) => {
                // During the briefing – no individual actor
                builder = builder
                    .actor("Mafia members are briefing.".to_string())
                    .timer(timer)
                    .commands(&["next", "warn"]);
            }
        }

        builder.build()
    }

    fn mafia_shooting(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Text {
        // Collect positions of Don and Mafia members
        let mafia_positions = players
            .iter()
            .filter(|p| matches!(p.role, Some(Role::Don) | Some(Role::Mafia)))
            .map(|p| p.position.expect("Player should have position").to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let mut builder = TextBuilder::new("Mafia Shooting")
            .subtitle(format!("Mafia members at positions: {mafia_positions}"));

        match actor {
            None => {
                // No active actor yet – instructions for host
                builder = builder
                    .description(
                        "Mafia will choose a target.\n\
                     If all shoot the same target, it's a kill.\n\
                     If any miss, the shot fails.\n\
                     Use `shoot n` to record a hit.\n\
                     Running `next` without recording counts as a miss.",
                    )
                    .commands(&["next", "shoot", "warn"]);
            }
            Some(_) => {
                // Active mafia shooting – no individual actor
                builder = builder
                    .actor("Mafia are executing their shot...".to_string())
                    .timer(timer)
                    .commands(&["next", "shoot", "warn"]);
            }
        }

        builder.build()
    }

    fn check(
        day: usize,
        role: Role, // Sheriff or Don
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        checks: HashMap<usize, Check>,
    ) -> Text {
        // Title based on role
        let title = match role {
            Role::Sheriff => "Sheriff Check",
            Role::Don => "Don Check",
            _ => panic!("Role must be Sheriff or Don for this function"),
        };

        // Subtitle: summarize previous checks
        let subtitle = if day > 1 {
            (1..=day)
                .filter_map(|d| checks.get(&d))
                .map(|check| match role {
                    Role::Sheriff => check
                        .sheriff
                        .map_or(String::new(), |pos| pos.value().to_string()),
                    Role::Don => check
                        .don
                        .map_or(String::new(), |pos| pos.value().to_string()),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        let mut builder = TextBuilder::new(title).subtitle(subtitle);

        match actor {
            None => {
                // No active actor yet – show instructions
                builder = builder
                    .description(format!(
                        "Wake the {role} to check a player.\n\
                     They have 10 seconds to act.\n\
                     Run `next` to select the {role}",
                    ))
                    .commands(&["next", "warn"]);
            }
            Some(position) => {
                // Find the active player
                let active_player = players
                    .iter()
                    .find(|p| p.role == Some(role))
                    .expect("{role} must exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is the {}.",
                        active_player.name, role
                    ))
                    .timer(timer)
                    .commands(&["next", "warn", "check"]);

                // Add result if check exists
                if let Some(check) = checks.get(&day) {
                    let target_pos = match role {
                        Role::Sheriff => check.sheriff,
                        Role::Don => check.don,
                        _ => None,
                    };

                    if let Some(target_pos) = target_pos {
                        let target_player = players
                            .iter()
                            .find(|p| p.position == Some(target_pos))
                            .expect("Target player must exist");

                        builder = builder.result(format!(
                            "{} checked player at position {}.\nFound role: {}",
                            role,
                            target_pos,
                            target_player.role.expect("Player must have role")
                        ));
                    }
                }
            }
        }

        builder.build()
    }

    fn guessing(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        guesses: &[Position],
    ) -> Text {
        let mut builder = TextBuilder::new("Mafia Guessing").subtitle("Last suspicion");

        match actor {
            None => {
                builder = builder
                    .description(
                        "Wake the killed player.\n\
                     They have 10s to name up to 3 mafia suspects.\n\
                     Run `next` to start the timer.\n\
                     If no guesses are recorded, it counts as none.",
                    )
                    .commands(&["next", "warn"]);
            }

            Some(position) => {
                let dead_player = players
                    .iter()
                    .find(|p| p.status == Status::Dead)
                    .expect("Dead player must exist");

                let result = if guesses.is_empty() {
                    "No mafia guesses were recorded.".to_string()
                } else {
                    format!(
                        "Guessed mafia positions: {}",
                        guesses
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} was killed",
                        dead_player.name
                    ))
                    .timer(timer)
                    .result(result)
                    .commands(&["next", "warn", "guess"]);
            }
        }

        builder.build()
    }

    fn death_speech(actor: Option<Position>, players: &[Player], timer: Option<u64>) -> Text {
        let mut builder = TextBuilder::new("Final Speech").subtitle("Last words");

        match actor {
            None => {
                builder = builder
                    .description(
                        "The killed player is allowed a final speech.\n\
                     They have 60s to share their last thoughts.\n\
                     Run `next` to select the player and start the timer.",
                    )
                    .commands(&["next", "warn"]);
            }

            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is giving final speech.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "warn"]);
            }
        }

        builder.build()
    }

    fn discussion(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        nominations: &HashMap<Position, Position>,
    ) -> Text {
        // Collect nominated players (values of the map)
        let nominated = nominees
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if nominated.is_empty() {
            "No nominations yet".to_string()
        } else {
            format!("Nominees: {nominated}")
        };

        let mut builder = TextBuilder::new("Discussion").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(
                        "Players may speak one by one.\n\
                     Each player has 60s to talk.\n\
                     Run `next` to select the next speaker.",
                    )
                    .commands(&["next", "warn"]);
            }

            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is speaking.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "nominate", "warn"]);

                if let Some(nominated) = nominations.get(&position) {
                    builder = builder.result(format!("Nominated player at position {nominated}"));
                }
            }
        }

        builder.build()
    }

    fn voting(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        votes: &HashMap<Position, Vec<Position>>,
    ) -> Text {
        let nominees_list = nominees
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if nominees.is_empty() {
            "No nominees".to_string()
        } else {
            format!("Nominees: {nominees_list}")
        };

        let mut builder = TextBuilder::new("Voting").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "The following players are nominated:\n{nominees_list}\n\
                     Votes are cast nominee by nominee.\n\
                     Use `vote <positions>` to record votes.\n\
                     Run `next` to move to the next nominee.",
                    ))
                    .commands(&["next", "vote", "warn"]);
            }

            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Voting for Player {} at position {position}.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "vote", "warn"]);

                if let Some(voters) = votes.get(&position) {
                    builder = builder.result(format!(
                        "Votes from: {}",
                        voters
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }

        builder.build()
    }

    fn tie_discussion(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        tied: &[Position],
    ) -> Text {
        let tied_list = tied
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if tied.is_empty() {
            "No tied nominees".to_string()
        } else {
            format!("Tied: {tied_list}")
        };

        let mut builder = TextBuilder::new("Tie Discussion").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "The vote resulted in a tie between:\n{tied_list}\n\
                     Each tied player gets 30s to speak.\n\
                     Run `next` to begin.",
                    ))
                    .commands(&["next", "warn"]);
            }

            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is speaking.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "warn"]);
            }
        }

        builder.build()
    }

    fn tie_voting(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        nominees: &[Position],
        votes: &HashMap<Position, Vec<Position>>,
    ) -> Text {
        let nominees_list = nominees
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if nominees.is_empty() {
            "No tied nominees".to_string()
        } else {
            format!("Tied nominees: {nominees_list}")
        };

        let mut builder = TextBuilder::new("Tie Voting").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "The following players are tied:\n{nominees_list}\n\
                     Votes are cast nominee by nominee.\n\
                     Use `vote <positions>` to record votes.\n\
                     Run `next` to move to the next nominee.",
                    ))
                    .commands(&["next", "vote", "warn"]);
            }

            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position should exist");

                builder = builder
                    .actor(format!(
                        "Voting for Player {} at position {position}.",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "vote", "warn"]);

                if let Some(voters) = votes.get(&position) {
                    builder = builder.result(format!(
                        "Votes from: {}",
                        voters
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }

        builder.build()
    }

    fn final_voting(
        actor: Option<Position>,
        timer: Option<u64>,
        nominees: &[Position],
        final_votes: &[Position],
    ) -> Text {
        // Prepare subtitle with tied nominees
        let nominees_list = nominees
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if nominees.is_empty() {
            "No tied nominees".to_string()
        } else {
            format!("Tied nominees: {nominees_list}")
        };

        let mut builder = TextBuilder::new("Final Voting").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "The following players were in tie voting:\n{nominees_list}\n\
                     Record final votes using `vote <positions>` (use 0 if none).\n\
                     Run `next` to move forward without recording votes."
                    ))
                    .commands(&["next", "vote", "warn"]);
            }
            Some(_) => {
                builder = builder
                    .actor("Cast final votes.".to_string())
                    .timer(timer)
                    .commands(&["next", "vote", "warn"]);

                if !final_votes.is_empty() {
                    builder = builder.result(format!(
                        "Players voted: {}",
                        final_votes
                            .iter()
                            .map(|p| p.value().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }
            }
        }

        builder.build()
    }

    fn final_speech(
        actor: Option<Position>,
        players: &[Player],
        timer: Option<u64>,
        eliminations: &[Position],
    ) -> Text {
        // Prepare subtitle with eliminated players
        let eliminated_list = eliminations
            .iter()
            .map(|p| p.value().to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let subtitle = if eliminations.is_empty() {
            "No eliminated players".to_string()
        } else {
            format!("Eliminated: {eliminated_list}")
        };

        let mut builder = TextBuilder::new("Final Speech").subtitle(subtitle);

        match actor {
            None => {
                builder = builder
                    .description(format!(
                        "The following players have been eliminated:\n{eliminated_list}\n\
                     Run `next` to select an eliminated player and start their final speech."
                    ))
                    .commands(&["next", "warn"]);
            }
            Some(position) => {
                let player = players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player with given position must exist");

                builder = builder
                    .actor(format!(
                        "Player {} at position {position} is giving their final speech",
                        player.name
                    ))
                    .timer(timer)
                    .commands(&["next", "warn"]);
            }
        }

        builder.build()
    }
}
