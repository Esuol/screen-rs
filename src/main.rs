mod bar_chart;
mod dashboard;
mod layout;
mod views;

pub use bar_chart::render_bar_charts;
pub use dashboard::render_dashboard;
use layout::Layout;
pub use layout::Message;

fn main() -> iced::Result {
    iced::program(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}
