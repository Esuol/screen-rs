use crate::Message;
use crate::State;
use crate::{render_bar_charts, render_dashboard, render_pie_charts};
use iced::widget::{canvas, column, container, horizontal_space, row, text};
use iced::{Alignment, Color, Element, Length};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Views {
    pub title: &'static str,
    pub view: fn(time: String) -> Element<'static, Message>,
}

impl Views {
    const LIST: &'static [Self] = &[
        Self {
            title: "First-Screen",
            view: render_first_screen,
        },
        Self {
            title: "Second-Screen",
            view: render_first_screen,
        },
    ];

    pub fn is_first(self) -> bool {
        Self::LIST.first() == Some(&self)
    }

    pub fn is_last(self) -> bool {
        Self::LIST.last() == Some(&self)
    }

    pub fn previous(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST
            .get(index.saturating_sub(1))
            .copied()
            .unwrap_or(self)
    }

    pub fn next(self) -> Self {
        let Some(index) = Self::LIST.iter().position(|&example| example == self) else {
            return self;
        };

        Self::LIST.get(index + 1).copied().unwrap_or(self)
    }

    pub fn view(&self, time: String) -> Element<Message> {
        (self.view)(time)
    }
}

impl Default for Views {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn render_first_screen<'a>(time: String) -> Element<'a, Message> {
    // 太阳
    let solar = canvas(State::new()).width(60).height(36);

    let header_title = "YUANQU HSE";
    let header_text = text(header_title)
        .size(28)
        .color(Color::from_rgb8(42, 163, 199));
    // 时间
    let date_result = format!("{} {}", "", time);
    let current_date = text(date_result)
        .size(16)
        .color(Color::from_rgb8(42, 163, 199));

    let header = container(
        row![
            solar,
            horizontal_space(),
            header_text,
            horizontal_space(),
            current_date,
        ]
        .padding(10)
        .align_items(Alignment::Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default().with_border(palette.background.strong.color, 1)
    })
    .height(60);

    let dashboard = render_dashboard();

    let bar_charts = render_bar_charts();

    let pie_charts = render_pie_charts();

    container(
        column![header, dashboard, bar_charts, pie_charts]
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::Fill),
    )
    .height(Length::Fill)
    .padding(10)
    .into()
}
