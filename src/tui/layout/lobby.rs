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

        let panel = Rect {
            x,
            y,
            width,
            height,
        };

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
