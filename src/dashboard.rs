use crate::Message;
use iced::widget::{column, container, horizontal_space, row, text};
use iced::{Border, Element, Length, Theme};

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
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
        render_dashboard_item(),
    ]
    .spacing(10)
    .width(Length::FillPortion(80))
    .height(Length::Fill);

    dashboards_container.into()
}

fn render_dashboard_item() -> Element<'static, Message> {
    let dashboard_item = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(12))
    .height(Length::Fill)
    .center_x()
    .style(dashbord_time_container_style)];

    dashboard_item.into()
}

pub fn dashbord_time_container_style(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}
