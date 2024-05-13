use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{stroke, Cache, Geometry, LineCap, Path, Stroke};
use iced::{Degrees, Point, Rectangle, Renderer, Theme, Vector};

pub struct Dashboard {
    now: u8,
    clock: Cache,
}

impl Dashboard {
    pub fn new(now: u8) -> Self {
        Self {
            now,
            clock: Cache::default(),
        }
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self {
            now: 15,
            clock: Cache::default(),
        }
    }
}

impl<Message> canvas::Program<Message> for Dashboard {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let clock = self.clock.draw(renderer, bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, iced::Color::from_rgb8(0x01, 0x1D, 0x42));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let width = radius / 100.0;

            let wide_stroke = || -> Stroke {
                Stroke {
                    width: width * 5.0,
                    style: stroke::Style::Solid(iced::Color::from_rgb8(0x9A, 0x1C, 0x31)),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            let scale_stroke = || -> Stroke {
                Stroke {
                    width: width * 5.0,
                    style: stroke::Style::Solid(iced::Color::from_rgb8(60, 208, 210)),
                    line_cap: LineCap::Round,
                    ..Stroke::default()
                }
            };

            frame.translate(Vector::new(center.x, center.y));

            // Draw hour marks
            for i in 0..12 {
                frame.with_save(|frame| {
                    frame.rotate(Degrees(30.0 * i as f32));
                    let mark = Path::line(
                        Point::new(0.0, -0.7 * radius),
                        Point::new(0.0, -0.8 * radius),
                    );
                    frame.stroke(&mark, scale_stroke());
                });
            }

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now, 60));
                frame.stroke(&long_hand, wide_stroke());
            });
        });

        vec![clock]
    }
}

fn hand_rotation(n: u8, total: u8) -> Degrees {
    let turns = n as f32 / total as f32;

    Degrees(360.0 * turns)
}
