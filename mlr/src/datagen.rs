use rand::Rng;

pub fn generate_100_samples() -> Vec<f64> {
    let mut arr_x = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let x = rng.gen_range(0.0..10.0); // Generates an integer in the range [0, 10)
        arr_x.push(x);
    }
    arr_x
}

pub fn gen_y(arr_x: Vec<f64>) -> Vec<f64> {
    let mut arr_y = Vec::new();
    let mut rng = rand::thread_rng();
    for &x in &arr_x {
        let noise = rng.gen_range(0.0..2.0); // This will give a noise of 0 or 1
        let y = 2.0 * x + 3.0 + noise; // y = 2x + 3 + noise
        arr_y.push(y);
    }
    arr_y
}
