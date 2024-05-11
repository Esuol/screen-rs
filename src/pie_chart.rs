use crate::Message;
use iced::widget::{column, container, horizontal_space, row, text};
use iced::{Border, Element, Length, Theme};

pub fn render_pie_charts() -> Element<'static, Message> {
    let dashboard = container(
        row![
            render_pie_chart_container(),
            render_bar_chart_container(),
            render_switch_info_container(),
        ]
        .spacing(10),
    )
    .width(Length::Fill)
    .height(Length::FillPortion(40));

    dashboard.into()
}

fn render_pie_chart_container() -> Element<'static, Message> {
    let time_container = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(30))
    .height(Length::Fill)
    .center_x()
    .style(dashbord_time_container_style)];

    time_container.into()
}

fn render_bar_chart_container() -> Element<'static, Message> {
    let time_container = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(30))
    .height(Length::Fill)
    .center_x()
    .style(dashbord_time_container_style)];

    time_container.into()
}

fn render_switch_info_container() -> Element<'static, Message> {
    let time_container = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(40))
    .height(Length::Fill)
    .center_x()
    .style(dashbord_time_container_style)];

    time_container.into()
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
