use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct CommandLayout {
    pub area: Rect,
    pub input: Rect,
}

pub fn command(area: Rect) -> CommandLayout {
    let input = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    CommandLayout { area, input }
}

