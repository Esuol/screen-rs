use crate::Message;
use iced::widget::{column, container, image, row};
use iced::{Element, Length};

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
    let url = "images/line-c-chart.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(30))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}

fn render_pie_chart_container() -> Element<'static, Message> {
    let url = "images/line-d-chart.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(23))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}
