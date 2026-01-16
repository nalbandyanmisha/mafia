use crate::tui::{layout, view};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Modifier, Stylize},
    text::Line,
};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::Header,
    view: &view::host::Header,
) -> anyhow::Result<()> {
    // Line 1: alive count | activity | out count
    // let width = layout.area.width as usize;

    // Center activity
    let activity = format!(" {} ", view.activity);
    let in_players = format!("In: {}", view.in_players);
    let out_players = format!("Out: {}", view.out_players);

    let line2 = Line::from(
        view.activity_info
            .clone()
            .expect("second line of header should exist"),
    )
    .alignment(Alignment::Center)
    .add_modifier(Modifier::ITALIC)
    .add_modifier(Modifier::DIM);

    frame.render_widget(
        Line::from(in_players)
            .alignment(Alignment::Left)
            .style(Color::Green)
            .add_modifier(Modifier::DIM),
        layout.left,
    );
    frame.render_widget(
        Line::from(activity)
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD),
        layout.center,
    );

    frame.render_widget(
        Line::from(out_players)
            .alignment(Alignment::Right)
            .style(Color::Red)
            .add_modifier(Modifier::DIM),
        layout.right,
    );
    frame.render_widget(line2, layout.s_line);

    Ok(())
}
