mod datagen;
mod simple_li_reg_model;
mod visualization; // Import the visualization module

fn main() {
    // Generate dataset
    let data_x: Vec<f64> = datagen::generate_100_samples();
    let data_y: Vec<f64> = datagen::gen_y(data_x.clone());

    // Initialize model
    let mut model = simple_li_reg_model::LinRegModel::new();

    // Before training: compute initial MSE
    let initial_loss = simple_li_reg_model::mse(&data_y, &model.predict(&data_x));
    println!("Initial MSE: {:.5}", initial_loss);

    // Train model
    model.train(&data_x, &data_y, 0.01, 100000);

    // After training: compute final MSE
    let final_loss = simple_li_reg_model::mse(&data_y, &model.predict(&data_x));
    println!("Final MSE: {:.5}", final_loss);

    println!("Trained parameters: w = {:.5}, b = {:.5}", model.w, model.b);

    // Check how many predictions are close to the real values
    let correct_predictions = simple_li_reg_model::test_model(&data_y, &model.predict(&data_x));
    println!("Numbers of close predictions: {}", correct_predictions);

    // Visualize predictions vs actual values
    let predictions = model.predict(&data_x);
    visualization::plot_predictions_vs_actual(&data_x, &data_y, &predictions);
}
