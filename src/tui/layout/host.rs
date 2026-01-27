pub mod footer;
pub mod header;
pub mod main;

pub use footer::Footer;
pub use header::Header;
pub use main::Main;
use ratatui::layout::{Constraint, Layout, Margin, Rect};

#[derive(Debug, Clone)]
pub struct Host {
    pub area: Rect,
    pub header: Header,
    pub body: Main,
    pub footer: Footer,
}

impl Host {
    /// Create a HostLayout from a given area
    pub fn new(main_area: Rect) -> Self {
        let width = main_area.width / 3;
        let height = main_area.height / 3;

        let x = main_area.x + (main_area.width - width) / 2;
        let y = main_area.y + (main_area.height - height) / 2;

        let area = Rect {
            x,
            y,
            width,
            height,
        };

        // Split vertically: header/body/footer
        let rects = Layout::vertical([
            Constraint::Length(1), // empty line
            Constraint::Length(1), // header
            Constraint::Min(9),    // main
            Constraint::Length(1), // footer
            Constraint::Length(1), // empty line
        ])
        .split(area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }));

        Self {
            area,
            header: Header::new(rects[1]),
            body: Main::new(rects[2]),
            footer: Footer::new(rects[3]),
        }
    }
}
