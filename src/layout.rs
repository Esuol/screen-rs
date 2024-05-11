use crate::views::Views;
use crate::{dashboard, render_bar_charts};
use iced::keyboard;
use iced::widget::{button, checkbox, column, container, horizontal_space, pick_list, row, text};
use iced::{color, Alignment, Element, Font, Length, Subscription, Theme};

#[derive(Default, Debug)]
pub struct Layout {
    pub views: Views,
    pub explain: bool,
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    Previous,
    ExplainToggked(bool),
    ThemeSelected(Theme),
}

impl Layout {
    pub fn title(&self) -> String {
        format!("{} - Layout - Iced", self.views.title)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.views = self.views.next();
            }
            Message::Previous => {
                self.views = self.views.previous();
            }
            Message::ExplainToggked(explain) => {
                self.explain = explain;
            }
            Message::ThemeSelected(theme) => {
                self.theme = theme;
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, _modifiers| match key {
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Message::Previous),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
            _ => None,
        })
    }

    pub fn view(&self) -> Element<Message> {
        let header = row![
            text(self.views.title).size(20).font(Font::MONOSPACE),
            horizontal_space(),
            checkbox("Explain", self.explain).on_toggle(Message::ExplainToggked),
            pick_list(Theme::ALL, Some(&self.theme), Message::ThemeSelected),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        let dashboard = dashboard::render_dashboard();

        let bar_charts = render_bar_charts();

        let example = container(if self.explain {
            self.views.view().explain(color!(0x0000ff))
        } else {
            self.views.view()
        })
        .style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default().with_border(palette.background.strong.color, 4.0)
        })
        .padding(4)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y();

        let controls = row([
            (!self.views.is_first()).then_some(
                button("← Previous")
                    .padding([5, 10])
                    .on_press(Message::Previous)
                    .into(),
            ),
            Some(horizontal_space().into()),
            (!self.views.is_last()).then_some(
                button("Next →")
                    .padding([5, 10])
                    .on_press(Message::Next)
                    .into(),
            ),
        ]
        .into_iter()
        .flatten());

        column![header, dashboard, bar_charts, example, controls]
            .spacing(10)
            .padding(20)
            .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
