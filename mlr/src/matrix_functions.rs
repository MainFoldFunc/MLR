pub fn matrix_add(mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    if mat1.len() != mat2.len()
        || mat1
            .iter()
            .zip(mat2.iter())
            .any(|(r1, r2)| r1.len() != r2.len())
    {
        return Err("Matrix dimensions must be the same".to_string());
    }

    let result: Vec<Vec<f64>> = mat1
        .iter()
        .zip(mat2.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| a + b).collect())
        .collect();

    Ok(result)
}

pub fn matrix_sub(mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    if mat1.len() != mat2.len()
        || mat1
            .iter()
            .zip(mat2.iter())
            .any(|(r1, r2)| r1.len() != r2.len())
    {
        return Err("Matrix dimensions must be the same".to_string());        
    }

    let result: Vec<Vec<f64>> = mat1
        .iter()
        .zip(mat2.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| a - b).collect())
        .collect();

    Ok(result)
}

pub fn matrix_scalar(mat1: &[Vec<f64>], scalar: f64) -> Result<Vec<Vec<f64>>, String> {
    // Ensure all rows have the same length (valid matrix check)
    if mat1.is_empty() || mat1.iter().any(|row| row.len() != mat1[0].len()) {
        return Err("Matrix rows must have the same number of columns.".to_string());
    }

    // Perform scalar multiplication
    let result: Vec<Vec<f64>> = mat1
        .iter()
        .map(|row| row.iter().map(|&a| a * scalar).collect()) // Multiply each element by scalar
        .collect();

    Ok(result)
}

pub fn matrix_hadamard_prod(mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    if mat1.len() != mat2.len() ||
        mat1
            .iter()
            .zip(mat2.iter())
            .any(|(r1, r2)| r1.len() != r2.len())

    {
        return Err("Matrix dimensions must be thes same".to_string());    
    }

    let result: Vec<Vec<f64>> = mat1
        .iter()
        .zip(mat2.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter()).map(|(a, b)| a * b).collect())
        .collect();


    Ok(result)
}

