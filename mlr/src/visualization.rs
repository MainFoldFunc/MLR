// visualization.rs
use plotters::prelude::*;

/// Plots the actual and predicted values and saves the chart as a PNG file.
///
/// # Arguments
///
/// * `x` - A vector containing the x-axis values.
/// * `y_true` - A vector containing the actual y-axis values.
/// * `y_pred` - A vector containing the predicted y-axis values.
///
/// # Returns
///
/// Returns a Result which is Ok if plotting succeeded, or an error otherwise.
pub fn plot_predictions_vs_actual(
    x: &Vec<f64>,
    y_true: &Vec<f64>,
    y_pred: &Vec<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area (800 x 600) for the plot and fill it with white.
    let root = BitMapBackend::new("prediction_vs_actual.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Calculate ranges from the provided data.
    let x_min = x.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = y_true
        .iter()
        .chain(y_pred.iter())
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let y_max = y_true
        .iter()
        .chain(y_pred.iter())
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    // Build a 2D cartesian chart with some margins and axis label areas.
    let mut chart = ChartBuilder::on(&root)
        .caption("Predictions vs Actual Values", ("Arial", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Draw mesh (grid and labels).
    chart.configure_mesh().draw()?;

    // Plot actual values in red.
    chart
        .draw_series(PointSeries::of_element(
            // Map x and y_true into a tuple of (x, y).
            x.iter()
                .zip(y_true.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            5,             // Point size.
            RED.to_rgba(), // Use red color.
            &|coord: (f64, f64), size: u32, style: ShapeStyle| {
                // Create a circle at the coordinate with the given size and style.
                Circle::new(coord, size, style)
            },
        ))?
        .label("Actual")
        .legend(|(x, y)| Circle::new((x, y), 5, RED.filled()));

    // Plot predicted values in blue.
    chart
        .draw_series(PointSeries::of_element(
            // Map x and y_pred into a tuple of (x, y).
            x.iter()
                .zip(y_pred.iter())
                .map(|(&x_val, &y_val)| (x_val, y_val)),
            5,              // Point size.
            BLUE.to_rgba(), // Use blue color.
            &|coord: (f64, f64), size: u32, style: ShapeStyle| Circle::new(coord, size, style),
        ))?
        .label("Predicted")
        .legend(|(x, y)| Circle::new((x, y), 5, BLUE.filled()));

    // Draw the legend on the chart.
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
