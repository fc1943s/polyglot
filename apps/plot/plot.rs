use plotters::prelude::*;

type Data<'a> = (
    &'a str,
    &'a str,
    &'a str,
    Vec<(&'a str, Vec<f64>, Vec<f64>)>,
);

fn draw_line_plot(
    (caption, x_desc, y_desc, axis): Data,
    path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(&path, (640, 480)).into_drawing_area();
    let _ = root.fill(&RGBColor(20, 20, 20));

    let get_min_max_axis = |axis: &mut dyn Iterator<Item = &f64>| {
        axis.fold((std::f64::MAX, std::f64::MIN), |(min, max), n| {
            (min.min(*n), max.max(*n))
        })
    };

    let (min_x, max_x) = get_min_max_axis(axis.iter().map(|(_, x, _)| x).flatten().by_ref());
    let (min_y, max_y) = get_min_max_axis(axis.iter().map(|(_, _, y)| y).flatten().by_ref());

    let axis: Vec<(&str, Vec<(f64, f64)>)> = axis
        .into_iter()
        .map(|(label, x, y)| {
            (
                label,
                y.into_iter()
                    .enumerate()
                    .map(|(idx, y)| (x[idx], y))
                    .collect(),
            )
        })
        .collect();

    let add_margin = |x, y| (x - (y - x) * 0.03, y + (y - x) * 0.03);
    let (min_x, max_x) = add_margin(min_x, max_x);
    let (min_y, max_y) = add_margin(min_y, max_y);

    let mut cc = ChartBuilder::on(&root)
        .margin(5)
        .caption(caption, &WHITE)
        .set_all_label_area_size(50)
        .build_cartesian_2d(min_x..max_x, min_y..max_y)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .max_light_lines(4)
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .x_desc(x_desc)
        .y_desc(y_desc)
        .label_style(&WHITE)
        .bold_line_style(RGBColor(90, 90, 90))
        .light_line_style(RGBColor(50, 50, 50))
        .axis_style(&WHITE)
        .draw()?;

    let colors = vec![&RED, &BLUE, &YELLOW, &GREEN];

    for (idx, (label, y)) in axis.iter().enumerate() {
        let color = colors[idx % colors.len()];
        cc.draw_series(LineSeries::new(y.clone(), color))?
            .label(*label)
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], color));
    }

    cc.configure_series_labels()
        .border_style(&WHITE)
        .label_font(&WHITE)
        .draw()?;

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = plot::SpiralFileSystem::get_workspace_root();
    let workspace_root = std::path::PathBuf::from(workspace_root.to_string());
    let tmp_spiral_dir = workspace_root.join("target/plot");
    let line_plots_data_dir = tmp_spiral_dir.join("line_data");
    let line_plots_svg_dir = tmp_spiral_dir.join("line_svg");

    std::fs::create_dir_all(&line_plots_data_dir)?;
    std::fs::create_dir_all(&line_plots_svg_dir)?;

    let missing_svg = std::fs::read_dir(&line_plots_data_dir)?
        .filter_map(|file| {
            let file = file.ok()?;
            let path = file.path();
            let hash = path.file_stem()?.to_str()?;
            let svg_path = line_plots_svg_dir.join(format!("{}.svg", hash));
            if !svg_path.exists() {
                Some((std::fs::read_to_string(path).ok()?, svg_path))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for (json, svg_path) in missing_svg {
        println!("plot.main / svg_path: {}", svg_path.display());

        let data: Data = serde_json::from_str(&json)?;
        draw_line_plot(data, svg_path.display().to_string())?;
    }

    Ok(())
}
