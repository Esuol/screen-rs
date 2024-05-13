use crate::Dashboard;
use crate::Message;
use iced::alignment::{self, Alignment};
use iced::widget::{canvas, column, container, horizontal_space, row, text};
use iced::Padding;
use iced::{Border, Color, Element, Length, Theme};

pub fn render_dashboard(time: String) -> Element<'static, Message> {
    let dashboard =
        container(row![render_time_container(time), render_dashboards_container()].spacing(10))
            .width(Length::Fill)
            .height(Length::FillPortion(20));

    dashboard.into()
}

fn render_time_container(time: String) -> Element<'static, Message> {
    // 时间
    let date_result = format!("{} {}", "", time);
    let current_date = text(date_result)
        .size(22)
        .color(Color::from_rgb8(255, 255, 255))
        .width(Length::Fill)
        .height(Length::FillPortion(30))
        .horizontal_alignment(alignment::Horizontal::Left)
        .vertical_alignment(alignment::Vertical::Center);

    // 副标题
    let header_title = "MARCH2024";
    let header_text = text(header_title)
        .size(16)
        .color(Color::from_rgb8(42, 163, 199))
        .width(Length::Fill)
        .height(Length::FillPortion(30))
        .horizontal_alignment(alignment::Horizontal::Right);

    // 数字容器 A
    let number_a = "20";
    let number_text_a = text(number_a)
        .size(20)
        .color(Color::from_rgb8(255, 195, 21))
        .width(40)
        .height(36)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center);
    let number_a_title = "WORK";
    let number_a_text = text(number_a_title)
        .size(24)
        .color(Color::from_rgb8(42, 163, 199))
        .width(90)
        .horizontal_alignment(alignment::Horizontal::Right)
        .vertical_alignment(alignment::Vertical::Center);
    let container_number_a = container(row![
        container(row![number_text_a])
            .width(Length::FillPortion(100))
            .height(Length::FillPortion(40))
            .center_x()
            .padding(4)
            .style(number_container_style),
        container(horizontal_space()),
        number_a_text
    ])
    .width(Length::Fill)
    .height(60)
    .padding(8);

    // container
    let time_container = row![
        container(column![current_date, header_text, container_number_a,])
            .width(Length::FillPortion(15))
            .height(Length::Fill)
            .center_x()
            .style(dashbord_time_container_style)
            .padding(Padding {
                top: 10.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0
            }),
    ];

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

pub fn dashbord_time_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Color::TRANSPARENT.into()),
        border: Border {
            width: 0.0,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn number_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(iced::Color::TRANSPARENT.into()),
        border: Border {
            width: 3.0,
            color: iced::Color::from_rgb8(32, 168, 230),
            radius: [6.0, 6.0, 6.0, 6.0].into(),
            ..Border::default()
        },
        ..Default::default()
    }
}

// 生成仪表盘
