use crate::Message;
use iced::widget::{column, container, horizontal_space, image, row, text};
use iced::{Border, Element, Length, Theme};

pub fn render_bar_charts() -> Element<'static, Message> {
    let dashboard = container(
        row![
            render_line_chart_container(),
            render_bar_chart_container(),
            render_pie_chart_container(),
            render_bar_cute_chart_container(),
        ]
        .spacing(10),
    )
    .width(Length::Fill)
    .height(Length::FillPortion(30));

    dashboard.into()
}

fn render_bar_chart_container() -> Element<'static, Message> {
    let url = "images/bar_chart_one.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(23))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}

fn render_bar_cute_chart_container() -> Element<'static, Message> {
    let url = "images/bar_chart_two.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(23))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}

fn render_line_chart_container() -> Element<'static, Message> {
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

fn render_pie_chart_container() -> Element<'static, Message> {
    let time_container = row![container(column![
        text("Time").size(20),
        horizontal_space(),
        text("00:00:00"),
    ])
    .width(Length::FillPortion(23))
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
