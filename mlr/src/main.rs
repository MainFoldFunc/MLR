mod datagen;
mod matrix_functions;
mod matrix_with_vector_functions;
mod vectors_functions;
fn main() {
    //vectors_functions::test_vec();
    //matrix_functions::test_matrix();
    // matrix_functions::test_vid();
    let data: Vec<i32> = datagen::generate_100_samples();
    let data_y: Vec<i32> = datagen::gen_y(data.clone());
    println!("{:?}\n{:?}", data, data_y);
}
