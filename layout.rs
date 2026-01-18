pub mod layout {
    pub mod chair {
        use ratatui::layout::{Margin, Rect};
        pub struct Chair {
            pub area: Rect,
            pub content: Rect,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Chair {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Chair",
                    "area",
                    &self.area,
                    "content",
                    &&self.content,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Chair {
            #[inline]
            fn clone(&self) -> Chair {
                Chair {
                    area: ::core::clone::Clone::clone(&self.area),
                    content: ::core::clone::Clone::clone(&self.content),
                }
            }
        }
        impl Chair {
            /// Create a ChairLayout from a given area
            pub fn new(area: Rect) -> Self {
                let content = area
                    .inner(Margin {
                        vertical: 1,
                        horizontal: 1,
                    });
                Self { area, content }
            }
        }
    }
    pub mod command {
        use ratatui::layout::Rect;
        pub struct Command {
            pub area: Rect,
            pub input: Rect,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Command {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Command",
                    "area",
                    &self.area,
                    "input",
                    &&self.input,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Command {
            #[inline]
            fn clone(&self) -> Command {
                Command {
                    area: ::core::clone::Clone::clone(&self.area),
                    input: ::core::clone::Clone::clone(&self.input),
                }
            }
        }
        impl Command {
            /// Create a CommandLayout from a given area
            pub fn new(area: Rect) -> Self {
                let input = area
                    .inner(ratatui::layout::Margin {
                        vertical: 1,
                        horizontal: 1,
                    });
                Self { area, input }
            }
        }
    }
    pub mod events {
        use ratatui::layout::{Margin, Rect};
        pub struct Events {
            pub area: Rect,
            pub content: Rect,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Events {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Events",
                    "area",
                    &self.area,
                    "content",
                    &&self.content,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Events {
            #[inline]
            fn clone(&self) -> Events {
                Events {
                    area: ::core::clone::Clone::clone(&self.area),
                    content: ::core::clone::Clone::clone(&self.content),
                }
            }
        }
        impl Events {
            /// Create an EventsLayout from a given area
            pub fn new(area: Rect) -> Self {
                let content = area
                    .inner(Margin {
                        vertical: 1,
                        horizontal: 1,
                    });
                Self { area, content }
            }
        }
    }
    pub mod host {
        pub mod footer {
            use ratatui::layout::Rect;
            pub struct Footer {
                pub area: Rect,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Footer {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Footer",
                        "area",
                        &&self.area,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Footer {
                #[inline]
                fn clone(&self) -> Footer {
                    Footer {
                        area: ::core::clone::Clone::clone(&self.area),
                    }
                }
            }
            impl Footer {
                pub fn new(area: Rect) -> Self {
                    Self { area }
                }
            }
        }
        pub mod header {
            use ratatui::layout::Rect;
            pub struct Header {
                pub left: Rect,
                pub center: Rect,
                pub right: Rect,
                pub s_line: Rect,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Header {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Header",
                        "left",
                        &self.left,
                        "center",
                        &self.center,
                        "right",
                        &self.right,
                        "s_line",
                        &&self.s_line,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Header {
                #[inline]
                fn clone(&self) -> Header {
                    Header {
                        left: ::core::clone::Clone::clone(&self.left),
                        center: ::core::clone::Clone::clone(&self.center),
                        right: ::core::clone::Clone::clone(&self.right),
                        s_line: ::core::clone::Clone::clone(&self.s_line),
                    }
                }
            }
            impl Header {
                pub fn new(area: Rect) -> Self {
                    use ratatui::layout::{Constraint, Layout};
                    let lines = Layout::vertical([
                            Constraint::Length(1),
                            Constraint::Length(1),
                        ])
                        .split(area);
                    let first_rows = Layout::horizontal([
                            Constraint::Percentage(25),
                            Constraint::Percentage(50),
                            Constraint::Percentage(25),
                        ])
                        .split(lines[0]);
                    Self {
                        left: first_rows[0],
                        center: first_rows[1],
                        right: first_rows[2],
                        s_line: lines[1],
                    }
                }
            }
        }
        pub mod main {
            mod actor {
                use ratatui::layout::{Constraint, Layout, Margin, Rect};
                pub struct Actor {
                    pub area: Rect,
                    pub position: Rect,
                    pub time: Rect,
                    pub instruction: Rect,
                    pub result: Rect,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Actor {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "Actor",
                            "area",
                            &self.area,
                            "position",
                            &self.position,
                            "time",
                            &self.time,
                            "instruction",
                            &self.instruction,
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
                            area: ::core::clone::Clone::clone(&self.area),
                            position: ::core::clone::Clone::clone(&self.position),
                            time: ::core::clone::Clone::clone(&self.time),
                            instruction: ::core::clone::Clone::clone(&self.instruction),
                            result: ::core::clone::Clone::clone(&self.result),
                        }
                    }
                }
                impl Actor {
                    /// Create a MainLayout from a given area
                    pub fn new(area: Rect) -> Self {
                        let content = area
                            .inner(Margin {
                                vertical: 1,
                                horizontal: 1,
                            });
                        let lines = Layout::vertical([
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Length(1),
                                Constraint::Length(1),
                            ])
                            .split(content);
                        Self {
                            area,
                            position: lines[1],
                            time: lines[2],
                            instruction: lines[3],
                            result: lines[4],
                        }
                    }
                }
            }
            mod description {
                use ratatui::layout::{Constraint, Layout, Margin, Rect};
                pub struct Description {
                    pub area: Rect,
                    pub desc: Rect,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for Description {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Description",
                            "area",
                            &self.area,
                            "desc",
                            &&self.desc,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for Description {
                    #[inline]
                    fn clone(&self) -> Description {
                        Description {
                            area: ::core::clone::Clone::clone(&self.area),
                            desc: ::core::clone::Clone::clone(&self.desc),
                        }
                    }
                }
                impl Description {
                    /// Create a MainLayout from a given area
                    pub fn new(area: Rect) -> Self {
                        let content = area
                            .inner(Margin {
                                vertical: 1,
                                horizontal: 1,
                            });
                        let lines = Layout::vertical([Constraint::Length(1)])
                            .split(content);
                        Self { area, desc: lines[0] }
                    }
                }
            }
            pub use actor::Actor;
            pub use description::Description;
            use ratatui::layout::Rect;
            pub struct Main {
                pub actor: Actor,
                pub desc: Description,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Main {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Main",
                        "actor",
                        &self.actor,
                        "desc",
                        &&self.desc,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Main {
                #[inline]
                fn clone(&self) -> Main {
                    Main {
                        actor: ::core::clone::Clone::clone(&self.actor),
                        desc: ::core::clone::Clone::clone(&self.desc),
                    }
                }
            }
            impl Main {
                pub fn new(area: Rect) -> Self {
                    Self {
                        actor: Actor::new(area),
                        desc: Description::new(area),
                    }
                }
            }
        }
        pub use footer::Footer;
        pub use header::Header;
        pub use main::Main;
        use ratatui::layout::{Constraint, Layout, Margin, Rect};
        pub struct Host {
            pub area: Rect,
            pub header: Header,
            pub body: Main,
            pub footer: Footer,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Host {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Host",
                    "area",
                    &self.area,
                    "header",
                    &self.header,
                    "body",
                    &self.body,
                    "footer",
                    &&self.footer,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Host {
            #[inline]
            fn clone(&self) -> Host {
                Host {
                    area: ::core::clone::Clone::clone(&self.area),
                    header: ::core::clone::Clone::clone(&self.header),
                    body: ::core::clone::Clone::clone(&self.body),
                    footer: ::core::clone::Clone::clone(&self.footer),
                }
            }
        }
        impl Host {
            /// Create a HostLayout from a given area
            pub fn new(main_area: Rect) -> Self {
                let width = main_area.width / 3;
                let height = main_area.height / 3;
                let x = main_area.x + (main_area.width - width) / 2;
                let y = main_area.y + (main_area.height - height) / 2;
                let area = Rect { x, y, width, height };
                let rects = Layout::vertical([
                        Constraint::Length(1),
                        Constraint::Length(2),
                        Constraint::Min(6),
                        Constraint::Length(1),
                        Constraint::Length(1),
                    ])
                    .split(
                        area
                            .inner(Margin {
                                vertical: 1,
                                horizontal: 1,
                            }),
                    );
                Self {
                    area,
                    header: Header::new(rects[1]),
                    body: Main::new(rects[2]),
                    footer: Footer::new(rects[3]),
                }
            }
        }
    }
    pub mod lobby {
        use ratatui::layout::{Constraint, Direction, Layout, Rect};
        pub struct Lobby {
            pub panel: Rect,
            pub header: Rect,
            pub body: Rect,
            pub footer: Rect,
        }
        impl Lobby {
            /// Create a LobbyLayout from a given area
            pub fn new(main_area: Rect) -> Self {
                let width = main_area.width / 3;
                let height = main_area.height / 3;
                let x = main_area.x + (main_area.width - width) / 2;
                let y = main_area.y + (main_area.height - height) / 2;
                let panel = Rect { x, y, width, height };
                let rects = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(3),
                        Constraint::Length(3),
                    ])
                    .split(panel);
                Self {
                    panel,
                    header: rects[0],
                    body: rects[1],
                    footer: rects[2],
                }
            }
        }
    }
    pub mod main {
        use ratatui::layout::{Margin, Rect};
        pub struct Main {
            pub area: Rect,
            pub content: Rect,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Main {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Main",
                    "area",
                    &self.area,
                    "content",
                    &&self.content,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Main {
            #[inline]
            fn clone(&self) -> Main {
                Main {
                    area: ::core::clone::Clone::clone(&self.area),
                    content: ::core::clone::Clone::clone(&self.content),
                }
            }
        }
        impl Main {
            /// Create a MainLayout from a given area
            pub fn new(area: Rect) -> Self {
                let content = area
                    .inner(Margin {
                        vertical: 1,
                        horizontal: 1,
                    });
                Self { area, content }
            }
        }
    }
    pub mod player {
        use ratatui::layout::{Constraint, Direction, Layout, Rect};
        pub struct Player {
            pub area: Rect,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Player {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Player",
                    "area",
                    &&self.area,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Player {
            #[inline]
            fn clone(&self) -> Player {
                Player {
                    area: ::core::clone::Clone::clone(&self.area),
                }
            }
        }
        impl Player {
            /// Create a ChairLayout from a given area
            pub fn new(area: Rect, height: u16) -> Self {
                let vertical = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(0),
                        Constraint::Length(height),
                        Constraint::Min(0),
                    ])
                    .split(area);
                Self { area: vertical[1] }
            }
        }
    }
    pub mod shell {
        use ratatui::layout::{Constraint, Layout, Rect};
        use super::{Command, Events, Main};
        pub struct Shell {
            pub main: Main,
            pub command: Command,
            pub events: Events,
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
            /// Compute the full shell layout given the terminal area
            pub fn new(area: Rect) -> Self {
                let [left, events] = Layout::horizontal([
                        Constraint::Percentage(75),
                        Constraint::Percentage(25),
                    ])
                    .areas(area);
                let [main, command] = Layout::vertical([
                        Constraint::Min(10),
                        Constraint::Length(3),
                    ])
                    .areas(left);
                Self {
                    main: Main::new(main),
                    command: Command::new(command),
                    events: Events::new(events),
                }
            }
        }
    }
    pub mod table {
        use crate::tui::layout;
        use ratatui::layout::Rect;
        pub struct Table {
            pub host: layout::Host,
            pub chairs: Vec<layout::Chair>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Table {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Table",
                    "host",
                    &self.host,
                    "chairs",
                    &&self.chairs,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Table {
            #[inline]
            fn clone(&self) -> Table {
                Table {
                    host: ::core::clone::Clone::clone(&self.host),
                    chairs: ::core::clone::Clone::clone(&self.chairs),
                }
            }
        }
        impl Table {
            /// Create a TableLayout from a given area and number of chairs
            pub fn new(main: Rect, chair_count: usize) -> Self {
                let host = layout::Host::new(main);
                let chair_w = main.width / 6;
                let chair_h = main.height / 6;
                let mut chairs = calculate_chairs(
                    main,
                    host.area,
                    chair_count,
                    chair_w,
                    chair_h,
                );
                sort_clockwise(&host.area, &mut chairs);
                chairs.rotate_right(6);
                let chairs = chairs.into_iter().map(layout::Chair::new).collect();
                Self { host, chairs }
            }
        }
        fn calculate_chairs(
            table: Rect,
            host: Rect,
            n: usize,
            w: u16,
            h: u16,
        ) -> Vec<Rect> {
            let mut areas = Vec::with_capacity(n);
            let host_center_x = host.x + host.width / 2;
            let host_center_y = host.y + host.height / 2;
            let radius_x = host.width / 2 + w + 2;
            let radius_y = host.height / 2 + h + 1;
            let angle_step = 360.0 / n as f64;
            for i in 0..n {
                let angle_deg = i as f64 * angle_step;
                let angle_rad = angle_deg.to_radians();
                let cx = host_center_x as f64 + radius_x as f64 * angle_rad.cos();
                let cy = host_center_y as f64 + radius_y as f64 * angle_rad.sin();
                let x = (cx - w as f64 / 2.0).round() as u16;
                let y = (cy - h as f64 / 2.0).round() as u16;
                let x = x.max(table.x).min(table.x + table.width - w);
                let y = y.max(table.y).min(table.y + table.height - h);
                areas.push(Rect { x, y, width: w, height: h });
            }
            areas
        }
        fn center_of(rect: &Rect) -> (f64, f64) {
            let cx = rect.x as f64 + rect.width as f64 / 2.0;
            let cy = rect.y as f64 + rect.height as f64 / 2.0;
            (cx, cy)
        }
        fn angle_from_host(host: &Rect, player: &Rect) -> f64 {
            let (host_cx, host_cy) = center_of(host);
            let corners = [
                (player.x as f64, player.y as f64),
                (player.x as f64 + player.width as f64, player.y as f64),
                (player.x as f64, player.y as f64 + player.height as f64),
                (
                    player.x as f64 + player.width as f64,
                    player.y as f64 + player.height as f64,
                ),
            ];
            let closest = corners
                .iter()
                .min_by(|a, b| {
                    let da = (a.0 - host_cx).powi(2) + (a.1 - host_cy).powi(2);
                    let db = (b.0 - host_cx).powi(2) + (b.1 - host_cy).powi(2);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            let dx = closest.0 - host_cx;
            let dy = closest.1 - host_cy;
            let mut angle = dy.atan2(dx);
            angle -= -std::f64::consts::FRAC_PI_4;
            if angle < 0.0 {
                angle += 2.0 * std::f64::consts::PI;
            }
            angle
        }
        fn sort_clockwise(host: &Rect, chairs: &mut [Rect]) {
            chairs
                .sort_by(|a, b| {
                    angle_from_host(host, a)
                        .partial_cmp(&angle_from_host(host, b))
                        .unwrap()
                });
        }
    }
    pub use chair::Chair;
    pub use command::Command;
    pub use events::Events;
    pub use host::Host;
    pub use lobby::Lobby;
    pub use main::Main;
    pub use player::Player;
    use ratatui::layout::Rect;
    pub use shell::Shell;
    pub use table::Table;
    pub struct Layout {
        pub screen: Shell,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Layout {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Layout",
                "screen",
                &&self.screen,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Layout {
        #[inline]
        fn clone(&self) -> Layout {
            Layout {
                screen: ::core::clone::Clone::clone(&self.screen),
            }
        }
    }
    impl Layout {
        pub fn new(area: Rect) -> Self {
            Self { screen: Shell::new(area) }
        }
    }
}
