pub fn generate_all_plots() {
    // 这里假设你有一个名为 `generate_plot` 的函数，它接受一个输出路径并在该路径下生成一个图片
    let output_paths = vec![
        Path::new("images/image1.png"),
        Path::new("images/image2.png"),
        // 添加更多的路径...
    ];

    for output_path in output_paths {
        let result = generate_plot(output_path);

        if let Err(e) = result {
            eprintln!("Error generating plot: {}", e);
        }
    }
}
