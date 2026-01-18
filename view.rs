pub mod view {
    pub mod chair {
        use ratatui::style::Color;
        use crate::domain::{Day, Position, Status};
        use crate::tui::view::PlayerView;
        pub struct ChairView {
            pub position: Position,
            pub state: ChairState,
            pub player: Option<PlayerView>,
            pub highlight: bool,
            pub border_style: Color,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChairView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ChairView",
                    "position",
                    &self.position,
                    "state",
                    &self.state,
                    "player",
                    &self.player,
                    "highlight",
                    &self.highlight,
                    "border_style",
                    &&self.border_style,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ChairView {
            #[inline]
            fn clone(&self) -> ChairView {
                ChairView {
                    position: ::core::clone::Clone::clone(&self.position),
                    state: ::core::clone::Clone::clone(&self.state),
                    player: ::core::clone::Clone::clone(&self.player),
                    highlight: ::core::clone::Clone::clone(&self.highlight),
                    border_style: ::core::clone::Clone::clone(&self.border_style),
                }
            }
        }
        pub enum ChairState {
            Empty,
            Alive,
            Dead,
            Eliminated,
            Removed,
            Speaking,
            Muted,
            Candidate,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChairState {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ChairState::Empty => "Empty",
                        ChairState::Alive => "Alive",
                        ChairState::Dead => "Dead",
                        ChairState::Eliminated => "Eliminated",
                        ChairState::Removed => "Removed",
                        ChairState::Speaking => "Speaking",
                        ChairState::Muted => "Muted",
                        ChairState::Candidate => "Candidate",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ChairState {
            #[inline]
            fn clone(&self) -> ChairState {
                match self {
                    ChairState::Empty => ChairState::Empty,
                    ChairState::Alive => ChairState::Alive,
                    ChairState::Dead => ChairState::Dead,
                    ChairState::Eliminated => ChairState::Eliminated,
                    ChairState::Removed => ChairState::Removed,
                    ChairState::Speaking => ChairState::Speaking,
                    ChairState::Muted => ChairState::Muted,
                    ChairState::Candidate => ChairState::Candidate,
                }
            }
        }
        impl ChairView {
            pub fn from_snapshot(
                position: Position,
                app: &crate::snapshot::App,
            ) -> Self {
                use Day::*;
                let player = app
                    .engine
                    .game
                    .players
                    .iter()
                    .find(|p| p.position == Some(position));
                let border_style = match app.engine.phase.unwrap().daytime() {
                    Night => Color::Magenta,
                    Morning => Color::Cyan,
                    Noon => Color::Yellow,
                    Evening => Color::Blue,
                };
                let player_view = player
                    .map(|_| PlayerView::from_snapshot(position, app));
                let state = match &player_view {
                    None => ChairState::Empty,
                    Some(view) => {
                        match view.status {
                            Status::Alive => ChairState::Alive,
                            Status::Dead => ChairState::Dead,
                            Status::Eliminated => ChairState::Eliminated,
                            Status::Removed => ChairState::Removed,
                        }
                    }
                };
                let highlight = app.engine.actor == Some(position);
                Self {
                    position,
                    state,
                    player: player_view,
                    highlight,
                    border_style,
                }
            }
        }
    }
    pub mod command {
        pub struct CommandView {
            pub input: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CommandView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "CommandView",
                    "input",
                    &&self.input,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for CommandView {
            #[inline]
            fn clone(&self) -> CommandView {
                CommandView {
                    input: ::core::clone::Clone::clone(&self.input),
                }
            }
        }
        impl CommandView {
            pub fn from_snapshot(app: &crate::snapshot::App) -> Self {
                Self { input: app.input.clone() }
            }
        }
    }
    pub mod events {
        pub struct EventsView {
            pub messages: Vec<crate::app::events::Event>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for EventsView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "EventsView",
                    "messages",
                    &&self.messages,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for EventsView {
            #[inline]
            fn clone(&self) -> EventsView {
                EventsView {
                    messages: ::core::clone::Clone::clone(&self.messages),
                }
            }
        }
        impl EventsView {
            pub fn from_snapshot(app: &crate::snapshot::App) -> Self {
                Self {
                    messages: app.events.clone(),
                }
            }
        }
    }
    pub mod host {
        pub mod footer {
            pub struct Footer {
                pub commands: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Footer {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Footer",
                        "commands",
                        &&self.commands,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Footer {
                #[inline]
                fn clone(&self) -> Footer {
                    Footer {
                        commands: ::core::clone::Clone::clone(&self.commands),
                    }
                }
            }
            impl Footer {
                pub fn new(commands: &[&str]) -> Self {
                    Self {
                        commands: commands.join(" | "),
                    }
                }
            }
        }
        pub mod header {
            use crate::{domain::Activity, snapshot};
            pub struct Header {
                pub in_players: usize,
                pub out_players: usize,
                pub activity: String,
                pub activity_info: Option<String>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Header {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Header",
                        "in_players",
                        &self.in_players,
                        "out_players",
                        &self.out_players,
                        "activity",
                        &self.activity,
                        "activity_info",
                        &&self.activity_info,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Header {
                #[inline]
                fn clone(&self) -> Header {
                    Header {
                        in_players: ::core::clone::Clone::clone(&self.in_players),
                        out_players: ::core::clone::Clone::clone(&self.out_players),
                        activity: ::core::clone::Clone::clone(&self.activity),
                        activity_info: ::core::clone::Clone::clone(&self.activity_info),
                    }
                }
            }
            impl Header {
                pub fn new(
                    in_players: usize,
                    out_players: usize,
                    activity: String,
                    activity_info: Option<String>,
                ) -> Self {
                    Self {
                        in_players,
                        out_players,
                        activity,
                        activity_info,
                    }
                }
            }
        }
        pub mod main {
            mod actor {
                use crate::domain::Position;
                pub struct Actor {
                    pub position: Position,
                    pub instructions: String,
                    pub timer: Option<u64>,
                    pub result: Option<String>,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Actor {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "Actor",
                            "position",
                            &self.position,
                            "instructions",
                            &self.instructions,
                            "timer",
                            &self.timer,
                            "result",
                            &&self.result,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Actor {
                    #[inline]
                    fn clone(&self) -> Actor {
                        Actor {
                            position: ::core::clone::Clone::clone(&self.position),
                            instructions: ::core::clone::Clone::clone(
                                &self.instructions,
                            ),
                            timer: ::core::clone::Clone::clone(&self.timer),
                            result: ::core::clone::Clone::clone(&self.result),
                        }
                    }
                }
                impl Actor {
                    pub fn new(
                        position: Position,
                        instructions: String,
                        timer: Option<u64>,
                        result: Option<String>,
                    ) -> Self {
                        Self {
                            position,
                            instructions,
                            timer,
                            result,
                        }
                    }
                }
            }
            pub use actor::Actor;
            pub enum Main {
                Actor(Actor),
                Description(String),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Main {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Main::Actor(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Actor",
                                &__self_0,
                            )
                        }
                        Main::Description(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Description",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Main {
                #[inline]
                fn clone(&self) -> Main {
                    match self {
                        Main::Actor(__self_0) => {
                            Main::Actor(::core::clone::Clone::clone(__self_0))
                        }
                        Main::Description(__self_0) => {
                            Main::Description(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
        }
        pub use footer::Footer;
        pub use header::Header;
        use main::Actor;
        pub use main::Main;
        use crate::domain::{
            Activity, Day, EveningActivity, MorningActivity, NightActivity, NoonActivity,
            Status,
        };
        use crate::snapshot;
        use ratatui::style::Color;
        pub struct HostView {
            pub title: String,
            pub title_style: Color,
            pub header: Header,
            pub main: Main,
            pub footer: Footer,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for HostView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "HostView",
                    "title",
                    &self.title,
                    "title_style",
                    &self.title_style,
                    "header",
                    &self.header,
                    "main",
                    &self.main,
                    "footer",
                    &&self.footer,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for HostView {
            #[inline]
            fn clone(&self) -> HostView {
                HostView {
                    title: ::core::clone::Clone::clone(&self.title),
                    title_style: ::core::clone::Clone::clone(&self.title_style),
                    header: ::core::clone::Clone::clone(&self.header),
                    main: ::core::clone::Clone::clone(&self.main),
                    footer: ::core::clone::Clone::clone(&self.footer),
                }
            }
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
                    Day::Night => {
                        (
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("🌙 Night · {0}", engine.day),
                                )
                            }),
                            Color::Magenta,
                        )
                    }
                    Day::Morning => {
                        (
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("☀ Morning · {0}", engine.day),
                                )
                            }),
                            Color::Cyan,
                        )
                    }
                    Day::Noon => {
                        (
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("☀ Day · {0}", engine.day),
                                )
                            }),
                            Color::Yellow,
                        )
                    }
                    Day::Evening => {
                        (
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("🌆 Evening · {0}", engine.day),
                                )
                            }),
                            Color::Blue,
                        )
                    }
                };
                let (activity, activity_info) = match phase {
                    Night(activity) => {
                        match activity {
                            RoleAssignment => {
                                let player_left_to_be_assinged_role = engine
                                    .game
                                    .players
                                    .iter()
                                    .fold(0, |c, p| if p.role.is_none() { c + 1 } else { c });
                                (
                                    activity.to_string(),
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Players left: {0}",
                                                player_left_to_be_assinged_role,
                                            ),
                                        )
                                    }),
                                )
                            }
                            SheriffReveal => (activity.to_string(), "???".to_string()),
                            DonReveal => (activity.to_string(), "???".to_string()),
                            MafiaBriefing => (activity.to_string(), "???".to_string()),
                            MafiaShooting => (activity.to_string(), "???".to_string()),
                            SheriffCheck => {
                                (activity.to_string(), "Prev: ???".to_string())
                            }
                            DonCheck => (activity.to_string(), "Prev: ???".to_string()),
                        }
                    }
                    Morning(activity) => {
                        match activity {
                            Guessing => (activity.to_string(), "???".to_string()),
                            DeathSpeech => (activity.to_string(), "???".to_string()),
                        }
                    }
                    Noon(activity) => {
                        match activity {
                            Discussion => {
                                let nominees = engine
                                    .game
                                    .voting
                                    .get(&engine.day)
                                    .unwrap_or(&snapshot::Voting::default())
                                    .nominees
                                    .clone()
                                    .iter()
                                    .map(|p| p.to_string())
                                    .collect::<String>();
                                (activity.to_string(), nominees)
                            }
                        }
                    }
                    Evening(activity) => {
                        match activity {
                            NominationAnnouncement => {
                                (activity.to_string(), "???".to_string())
                            }
                            Voting => (activity.to_string(), "???".to_string()),
                            TieDiscussion => (activity.to_string(), "???".to_string()),
                            TieVoting => (activity.to_string(), "???".to_string()),
                            FinalVoting => (activity.to_string(), "???".to_string()),
                            FinalSpeech => (activity.to_string(), "???".to_string()),
                        }
                    }
                };
                let in_p_c = engine
                    .game
                    .players
                    .iter()
                    .fold(0, |c, p| if p.status == Status::Alive { c + 1 } else { c });
                let out_p_c = 10 - in_p_c;
                let main = if let Some(actor) = engine.actor {
                    Main::Actor(
                        Actor::new(
                            actor,
                            "instructions".to_string(),
                            app.current_timer,
                            Some("result".to_string()),
                        ),
                    )
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
    }
    pub mod host_narration {
        use crate::{
            domain::{Activity, NightActivity, Position, Role},
            snapshot,
        };
        /// HostNarration structure
        pub struct HostNarration {
            pub title: String,
            pub body: Vec<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for HostNarration {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "HostNarration",
                    "title",
                    &self.title,
                    "body",
                    &&self.body,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for HostNarration {
            #[inline]
            fn clone(&self) -> HostNarration {
                HostNarration {
                    title: ::core::clone::Clone::clone(&self.title),
                    body: ::core::clone::Clone::clone(&self.body),
                }
            }
        }
        /// Build narration for the host panel directly from snapshot::App
        pub fn build_host_narration(app: &snapshot::App) -> HostNarration {
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
                    p.role.is_some_and(|r| r == Role::Mafia)
                        || p.role.is_some_and(|r| r == Role::Don)
                })
                .collect();
            use Activity::*;
            use NightActivity::*;
            match app.engine.phase.expect("phase should exist") {
                Night(RoleAssignment) => {
                    if let Some(player) = active_players.first() {
                        match player.role {
                            Some(role) => {
                                HostNarration {
                                    title: "Role Assigned".into(),
                                    body: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "Chair {0} has been assigned role \'{1}\'.",
                                                        player.position.expect("Player must have position"),
                                                        role,
                                                    ),
                                                )
                                            }),
                                        ]),
                                    ),
                                }
                            }
                            None => {
                                HostNarration {
                                    title: "Role Assignment".into(),
                                    body: <[_]>::into_vec(
                                        ::alloc::boxed::box_new([
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!(
                                                        "Chair {0} is active, awaiting role.",
                                                        player.position.expect("Player must have position"),
                                                    ),
                                                )
                                            }),
                                        ]),
                                    ),
                                }
                            }
                        }
                    } else {
                        HostNarration {
                            title: "Role Assignment".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new(["No chairs active.".into()]),
                            ),
                        }
                    }
                }
                Night(SheriffReveal) => {
                    if let Some(player) = active_players.first() {
                        HostNarration {
                            title: "Sheriff Reveal".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Chair {0} is revealed as Sheriff.",
                                                player.position.expect("Player must have position"),
                                            ),
                                        )
                                    }),
                                ]),
                            ),
                        }
                    } else {
                        HostNarration {
                            title: "Sheriff Reveal".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new(["No Sheriff active.".into()]),
                            ),
                        }
                    }
                }
                Night(DonReveal) => {
                    if let Some(chair) = active_players.first() {
                        HostNarration {
                            title: "Don Reveal".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Chair {0} is revealed as Don.",
                                                chair.position.expect("Player must have position"),
                                            ),
                                        )
                                    }),
                                ]),
                            ),
                        }
                    } else {
                        HostNarration {
                            title: "Don Reveal".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new(["No Don active.".into()]),
                            ),
                        }
                    }
                }
                Night(MafiaBriefing) => {
                    if !mafia.is_empty() {
                        let positions: Vec<_> = mafia
                            .iter()
                            .map(|c| c.position)
                            .collect();
                        HostNarration {
                            title: "Mafia Briefing".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Mafia Chairs: {0:?}",
                                                format_positions(positions.as_slice()),
                                            ),
                                        )
                                    }),
                                ]),
                            ),
                        }
                    } else {
                        HostNarration {
                            title: "Mafia Briefing".into(),
                            body: <[_]>::into_vec(
                                ::alloc::boxed::box_new(["No Mafia active.".into()]),
                            ),
                        }
                    }
                }
                _ => {
                    HostNarration {
                        title: "Waiting…".into(),
                        body: <[_]>::into_vec(
                            ::alloc::boxed::box_new(["No active activity.".into()]),
                        ),
                    }
                }
            }
        }
        fn format_positions(positions: &[Option<Position>]) -> String {
            positions
                .iter()
                .map(|p| p.unwrap().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }
    }
    pub mod lobby {
        use crate::{
            domain::{EngineState, LobbyStatus},
            snapshot,
        };
        pub struct LobbyView {
            pub title: String,
            pub players: Vec<LobbyPlayerView>,
            pub player_count: usize,
            pub max_players: u8,
            pub available_positions: Vec<u8>,
            pub ready: bool,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LobbyView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "title",
                    "players",
                    "player_count",
                    "max_players",
                    "available_positions",
                    "ready",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.title,
                    &self.players,
                    &self.player_count,
                    &self.max_players,
                    &self.available_positions,
                    &&self.ready,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "LobbyView",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LobbyView {
            #[inline]
            fn clone(&self) -> LobbyView {
                LobbyView {
                    title: ::core::clone::Clone::clone(&self.title),
                    players: ::core::clone::Clone::clone(&self.players),
                    player_count: ::core::clone::Clone::clone(&self.player_count),
                    max_players: ::core::clone::Clone::clone(&self.max_players),
                    available_positions: ::core::clone::Clone::clone(
                        &self.available_positions,
                    ),
                    ready: ::core::clone::Clone::clone(&self.ready),
                }
            }
        }
        pub struct LobbyPlayerView {
            pub name: String,
            pub position: Option<u8>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for LobbyPlayerView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "LobbyPlayerView",
                    "name",
                    &self.name,
                    "position",
                    &&self.position,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for LobbyPlayerView {
            #[inline]
            fn clone(&self) -> LobbyPlayerView {
                LobbyPlayerView {
                    name: ::core::clone::Clone::clone(&self.name),
                    position: ::core::clone::Clone::clone(&self.position),
                }
            }
        }
        impl LobbyView {
            pub fn from_snapshot(app: &snapshot::App) -> Self {
                let title = match app.engine.state {
                    EngineState::Lobby(LobbyStatus::Waiting) => "Waiting",
                    EngineState::Lobby(LobbyStatus::Ready) => "Ready",
                    _ => "Unknown",
                }
                    .to_string();
                let players_vec: Vec<LobbyPlayerView> = app
                    .engine
                    .game
                    .players
                    .iter()
                    .map(|p| LobbyPlayerView {
                        name: p.name.clone(),
                        position: p.position.map(|pos| pos.value()),
                    })
                    .collect();
                let player_count = players_vec.len();
                let assigned_positions: Vec<u8> = players_vec
                    .iter()
                    .filter_map(|p| p.position)
                    .collect();
                const MAX_PLAYERS: u8 = 10;
                let available_positions: Vec<u8> = (1..=MAX_PLAYERS)
                    .filter(|p| !assigned_positions.contains(p))
                    .collect();
                let ready = players_vec.len() == MAX_PLAYERS as usize
                    && players_vec.iter().all(|p| p.position.is_some());
                Self {
                    title,
                    players: players_vec,
                    player_count,
                    max_players: MAX_PLAYERS,
                    available_positions,
                    ready,
                }
            }
        }
    }
    pub mod main {
        use crate::{domain::EngineState, snapshot, tui::view::{LobbyView, TableView}};
        pub enum MainView {
            Lobby(LobbyView),
            Table(TableView),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MainView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    MainView::Lobby(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Lobby",
                            &__self_0,
                        )
                    }
                    MainView::Table(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Table",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MainView {
            #[inline]
            fn clone(&self) -> MainView {
                match self {
                    MainView::Lobby(__self_0) => {
                        MainView::Lobby(::core::clone::Clone::clone(__self_0))
                    }
                    MainView::Table(__self_0) => {
                        MainView::Table(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        impl MainView {
            pub fn from_snapshot(app: &snapshot::App) -> Self {
                match app.engine.state {
                    EngineState::Lobby(_) => {
                        MainView::Lobby(LobbyView::from_snapshot(app))
                    }
                    _ => MainView::Table(TableView::from_snapshot(app)),
                }
            }
        }
    }
    pub mod player {
        use crate::domain::{Position, Role, Status};
        use crate::snapshot::{self, Voting};
        pub struct PlayerView {
            pub name: String,
            pub role: Option<Role>,
            pub warnings: u8,
            pub status: Status,
            pub is_nominated: bool,
            pub nominated: Option<Position>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlayerView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "name",
                    "role",
                    "warnings",
                    "status",
                    "is_nominated",
                    "nominated",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.name,
                    &self.role,
                    &self.warnings,
                    &self.status,
                    &self.is_nominated,
                    &&self.nominated,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "PlayerView",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PlayerView {
            #[inline]
            fn clone(&self) -> PlayerView {
                PlayerView {
                    name: ::core::clone::Clone::clone(&self.name),
                    role: ::core::clone::Clone::clone(&self.role),
                    warnings: ::core::clone::Clone::clone(&self.warnings),
                    status: ::core::clone::Clone::clone(&self.status),
                    is_nominated: ::core::clone::Clone::clone(&self.is_nominated),
                    nominated: ::core::clone::Clone::clone(&self.nominated),
                }
            }
        }
        impl PlayerView {
            pub fn from_snapshot(position: Position, app: &snapshot::App) -> Self {
                let player = app
                    .engine
                    .game
                    .players
                    .iter()
                    .find(|p| p.position == Some(position))
                    .expect("Player at given position not found");
                let is_nominated = app
                    .engine
                    .game
                    .voting
                    .get(&app.engine.day)
                    .cloned()
                    .unwrap_or_else(Voting::default)
                    .nominees
                    .iter()
                    .any(|n| n == &position);
                let nominated = app
                    .engine
                    .game
                    .voting
                    .get(&app.engine.day)
                    .cloned()
                    .unwrap_or_else(Voting::default)
                    .nominations
                    .iter()
                    .find(|n| n.0 == &position)
                    .map(|n| *n.1);
                Self {
                    name: player.name.clone(),
                    role: player.role,
                    warnings: player.warnings,
                    status: player.status,
                    is_nominated,
                    nominated,
                }
            }
        }
    }
    pub mod shell {
        use super::{CommandView, EventsView, MainView};
        use crate::snapshot;
        pub struct Shell {
            pub main: MainView,
            pub command: CommandView,
            pub events: EventsView,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Shell {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Shell",
                    "main",
                    &self.main,
                    "command",
                    &self.command,
                    "events",
                    &&self.events,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Shell {
            #[inline]
            fn clone(&self) -> Shell {
                Shell {
                    main: ::core::clone::Clone::clone(&self.main),
                    command: ::core::clone::Clone::clone(&self.command),
                    events: ::core::clone::Clone::clone(&self.events),
                }
            }
        }
        impl Shell {
            /// Compute the views from the snapshot
            pub fn new(app: &snapshot::App) -> Self {
                Self {
                    main: MainView::from_snapshot(app),
                    command: CommandView::from_snapshot(app),
                    events: EventsView::from_snapshot(app),
                }
            }
        }
    }
    pub mod table {
        use crate::snapshot;
        use crate::tui::view::{ChairView, HostView};
        pub struct TableView {
            pub host: HostView,
            pub chairs: Vec<ChairView>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TableView {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TableView",
                    "host",
                    &self.host,
                    "chairs",
                    &&self.chairs,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TableView {
            #[inline]
            fn clone(&self) -> TableView {
                TableView {
                    host: ::core::clone::Clone::clone(&self.host),
                    chairs: ::core::clone::Clone::clone(&self.chairs),
                }
            }
        }
        impl TableView {
            pub fn from_snapshot(app: &snapshot::App) -> Self {
                let host = HostView::from_snapshot(app);
                let chairs = (1u8..=10)
                    .map(|i| {
                        let position = i.into();
                        ChairView::from_snapshot(position, app)
                    })
                    .collect();
                Self { host, chairs }
            }
        }
    }
    pub use chair::ChairView;
    pub use command::CommandView;
    pub use events::EventsView;
    pub use host::HostView;
    pub use host_narration::HostNarration;
    pub use lobby::LobbyView;
    pub use main::MainView;
    pub use player::PlayerView;
    pub use shell::Shell;
    pub use table::TableView;
    pub struct View {
        pub screen: Shell,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for View {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "View",
                "screen",
                &&self.screen,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for View {
        #[inline]
        fn clone(&self) -> View {
            View {
                screen: ::core::clone::Clone::clone(&self.screen),
            }
        }
    }
    impl View {
        /// Compute the views from the snapshot
        pub fn new(app: &crate::snapshot::App) -> Self {
            Self { screen: Shell::new(app) }
        }
    }
}
