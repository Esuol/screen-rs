use crate::Message;
use iced::alignment::{self};
use iced::widget::{column, container, image, row, text};
use iced::{Color, Element, Length, Padding};

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
    let url = "images/line-chart-one.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(30))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}

fn render_bar_chart_container() -> Element<'static, Message> {
    let url = "images/bar_chart_three.png";
    let img = image::Handle::from_path(url);
    let time_container = row![container(column![image(img)
        .width(Length::Fill)
        .height(Length::Fill)])
    .width(Length::FillPortion(30))
    .height(Length::Fill)
    .center_x()];

    time_container.into()
}

fn render_switch_info_container() -> Element<'static, Message> {
    let dot_color = Color::from_rgb8(254, 174, 17);
    let color = Color::from_rgb8(16, 136, 212);
    let a_txt = render_txt("To be both a speaker of words and a doer of deeds.", color);
    let b_txt = render_txt(
        "The shortest way to do many things is to only one thing at a time.",
        color,
    );
    let c_tex = render_txt("Variety is the spice of life.", color);
    let d_txt = render_txt("Bad times make a good man.", dot_color);
    let e_txt = render_txt("There is no royal road to learning.", color);
    let f_txt = render_txt("Doubt is the key to knowledge.", color);
    let g_txt = render_txt(
        "Life is fine and enjoyable, yet you must learn to enjoy your fine life.",
        color,
    );
    let h_txt = render_txt("A man's best friends are his ten fingers.", color);
    let i_txt = render_txt(
        "Life is the art of drawing sufficient conclusions form insufficient premises.",
        color,
    );

    let time_container = row![container(column![
        a_txt, b_txt, c_tex, d_txt, e_txt, f_txt, g_txt, h_txt, i_txt
    ])
    .width(Length::FillPortion(40))
    .height(Length::Fill)
    .center_x()
    .padding(Padding {
        top: 0.0,
        right: 0.0,
        bottom: 10.0,
        left: 0.0
    })];

    time_container.into()
}

fn render_txt(content: &str, color: Color) -> Element<'static, Message> {
    let header_text = text(content.to_string())
        .size(13)
        .width(Length::Fill)
        .height(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Left)
        .color(color);

    header_text.into()
}
