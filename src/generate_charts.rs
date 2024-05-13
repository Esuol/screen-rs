use charts::{draw_bar_chart, draw_line_c_chart, draw_line_chart, draw_line_d_chart};
use plotters::style::{RGBAColor, RGBColor};

pub fn generate_all_plots() {
    draw_bar_chart_one();
    draw_bar_chart_two();
    draw_bar_chart_three();
    draw_line_chart_one();
    draw_line_ds_chart();
    draw_line_cs_chart();
}

fn draw_bar_chart_one() {
    let width = 800;
    let height = 600;
    let color = RGBColor(16, 136, 212); // 绿色
    let background = RGBAColor(8, 8, 8, 255.0);
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
        "Hidden danger rectification rate",
    );

    match result {
        Ok(path) => println!("Bar bar_chart_one: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn draw_bar_chart_two() {
    let width = 800;
    let height = 600;
    let color = RGBColor(137, 245, 168); // 绿色
    let background = RGBAColor(8, 8, 8, 255.0);
    let data = [
        0u32, 2, 8, 1, 8, 2, 5, 4, 8, 5, 1, 2, 3, 6, 7, 2, 1, 2, 4, 5, 6, 1,
    ];

    let result = draw_bar_chart(
        width,
        height,
        color,
        background,
        &data,
        "images/bar_chart_two.png",
        "Hidden danger status",
    );

    match result {
        Ok(path) => println!("Bar bar_chart_two: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn draw_bar_chart_three() {
    let width = 800;
    let height = 600;
    let color = RGBColor(254, 174, 17);
    let background = RGBAColor(8, 8, 8, 255.0);
    let data = [0u32, 8, 2, 6, 8];

    let result = draw_bar_chart(
        width,
        height,
        color,
        background,
        &data,
        "images/bar_chart_three.png",
        "Homework completion status",
    );

    match result {
        Ok(path) => println!("Bar bar_chart_three: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn draw_line_chart_one() {
    let background = RGBAColor(8, 8, 8, 255.0);
    let one_line_color = RGBColor(254, 174, 17);
    let two_line_color = RGBColor(16, 136, 212); // 绿色
    let _ = draw_line_chart(
        "Hidden danger reporting situation",
        "images/line-chart-one.png",
        background,
        one_line_color,
        two_line_color,
    );
}

pub fn draw_line_ds_chart() {
    let color = RGBColor(16, 136, 212);
    let dot_color = RGBColor(254, 174, 17);
    let background = RGBAColor(8, 8, 8, 255.0);
    let _ = draw_line_d_chart(
        "Hidden removal situation",
        "images/line-d-chart.png",
        background,
        color,
        dot_color,
    );
}

pub fn draw_line_cs_chart() {
    let color = RGBColor(16, 136, 212);
    let dot_color = RGBColor(254, 174, 17);
    let background = RGBAColor(8, 8, 8, 255.0);
    let _ = draw_line_c_chart(
        "Hidden addition situation",
        "images/line-c-chart.png",
        background,
        color,
        dot_color,
    );
}
