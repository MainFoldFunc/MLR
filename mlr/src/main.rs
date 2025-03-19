mod datagen;
mod matrix_functions;
mod matrix_with_vector_functions;
mod simple_li_reg_model;
mod vectors_functions;
fn main() {
    //vectors_functions::test_vec();
    //matrix_functions::test_matrix();
    // matrix_functions::test_vid();
    let data: Vec<f64> = datagen::generate_100_samples();
    let data_y: Vec<f64> = datagen::gen_y(data.clone());

    println!("{:?}\n{:?}", data, data_y);

    let model = simple_li_reg_model::LinRegModel::new(); // Corrected module call
    let predict_y: Vec<f64> = model.predict(data.clone());

    let how_much_good = simple_li_reg_model::test_model(data_y.clone(), predict_y.clone());

    println!("Numbers of guessed: {}", how_much_good);
}
