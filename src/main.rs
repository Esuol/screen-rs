extern crate charts;

mod bar_chart;
mod dashboard;
mod generate_charts;
mod layout;
mod pie_chart;
mod solar;
mod views;

pub use bar_chart::render_bar_charts;
pub use dashboard::render_dashboard;
use iced::Font;
use layout::Layout;
pub use layout::Message;
pub use pie_chart::render_pie_charts;
pub use solar::State;

fn main() -> iced::Result {
    // // 生成所有的图表
    generate_charts::generate_all_plots();

    iced::program(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run()
}
