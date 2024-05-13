use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::{Cache, Fill, Frame, Path, Stroke, Text};
use iced::{Color, Element, Point, Rectangle, Renderer, Size, Theme, Vector};

pub struct Dashboard {
    size: f32,
    background: Color,
    ticks: Vec<u32>,
    value: u32,
    cache: Cache,
}

impl Dashboard {
    pub fn new(
        size: f32,
        background: Color,
        ticks: Vec<u32>,
        value: u32,
        cache: Cache,
    ) -> Dashboard {
        Dashboard {
            size,
            background,
            ticks,
            value,
            cache,
        }
    }

    pub fn draw_dashboard<'a, Message>(&self) -> Element<'a, Message> {
        canvas(self).into()
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
    ) -> Vec<canvas::Geometry> {
        let mut geometry = Vec::new();

        // 创建一个新的路径来绘制仪表盘
        let mut path = Path::new(|path| {
            // 绘制仪表盘的刻度
            for i in (0..=2000).step_by(500) {
                let angle = i as f32 / 2000.0 * 2.0 * std::f32::consts::PI;
                let from = Point::new(
                    bounds.center().x + 200.0 * angle.cos(),
                    bounds.center().y + 200.0 * angle.sin(),
                );
                let to = Point::new(
                    bounds.center().x + 220.0 * angle.cos(),
                    bounds.center().y + 220.0 * angle.sin(),
                );
                path.move_to(from);
                path.line_to(to);
            }
        });

        // 创建一个新的路径来绘制指针
        let pointer_angle = 700.0 / 2000.0 * 2.0 * std::f32::consts::PI;

        let pointer_to = Point::new(
            bounds.center().x + 200.0 * pointer_angle.cos(),
            bounds.center().y + 200.0 * pointer_angle.sin(),
        );

        // 使用黑色描边来绘制路径
        let stroke = Stroke::default().with_color(Color::BLACK);
        geometry.push(self.cache.draw(renderer, bounds.size(), |frame| {
            // 创建一个新的 Path
            let mut path = Path::new();
            path.move_to(Point::new(50.0, 50.0));
            path.line_to(Point::new(100.0, 100.0));

            // 使用黑色描边来绘制路径
            frame.stroke(&path, &stroke);
        }));
    }
}
