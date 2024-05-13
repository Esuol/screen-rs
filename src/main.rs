extern crate charts;

mod bar_chart;
mod cus_dashboard;
mod dashboard;
mod generate_charts;
mod layout;
mod pie_chart;
mod solar;
mod views;
mod clock;

pub use bar_chart::render_bar_charts;
pub use cus_dashboard::Dashboard;
pub use dashboard::render_dashboard;
use iced::Font;
use layout::Layout;
pub use layout::Message;
pub use pie_chart::render_pie_charts;
pub use solar::State;
use std::thread;
pub use clock::Clock;

fn main() -> iced::Result {
    // 创建一个新的线程来生成所有的图表
    let handle = thread::spawn(|| {
        generate_charts::generate_all_plots();
    });

    // 在主线程中运行 Iced 程序
    let iced_result = iced::program(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run();

    // 等待图表生成线程完成
    handle.join().unwrap();

    iced_result
}
