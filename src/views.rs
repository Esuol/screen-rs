use crate::Message;
use iced::mouse;
use iced::widget::{canvas, column, container, horizontal_space, row, scrollable};
use iced::{Alignment, Element, Length, Point, Rectangle, Renderer, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Views {
    pub title: &'static str,
    pub view: fn() -> Element<'static, Message>,
}

impl Views {
    const LIST: &'static [Self] = &[
        Self {
            title: "rs-big-data-screen-1",
            view: application,
        },
        Self {
            title: "rs-big-data-screen-2",
            view: application,
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

    pub fn view(&self) -> Element<Message> {
        (self.view)()
    }
}

impl Default for Views {
    fn default() -> Self {
        Self::LIST[0]
    }
}

fn application<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            square(40),
        ]
        .padding(10)
        .align_items(Alignment::Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default().with_border(palette.background.strong.color, 1)
    });

    let sidebar = container(
        column!["Sidebar!", square(50), square(50)]
            .spacing(40)
            .padding(10)
            .width(200)
            .align_items(Alignment::Center),
    )
    .style(container::rounded_box)
    .height(Length::Fill)
    .center_y();

    let content = container(
        scrollable(
            column!["Content!", square(400), square(200), square(400), "The end"]
                .spacing(40)
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .height(Length::Fill),
    )
    .padding(10);

    column![header, row![sidebar, content]].into()
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}
