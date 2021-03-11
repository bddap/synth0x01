/// a linear function of x intersecting the provided points
pub fn lin(x1: f64, y1: f64, x2: f64, y2: f64, x: f64) -> f64 {
    let slope = (y2 - y1) / (x2 - x1);
    let yintercept = y1 - slope * x1;
    x * slope + yintercept
}

/// transforms a number x from range (inmin, inmax) to range (outmin, outmax).
pub fn map(x: f64, inmin: f64, inmax: f64, outmin: f64, outmax: f64) -> f64 {
    (x - inmin) / (inmax - inmin) * (outmax - outmin) + outmin
}

#[cfg(feature = "plotters")]
pub fn plot(samples: impl Iterator<Item = f64> + Clone) {
    use plotters::prelude::*;

    let len = samples.clone().count();

    let root = BitMapBackend::new("dump.png", (640 * 2, 480 * 2)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..(len as f64), -2.0..2.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(LineSeries::new(
            samples.enumerate().map(|(i, s)| (i as f64, s)),
            &RED,
        ))
        .unwrap();

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    println!("dumped!");
}
