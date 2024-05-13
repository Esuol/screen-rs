use crate::Dashboard;
use crate::Message;
use iced::alignment::Alignment;
use iced::widget::{canvas, column, container, horizontal_space, row, text};
use iced::{Border, Color, Element, Length, Theme};

pub fn render_dashboard() -> Element<'static, Message> {
    let dashboard =
        container(row![render_time_container(), render_dashboards_container()].spacing(10))
            .width(Length::Fill)
            .height(Length::FillPortion(20));

    dashboard.into()
}

fn render_time_container() -> Element<'static, Message> {
    let time_container = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(15))
    .height(Length::Fill)
    .center_x()
    .style(dashbord_time_container_style)];

    time_container.into()
}

fn render_dashboards_container() -> Element<'static, Message> {
    let dashboards_container = row![
        render_dashboard_item(20, "Fluid"),
        render_dashboard_item(10, "Scale"),
        render_dashboard_item(18, "Dawn"),
        render_dashboard_item(45, "Sober"),
        render_dashboard_item(54, "Wanna"),
        render_dashboard_item(32, "Remix"),
        render_dashboard_item(26, "Hopper"),
        render_dashboard_item(59, "Earth"),
    ]
    .spacing(10)
    .width(Length::FillPortion(80))
    .height(Length::Fill);

    dashboards_container.into()
}

fn render_dashboard_item(now: u8, title: &str) -> Element<'static, Message> {
    let dashboard_one = canvas(Dashboard::new(now))
        .width(Length::Fill)
        .height(Length::Fill);
    let header_text = text(title.to_string())
        .size(14)
        .color(Color::from_rgb8(42, 163, 199));
    let dashboard_item = row![container(
        column![header_text, dashboard_one,]
            .align_items(Alignment::Center)
            .padding(8)
    )
    .width(Length::FillPortion(12))
    .height(Length::Fill)
    .center_x()
    .center_y()
    .style(dashbord_time_container_style)];

    dashboard_item.into()
}

pub fn dashbord_time_container_style(theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Color::TRANSPARENT.into()),
        border: Border {
            width: 0.0,

            ..Border::default()
        },
        ..Default::default()
    }
}

// 生成仪表盘
