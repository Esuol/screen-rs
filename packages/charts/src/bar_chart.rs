use plotters::prelude::*;
use plotters::style::{RGBAColor, RGBColor};

// 绘制柱状图并返回图像
pub fn draw_bar_chart(
    width: u32,
    height: u32,
    color: RGBColor,
    background: RGBAColor,
    data: &[u32],
    output_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_path, (width, height)).into_drawing_area();

    root.fill(&background)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(
            "Hidden danger rectification rate",
            ("sans-serif", 60.0)
                .into_font()
                .color(&RGBColor(42, 163, 199)),
        )
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..10u32)?;

    chart
        .configure_mesh()
        .light_line_style(&WHITE) // 设置轻网格线的颜色
        .bold_line_style(&WHITE) // 设置重网格线的颜色
        .disable_x_mesh()
        .disable_y_mesh()
        .axis_style(&WHITE) // 设置轴
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(
            ("sans-serif", 30)
                .into_font()
                .color(&RGBColor(42, 163, 199)),
        )
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(color.filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    println!("Result has been saved to {}", output_path);

    Ok(output_path.to_string())
}
