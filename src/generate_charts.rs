use charts::draw_bar_chart;
use plotters::style::{RGBAColor, RGBColor};

pub fn generate_all_plots() {
    draw_bar_chart_one();
    draw_dashboard_one();
}

fn draw_bar_chart_one() {
    let width = 800;
    let height = 600;
    let color = RGBColor(137, 245, 168); // 绿色
    let background = RGBAColor(255, 255, 255, 0.0); // 透明底色
    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 8,
    ];

    let result = draw_bar_chart(
        width,
        height,
        color,
        background,
        &data,
        "images/bar_chart_one.png",
    );

    match result {
        Ok(path) => println!("Bar bar_chart_one: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn draw_dashboard_one() {}
