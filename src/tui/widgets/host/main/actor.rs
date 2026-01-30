use ratatui::{
    Frame,
    layout::Alignment,
    style::Style,
    style::*,
    text::{Line, Span, Text},
};

use crate::tui::{layout, view};

pub fn draw(
    frame: &mut Frame,
    layout: &layout::host::main::Actor,
    view: &view::host::main::Actor,
) -> anyhow::Result<()> {
    frame.render_widget(
        view.actor
            .lines()
            .map(Line::from)
            .collect::<Text>()
            .fg(Color::White)
            .alignment(Alignment::Center),
        layout.player,
    );

    if let Some(sec) = view.timer {
        let style = if sec <= 10 {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        };

        frame.render_widget(
            big_timer_text(sec, style).alignment(Alignment::Center),
            layout.timer,
        );
    }

    if let Some(r) = &view.result {
        frame.render_widget(
            r.lines()
                .map(Line::from)
                .collect::<Text>()
                .fg(Color::Green)
                .alignment(Alignment::Center),
            layout.result,
        );
    }

    Ok(())
}

fn big_timer_text(sec: u64, style: Style) -> Text<'static> {
    let time = format!("{:02}:{:02}", sec / 60, sec % 60);

    let mut lines = vec![String::new(); 7];

    for ch in time.chars() {
        let g = glyph(ch);
        for i in 0..7 {
            lines[i].push_str(g[i]);
            lines[i].push_str("  "); // spacing between glyphs
        }
    }

    Text::from(
        lines
            .into_iter()
            .map(|line| Line::from(Span::styled(line, style)))
            .collect::<Vec<_>>(),
    )
}

const DIGITS: [[&str; 7]; 10] = [
    // 0
    [
        " ███ ",
        "█   █",
        "█   █",
        "█   █",
        "█   █",
        "█   █",
        " ███ ",
    ],
    // 1
    [
        "  █  ",
        " ██  ",
        "  █  ",
        "  █  ",
        "  █  ",
        "  █  ",
        " ███ ",
    ],
    // 2
    [
        " ███ ",
        "█   █",
        "    █",
        " ███ ",
        "█    ",
        "█    ",
        "█████",
    ],
    // 3
    [
        "████ ",
        "    █",
        "    █",
        " ███ ",
        "    █",
        "    █",
        "████ ",
    ],
    // 4
    [
        "█   █",
        "█   █",
        "█   █",
        "█████",
        "    █",
        "    █",
        "    █",
    ],
    // 5
    [
        "█████",
        "█    ",
        "█    ",
        "████ ",
        "    █",
        "    █",
        "████ ",
    ],
    // 6
    [
        " ███ ",
        "█    ",
        "█    ",
        "████ ",
        "█   █",
        "█   █",
        " ███ ",
    ],
    // 7
    [
        "█████",
        "    █",
        "   █ ",
        "  █  ",
        " █   ",
        " █   ",
        " █   ",
    ],
    // 8
    [
        " ███ ",
        "█   █",
        "█   █",
        " ███ ",
        "█   █",
        "█   █",
        " ███ ",
    ],
    // 9
    [
        " ███ ",
        "█   █",
        "█   █",
        " ████",
        "    █",
        "    █",
        " ███ ",
    ],
];

fn glyph(ch: char) -> [&'static str; 7] {
    match ch {
        '0'..='9' => DIGITS[ch.to_digit(10).unwrap() as usize],
        ':' => [
            "     ", "  █  ", "     ", "     ", "  █  ", "     ", "     ",
        ],
        _ => [
            "     ", "     ", "     ", "     ", "     ", "     ", "     ",
        ],
    }
}