pub fn matrix_dot_prod(mat1: &[Vec<f64>], mat2: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    if mat1[0].len() != mat2.len() {
        return Err("Matrix dimensions are incompatible for multiplication".to_string())
    }

    let num_rows = mat1.len();
    let num_cols = mat2[0].len();
    let num_common = mat2.len();


    let mut result: Vec<Vec<f64>> = vec![vec![0.0; num_cols]; num_rows];


    for i in 0..num_rows {
        for j in 0..num_cols {
            for k in 0..num_common {
                result[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }

    Ok(result)
}

pub fn matrix_transpose(mat1: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let num_rows = mat1.len();
    let num_cols = mat1[0].len();

    let mut result: Vec<Vec<f64>> = vec![vec![0.0;num_rows];num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            result[i][j] = mat1[j][i]
        }
    }

    result
}

pub fn matrix_determinant(matrix: &[Vec<f64>]) -> f64 {
    let n = matrix.len();
    if n == 1 {
        return matrix[0][0]
    }
    if n == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
    }

    let mut det = 0.0;

    for i in 0..n {
        let mut sub_matrix = vec![vec![0.0;n-1];n-1];
        for j in 1..n {
            let mut col_index = 0;
            for k in 0..n {
                if k != i {
                    sub_matrix[j - 1][col_index] = matrix[j][k];
                    col_index += 1;
                }
            }
        }

        let sign = if i % 2 == 0 {1.0} else {-1.0};
        det += sign * matrix[0][i] * matrix_determinant(&sub_matrix);
    }

    det
}

pub fn matrix_cofactor(matrix: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = matrix.len();
    let mut cofactor_matrix = vec![vec![0.0;n];n];

    for i in 0..n {
        for j in 0..n {
            let mut sub_matrix = vec![vec![0.0;n - 1];n - 1];
            let mut sub_i = 0;
            for k in 0..n {
                if k != i {
                    let mut sub_j = 0;
                    for l in 0..n {
                        if l != j {
                            sub_matrix[sub_i][sub_j] = matrix[k][l];
                            sub_j += 1;
                        }
                    }

                    sub_i += 1;
                }
            }

            let sign = if (i + j) % 2 == 0 {1.0} else {-1.0};
            cofactor_matrix[i][j] = sign * matrix_determinant(&sub_matrix);
        }
    }

    cofactor_matrix
}

pub fn matrix_adjugate(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = matrix.len();

    if matrix.iter().any(|row| row.len() != n) {
        return Err("Matrix must be square to calculate the adjugate".to_string())
    }

    let cofactor = matrix_cofactor(matrix);
    let adjugate_matrix = matrix_transpose(&cofactor);

    Ok(adjugate_matrix)
}

pub fn matrix_inverse(matrix: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, String> {
    let n = matrix.len();

    if matrix.iter().any(|row| row.len() != n) {
        return Err("Matrix must be square to calculate the inverse".to_string());
    }

    let det = matrix_determinant(matrix);

    if det == 0.0 {
        return Err("Matrix is singular and does not have an inverse".to_string());
    }

    let adjuge = matrix_adjugate(matrix)?;

    let inverse: Vec<Vec<f64>> = adjuge
        .iter()
        .map(|row| row.iter().map(|&val1| val1 / det).collect())
        .collect();

    Ok(inverse)
}
pub fn test_matrix() {
    // Test matrices for operations
    let mat1 = vec![
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    ];

    let mat2 = vec![
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 3.0, 6.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
    ];


    println!("Testing matrix addition...");
    match matrix_add(&mat1, &mat2) {
        Ok(result) => println!("Matrix Addition Result: {:?}", result),
        Err(e) => println!("Matrix Addition Error: {}", e),
    }

    println!("Testing matrix subtraction...");
    match matrix_sub(&mat1, &mat2) {
        Ok(result) => println!("Matrix Subtraction Result: {:?}", result),
        Err(e) => println!("Matrix Subtraction Error: {}", e),
    }

    println!("Testing matrix scalar multiplication...");
    let scalar = 2.0;
    match matrix_scalar(&mat1, scalar) {
        Ok(result) => println!("Matrix Scalar Multiplication Result: {:?}", result),
        Err(e) => println!("Matrix Scalar Multiplication Error: {}", e),
    }

    println!("Testing matrix Hadamard product...");
    match matrix_hadamard_prod(&mat1, &mat2) {
        Ok(result) => println!("Matrix Hadamard Product Result: {:?}", result),
        Err(e) => println!("Matrix Hadamard Product Error: {}", e),
    }

    println!("Testing matrix dot product...");
    match matrix_dot_prod(&mat1, &mat2) {
        Ok(result) => println!("Matrix Dot Product Result: {:?}", result),
        Err(e) => println!("Matrix Dot Product Error: {}", e),
    }

    println!("Testing matrix transpose...");
    let transposed = matrix_transpose(&mat1);
    println!("Matrix Transpose Result: {:?}", transposed);

    println!("Testing matrix determinant...");
    let determinant = matrix_determinant(&mat1);
    println!("Matrix Determinant Result: {}", determinant);

    println!("Testing matrix cofactor...");
    let cofactor = matrix_cofactor(&mat1);
    println!("Matrix Cofactor Result: {:?}", cofactor);

    println!("Testing matrix adjugate...");
    match matrix_adjugate(&mat1) {
        Ok(result) => println!("Matrix Adjugate Result: {:?}", result),
        Err(e) => println!("Matrix Adjugate Error: {}", e),
    }

    println!("Testing matrix inverse...");
    match matrix_inverse(&mat1) {
        Ok(result) => println!("Matrix Inverse Result: {:?}", result),
        Err(e) => println!("Matrix Inverse Error: {}", e),
    }
}

pub fn test_vid() {
    let matrix: Vec<Vec<f64>> = vec![
        vec![11.0, 12.0],
        vec![4.0, 8.0]
    ];
    match matrix_inverse(&matrix) {
        Ok(result) => println!("Matrix inverse: {:?}", result),
        Err(e) => println!("MAtrix inverse Error: {}", e),
    }
}
