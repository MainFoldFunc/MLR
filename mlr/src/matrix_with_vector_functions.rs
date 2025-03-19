pub fn matrix_by_vector(matrix: &[Vec<f64>], vector: Vec<f64>) -> Result<Vec<f64>, String> {
    if matrix.is_empty() || matrix[0].len() != vector.len() {
        return Err("Matrix column count must match vector length".to_string());
    }

    let result: Vec<f64> = matrix
        .iter()
        .map(|row| row.iter().zip(&vector).map(|(a, b)| a * b).sum())
        .collect();

    Ok(result)
}
