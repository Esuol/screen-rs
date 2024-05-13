use charts::draw_bar_chart;
use plotters::style::RGBColor;
use std::path::Path;

pub fn generate_all_plots() {
    draw_bar_chart_one()
}

fn draw_bar_chart_one() {
    let width = 800;
    let height = 600;
    let color = RGBColor(255, 0, 0); // 红色
    let background = RGBColor(255, 255, 255); // 白色
    let data = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let output_path = Path::new("images/bar_chart_one.png");
    let result = draw_bar_chart(width, height, color, background, data, output_path);

    match result {
        Ok(path) => println!("Bar chart saved to: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}
