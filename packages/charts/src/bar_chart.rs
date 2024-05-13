use image::RgbaImage;
use plotters::prelude::*;
use plotters::style::RGBColor;
use std::path::Path;

// 绘制柱状图并返回图像
pub fn draw_bar_chart(
    width: u32,
    height: u32,
    color: RGBColor,
    background: RGBColor,
    data: &[u32],
    output_path: &Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = vec![0; (width * height * 4) as usize];
    {
        let root =
            BitMapBackend::with_buffer(buffer.as_mut_slice(), (width, height)).into_drawing_area();

        root.fill(&background)?;

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption("Bar Chart", ("sans-serif", 50.0))
            .build_cartesian_2d(0..data.len() as u32, 0u32..10u32)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(background.mix(0.3))
            .y_desc("Count")
            .x_desc("Bucket")
            .axis_desc_style(("sans-serif", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(color.filled())
                .data(data.iter().enumerate().map(|(x, y)| (x as u32, *y))),
        )?;
    }

    let img = RgbaImage::from_raw(width, height, buffer).unwrap();
    img.save(output_path)?;

    Ok(output_path.to_string_lossy().into_owned())
}
