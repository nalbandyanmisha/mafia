pub mod widgets {
    pub mod chair {
        use ratatui::{
            Frame, layout::Alignment, style::{Color, Modifier, Style},
            widgets::{Block, BorderType, Borders},
        };
        use crate::tui::{layout, view::chair::{ChairState, ChairView}};
        use super::player;
        fn build_chair_frame(view: &ChairView) -> Block<'static> {
            let pos = view.position.value();
            let (style, icon) = match view.state {
                ChairState::Empty => {
                    (
                        Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM),
                        "⬜",
                    )
                }
                ChairState::Alive => (Style::default().fg(Color::White), "💚"),
                ChairState::Dead => {
                    (Style::default().fg(Color::Red).add_modifier(Modifier::DIM), "💀")
                }
                ChairState::Eliminated => {
                    (Style::default().fg(Color::Red).add_modifier(Modifier::DIM), "❌")
                }
                ChairState::Removed => {
                    (Style::default().fg(Color::Red).add_modifier(Modifier::DIM), "🚫")
                }
                ChairState::Speaking => {
                    (
                        Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
                        "🗣️",
                    )
                }
                ChairState::Muted => {
                    (
                        Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
                        "🤐",
                    )
                }
                ChairState::Candidate => (Style::default().fg(Color::Magenta), "🎯"),
            };
            let border_style = if view.highlight {
                Style::default().fg(view.border_style).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            };
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style)
                .title(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("Chair {0} ({1})", pos, icon))
                    }),
                )
                .title_alignment(Alignment::Center)
                .style(style)
        }
        pub fn draw(frame: &mut Frame, chair: &layout::Chair, view: &ChairView) {
            let block = build_chair_frame(view);
            frame.render_widget(block, chair.area);
            if let Some(player_view) = &view.player {
                let player_layout = layout::Player::new(chair.area, 6);
                player::draw(frame, &player_layout, player_view);
            }
        }
    }
    pub mod command {
        use ratatui::{
            Frame, style::{Color, Style},
            widgets::{Block, Borders, Paragraph},
        };
        use crate::tui::{layout, view};
        pub fn draw(
            frame: &mut Frame,
            layout: &layout::Command,
            view: &view::CommandView,
        ) {
            let layout = layout::Command::new(layout.area);
            frame
                .render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Command Input")
                        .style(Style::default().fg(Color::Cyan)),
                    layout.area,
                );
            frame.render_widget(Paragraph::new(view.input.clone()), layout.input);
        }
    }
    pub mod events {
        use ratatui::{
            Frame, style::{Color, Style},
            text::Line, widgets::{Block, Borders, Paragraph, Wrap},
        };
        use crate::{tui::layout, tui::view::events::EventsView};
        pub fn draw(frame: &mut Frame, layout: &layout::Events, view: &EventsView) {
            frame
                .render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" EVENTS ")
                        .style(Style::default().fg(Color::Magenta)),
                    layout.area,
                );
            let lines: Vec<Line> = if view.messages.is_empty() {
                <[_]>::into_vec(::alloc::boxed::box_new([Line::from("No events yet")]))
            } else {
                view.messages.iter().map(|m| Line::from(m.to_string().clone())).collect()
            };
            frame
                .render_widget(
                    Paragraph::new(lines).wrap(Wrap { trim: true }),
                    layout.content,
                );
        }
    }
    pub mod host {
        pub mod footer {
            use crate::tui::{layout, view};
            use ratatui::{
                Frame, layout::Alignment, style::{Modifier, Stylize},
                widgets::Paragraph,
            };
            pub fn draw(
                frame: &mut Frame,
                layout: &layout::host::Footer,
                view: &view::host::Footer,
            ) -> anyhow::Result<()> {
                let paragraph = Paragraph::new(view.commands.clone())
                    .alignment(Alignment::Center)
                    .add_modifier(Modifier::ITALIC);
                frame.render_widget(paragraph, layout.area);
                Ok(())
            }
        }
        pub mod header {
            use crate::tui::{layout, view};
            use ratatui::{
                Frame, layout::Alignment, style::{Color, Modifier, Stylize},
                text::Line,
            };
            pub fn draw(
                frame: &mut Frame,
                layout: &layout::host::Header,
                view: &view::host::Header,
            ) -> anyhow::Result<()> {
                let activity = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!(" {0} ", view.activity))
                });
                let in_players = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("In: {0}", view.in_players))
                });
                let out_players = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Out: {0}", view.out_players))
                });
                let line2 = Line::from(
                        view
                            .activity_info
                            .clone()
                            .expect("second line of header should exist"),
                    )
                    .alignment(Alignment::Center)
                    .add_modifier(Modifier::ITALIC)
                    .add_modifier(Modifier::DIM);
                frame
                    .render_widget(
                        Line::from(in_players)
                            .alignment(Alignment::Left)
                            .style(Color::Green)
                            .add_modifier(Modifier::DIM),
                        layout.left,
                    );
                frame
                    .render_widget(
                        Line::from(activity)
                            .alignment(Alignment::Center)
                            .add_modifier(Modifier::BOLD),
                        layout.center,
                    );
                frame
                    .render_widget(
                        Line::from(out_players)
                            .alignment(Alignment::Right)
                            .style(Color::Red)
                            .add_modifier(Modifier::DIM),
                        layout.right,
                    );
                frame.render_widget(line2, layout.s_line);
                Ok(())
            }
        }
        pub mod main {
            mod actor {
                use ratatui::{Frame, layout::Alignment, text::Text, widgets::Paragraph};
                use crate::tui::{layout, util::centered_area, view};
                pub fn draw(
                    frame: &mut Frame,
                    layout: &layout::host::main::Actor,
                    view: &view::host::main::Actor,
                ) -> anyhow::Result<()> {
                    use ratatui::{style::*, text::{Line, Span}};
                    let mut lines = <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Line::from(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("🎭 {0}", view.position),
                                    )
                                }),
                            ),
                        ]),
                    );
                    if let Some(sec) = view.timer {
                        let style = if sec <= 10 {
                            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        };
                        lines
                            .push(
                                Line::from(
                                    Span::styled(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("⏳ {0:02}:{1:02}", sec / 60, sec % 60),
                                            )
                                        }),
                                        style,
                                    ),
                                ),
                            );
                    }
                    lines.push(Line::from(view.instructions.clone()));
                    if let Some(r) = &view.result {
                        lines
                            .push(
                                Line::from(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("✅ {0}", r))
                                    }),
                                ),
                            );
                    }
                    let text = Text::from(lines);
                    let centered = centered_area(layout.area, text.height() as u16);
                    frame
                        .render_widget(
                            Paragraph::new(text).alignment(Alignment::Center),
                            centered,
                        );
                    Ok(())
                }
            }
            mod description {
                use ratatui::{
                    Frame, layout::Alignment, text::Text, widgets::{Paragraph, Wrap},
                };
                use crate::tui::{layout, util::centered_area};
                pub fn draw(
                    frame: &mut Frame,
                    layout: &layout::host::main::Description,
                    text: &str,
                ) -> anyhow::Result<()> {
                    let text = Text::from(text.to_string());
                    let centered = centered_area(layout.area, text.height() as u16);
                    frame
                        .render_widget(
                            Paragraph::new(text.to_string())
                                .alignment(Alignment::Center)
                                .wrap(Wrap { trim: true }),
                            centered,
                        );
                    Ok(())
                }
            }
            use crate::tui::{layout, view};
            use ratatui::Frame;
            pub fn draw(
                frame: &mut Frame,
                layout: &layout::host::Main,
                view: &view::host::Main,
            ) -> anyhow::Result<()> {
                match view {
                    view::host::Main::Actor(actor) => {
                        actor::draw(frame, &layout.actor, actor)?;
                    }
                    view::host::Main::Description(text) => {
                        description::draw(frame, &layout.desc, text)?;
                    }
                }
                Ok(())
            }
        }
        use crate::tui::{layout, view};
        use ratatui::{Frame, layout::Alignment, widgets::{Block, BorderType, Borders}};
        pub fn draw(
            frame: &mut Frame,
            host: &layout::Host,
            view: &view::HostView,
        ) -> anyhow::Result<()> {
            frame
                .render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .title(view.title.clone())
                        .title_alignment(Alignment::Center)
                        .style(view.title_style),
                    host.area,
                );
            header::draw(frame, &host.header, &view.header)?;
            main::draw(frame, &host.body, &view.main)?;
            footer::draw(frame, &host.footer, &view.footer)?;
            Ok(())
        }
    }
    pub mod lobby {
        use crate::tui::layout;
        use crate::tui::view::LobbyView;
        use ratatui::{
            Frame, layout::Alignment, style::{Modifier, Style},
            text::{Line, Span},
            widgets::{Block, Borders, Paragraph},
        };
        pub fn draw(
            frame: &mut Frame,
            lobby_area: &layout::Lobby,
            view: &LobbyView,
        ) -> Result<(), anyhow::Error> {
            let header = Paragraph::new(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Line::from(
                                    Span::styled(
                                        &view.title,
                                        Style::default().add_modifier(Modifier::BOLD),
                                    ),
                                )
                                .alignment(Alignment::Center),
                            Line::from(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "{0} / {1} players",
                                            view.player_count,
                                            view.max_players,
                                        ),
                                    )
                                }),
                            ),
                        ]),
                    ),
                )
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(header, lobby_area.header);
            let mut lines = Vec::new();
            for player in &view.players {
                let pos = match player.position {
                    Some(p) => p.to_string(),
                    None => "—".to_string(),
                };
                lines
                    .push(
                        Line::from(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    Span::raw(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0:<12}", player.name))
                                        }),
                                    ),
                                    Span::raw(" position: "),
                                    Span::styled(
                                        pos,
                                        Style::default().add_modifier(Modifier::BOLD),
                                    ),
                                ]),
                            ),
                        ),
                    );
            }
            if view.players.is_empty() {
                lines
                    .push(
                        Line::from(
                            Span::styled(
                                "No players joined yet",
                                Style::default().add_modifier(Modifier::ITALIC),
                            ),
                        ),
                    );
            }
            let player_list = Paragraph::new(lines)
                .block(Block::default().borders(Borders::ALL).title("Players"));
            frame.render_widget(player_list, lobby_area.body);
            let available = if view.available_positions.is_empty() {
                "none".to_string()
            } else {
                view.available_positions
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let status = if view.ready {
                Span::styled(
                    "READY TO START",
                    Style::default().add_modifier(Modifier::BOLD),
                )
            } else {
                Span::raw("Waiting for players")
            };
            let footer = Paragraph::new(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Line::from(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("Available positions: {0}", available),
                                    )
                                }),
                            ),
                            Line::from(status),
                        ]),
                    ),
                )
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(footer, lobby_area.footer);
            Ok(())
        }
    }
    pub mod main {
        use ratatui::{
            Frame, style::{Color, Style},
            widgets::{Block, Borders},
        };
        use crate::tui::{layout, view::MainView, widgets::{lobby, table}};
        pub fn draw(frame: &mut Frame, layout: &layout::Main, view: &MainView) {
            frame
                .render_widget(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" MAIN ")
                        .style(Style::default().fg(Color::Green)),
                    layout.area,
                );
            match view {
                MainView::Lobby(lobby_view) => {
                    let lobby_layout = layout::Lobby::new(layout.content);
                    lobby::draw(frame, &lobby_layout, lobby_view).unwrap();
                }
                MainView::Table(table_view) => {
                    let table_layout = layout::Table::new(layout.content, 10);
                    table::draw(frame, &table_layout, table_view).unwrap();
                }
            }
        }
    }
    pub mod player {
        use crate::domain::Status;
        use crate::tui::layout;
        use crate::tui::view::player::PlayerView;
        use ratatui::Frame;
        use ratatui::layout::Alignment;
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Paragraph, Wrap};
        pub fn draw(frame: &mut Frame, l: &layout::Player, view: &PlayerView) {
            let mut lines: Vec<Line> = Vec::new();
            lines
                .push(
                    Line::from(
                        Span::styled(
                            &view.name,
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ),
                );
            let warnings = "⚠️".repeat(view.warnings as usize);
            let status_icon = match view.status {
                Status::Alive => "🟢 Alive",
                Status::Dead => "💀 Dead",
                Status::Eliminated => "❌ Out",
                Status::Removed => "🚫 Removed",
            };
            lines
                .push(
                    Line::from(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Span::styled(warnings, Style::default().fg(Color::Yellow)),
                                Span::raw("   "),
                                Span::styled(status_icon, Style::default().fg(Color::Gray)),
                            ]),
                        ),
                    ),
                );
            if let Some(role) = &view.role {
                lines
                    .push(
                        Line::from(
                            Span::styled(
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("🎭 {0}", role))
                                }),
                                Style::default().fg(Color::Magenta),
                            ),
                        ),
                    );
            }
            if view.is_nominated || view.nominated.is_some() {
                let mut spans = Vec::new();
                if view.is_nominated {
                    spans
                        .push(
                            Span::styled(
                                "📌",
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ),
                        );
                    spans.push(Span::raw("   "));
                }
                if let Some(target) = view.nominated {
                    spans
                        .push(Span::styled("🗳️", Style::default().fg(Color::Cyan)));
                    spans.push(Span::raw(" → "));
                    spans
                        .push(
                            Span::styled(
                                target.to_string(),
                                Style::default().fg(Color::Cyan),
                            ),
                        );
                }
                lines.push(Line::from(spans));
            }
            let paragraph = Paragraph::new(lines)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });
            frame.render_widget(paragraph, l.area);
        }
    }
    pub mod table {
        use crate::tui::view::{ChairView, TableView};
        use crate::tui::widgets::{chair, host};
        use ratatui::Frame;
        use crate::tui::layout;
        fn draw_chairs_around_host(
            frame: &mut Frame,
            layout_chairs: &[layout::Chair],
            view_chairs: &[ChairView],
        ) -> Result<(), anyhow::Error> {
            for (i, chair) in layout_chairs.iter().enumerate() {
                chair::draw(frame, chair, &view_chairs[i]);
            }
            Ok(())
        }
        pub fn draw(
            frame: &mut Frame,
            layout: &layout::Table,
            view: &TableView,
        ) -> Result<(), anyhow::Error> {
            host::draw(frame, &layout.host, &view.host)?;
            draw_chairs_around_host(frame, &layout.chairs, &view.chairs).unwrap();
            Ok(())
        }
    }
    use ratatui::Frame;
    use super::{layout::Layout, view::View};
    pub fn draw(frame: &mut Frame, terminal: &Layout, data: &View) {
        main::draw(frame, &terminal.screen.main, &data.screen.main);
        command::draw(frame, &terminal.screen.command, &data.screen.command);
        events::draw(frame, &terminal.screen.events, &data.screen.events);
    }
}
