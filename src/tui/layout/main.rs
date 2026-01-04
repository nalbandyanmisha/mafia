use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct MainLayout {
    pub area: Rect,
    pub content: Rect,
}

pub fn main(area: Rect) -> MainLayout {
    let content = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });

    MainLayout { area, content }
}
