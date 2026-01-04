use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct EventsLayout {
    pub area: Rect,
    pub content: Rect,
}

pub fn events(area: Rect) -> EventsLayout {
    let content = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    EventsLayout { area, content }
}

