use rand::Rng;

pub struct LinRegModel {
    w: f64,
    b: f64,
}

impl LinRegModel {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            w: rng.gen_range(-1.0..1.0),
            b: rng.gen_range(-1.0..1.0),
        }
    }

    pub fn predict(&self, x: Vec<f64>) -> Vec<f64> {
        let mut ret: Vec<f64> = Vec::new();
        for &y in &x {
            // Use reference so `x` isn't consumed
            ret.push(self.w * y + self.b);
        }
        ret
    }
}
pub fn test_model(correct: Vec<f64>, guessed: Vec<f64>) -> i32 {
    let mut score: i32 = 0;

    for (corr, guess) in correct.iter().zip(guessed.iter()) {
        if (corr - guess).abs() < 1.0 {
            // Allow a small margin of error
            score += 1;
        }
    }

    score
}

pub fn MSE(correct: Vec<f64>, guessed: Vec<f64>) -> f64 {
    let lenght = correct.len();
    if lenght == 0 {
        return 0.0;
    }

    let mut sum_error = 0.0;
    for (y_true, y_pred) in correct.iter().zip(guessed.iter()) {
        let error = y_true - y_pred;
        sum_error += error * error;
    }

    sum_error / (lenght as f64)
}
