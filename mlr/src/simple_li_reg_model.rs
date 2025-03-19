use rand::Rng;

pub struct LinRegModel {
    pub w: f64, // ✅ Correct: Make field public here
    pub b: f64, // ✅ Correct
}

impl LinRegModel {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            w: rng.gen_range(-1.0..1.0),
            b: rng.gen_range(-1.0..1.0),
        }
    }

    pub fn predict(&self, x: &Vec<f64>) -> Vec<f64> {
        let mut ret: Vec<f64> = Vec::new();
        for &y in x.iter() {
            ret.push(self.w * y + self.b);
        }
        ret
    }

    pub fn train(&mut self, x: &Vec<f64>, y: &Vec<f64>, learning_rate: f64, epochs: usize) {
        let n = x.len();
        if n == 0 {
            return; // Avoid training on empty data
        }

        for _ in 0..epochs {
            let mut dw = 0.0; // Gradient for w
            let mut db = 0.0; // Gradient for b

            // Compute gradients
            for (x_i, y_i) in x.iter().zip(y.iter()) {
                let y_pred = self.w * x_i + self.b; // Predicted value
                let error = y_i - y_pred;

                dw += -2.0 * x_i * error;
                db += -2.0 * error;
            }

            // Average gradients
            dw /= n as f64;
            db /= n as f64;

            // Update parameters
            self.w -= learning_rate * dw;
            self.b -= learning_rate * db;
        }
    }
}

// Function to check how many predictions are within an error margin
pub fn test_model(correct: &Vec<f64>, guessed: &Vec<f64>) -> i32 {
    let mut score: i32 = 0;
    for (corr, guess) in correct.iter().zip(guessed.iter()) {
        if (corr - guess).abs() < 0.1 {
            // Allow a small margin of error
            score += 1;
        }
    }
    score
}

// Mean Squared Error function
pub fn mse(correct: &Vec<f64>, guessed: &Vec<f64>) -> f64 {
    let length = correct.len();
    if length == 0 {
        return 0.0;
    }

    let mut sum_error = 0.0;
    for (y_true, y_pred) in correct.iter().zip(guessed.iter()) {
        let error = y_true - y_pred;
        sum_error += error * error;
    }

    sum_error / (length as f64)
}
