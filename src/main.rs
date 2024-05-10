mod layout;
mod views;

use layout::Layout;
pub use layout::Message;

fn main() -> iced::Result {
    iced::program(Layout::title, Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}
