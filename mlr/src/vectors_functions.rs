use std::io::{self, Write};
pub fn ask_vector() -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();

    // Ask for the number of elements in the vector
    print!("How many elements in the vector? ");
    io::stdout().flush().unwrap();

    let mut amount = String::new();
    io::stdin().read_line(&mut amount).unwrap();

    // Convert input to i32 and handle possible errors
    let amount: i32 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return vec; // Return the empty vector if parsing fails
        }
    };

    // Loop to get each element from the user
    for i in 0..amount {
        print!("Enter the {} element of the vector: ", i + 1);
        io::stdout().flush().unwrap(); // Ensure the prompt is printed before input

        let mut element = String::new();
        io::stdin().read_line(&mut element).unwrap();

        // Parse the input into a f64 and handle possible errors
        let element: f64 = match element.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number for element {}.", i + 1);
                return vec; // Return the current state of the vector if parsing fails
            }
        };

        // Push the element into the vector
        vec.push(element);
    }

    // Return the filled vector
    vec
}

pub fn vec_wedge_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<f64, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors need to be the same length".to_string());
    }
    if vec1.len() != 2 || vec2.len() != 2 {
        return Err("Vectors need to be 2-dimensional".to_string());
    }

    let result = vec1[0] * vec2[1] - vec1[1] * vec2[0];
    Ok(result)
}

pub fn vec_alg_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<f64, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors need to be the same length".to_string());
    }

    let dot_prod = vec_dot_prod(vec1.clone(), vec2.clone())?; // Clone to avoid ownership issues
    let wedge_prod = vec_wedge_prod(vec1, vec2)?;

    // The geometric product is the sum of dot and wedge product
    Ok(dot_prod + wedge_prod)
}

pub fn vec_tensor_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<Vec<f64>>, String> {
    if vec1.is_empty() || vec2.is_empty() {
        return Err("Vectors cannot be empty".to_string());
    }

    let mut mat_res: Vec<Vec<f64>> = Vec::new();

    for &val1 in &vec1 {
        let mut row: Vec<f64> = Vec::new();
        for &val2 in &vec2 {
            row.push(val1 * val2);
        }
        mat_res.push(row);
    }

    Ok(mat_res)
}

pub fn vec_exterior_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<Vec<f64>>, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length.".to_string());
    }

    let n = vec1.len();
    let mut result = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            let mut bivector = vec![0.0; n];
            bivector[i] = vec1[i] * vec2[j];
            bivector[j] = -vec1[j] * vec2[i];
            result.push(bivector);
        }
    }

    Ok(result)
}

pub fn vec_cross_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<f64>, String> {
    if vec1.len() != 3 || vec2.len() != 3 {
        return Err("Both vectors need to be 3-dimensional.".to_string());
    }

    let cross_prod = vec![
        vec1[1] * vec2[2] - vec1[2] * vec2[1], // x-component
        vec1[2] * vec2[0] - vec1[0] * vec2[2], // y-component
        vec1[0] * vec2[1] - vec1[1] * vec2[0], // z-component
    ];

    Ok(cross_prod)
}

pub fn vec_dot_prod(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<f64, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length".to_string());
    }

    let sum: f64 = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x * y)
        .sum();

    Ok(sum)
}

pub fn vec_add(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<f64>, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length".to_string());
    }

    let result = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x + y)
        .collect();

    Ok(result)
}

pub fn vec_mul(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<f64>, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length".to_string());
    }

    let result = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x * y)
        .collect();

    Ok(result)
}

pub fn vec_sub(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<f64>, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length".to_string());
    }

    let result = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x - y)
        .collect();

    Ok(result)
}

pub fn vec_div(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<f64>, String> {
    if vec1.len() != vec2.len() {
        return Err("Vectors must have the same length".to_string());
    }

    let result = vec1
        .into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x / y)
        .collect();

    Ok(result)
}

pub fn vec_by_scalar(vec: Vec<f64>, scalar: f64) -> Vec<f64> {
    let result = vec.iter().map(|a| a * scalar).collect();

    result
}
pub fn test_vec() {
    let vec_test1 = vec![1.09, 3.89, 7.90];
    let vec_test2 = vec![3.89, 2.90, 1.56];

    match vec_add(vec_test1.clone(), vec_test2.clone()) {
        Ok(result) => println!("{:?} + {:?} = {:?}", vec_test1, vec_test2, result),
        Err(e) => println!("Error: {}", e),
    }
    match vec_mul(vec_test1.clone(), vec_test2.clone()) {
        Ok(result) => println!("{:?} times {:?} = {:?}", vec_test1, vec_test2, result),
        Err(e) => println!("Error: {}", e),
    }
    match vec_sub(vec_test1.clone(), vec_test2.clone()) {
        Ok(result) => println!("{:?} - {:?} = {:?}", vec_test1, vec_test2, result),
        Err(e) => println!("Error: {}", e),
    }
    match vec_div(vec_test1.clone(), vec_test2.clone()) {
        Ok(result) => println!("{:?} / {:?} = {:?}", vec_test1, vec_test2, result),
        Err(e) => println!("Error: {}", e),
    }
    match vec_dot_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(sum) => println!("{:?} dot_prod {:?} = {}", vec_test1, vec_test2, sum),
        Err(e) => println!("Error: {}", e),
    }
    match vec_cross_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(cross_prod) => println!(
            "{:?} cross_prod {:?} = {:?}",
            vec_test1, vec_test2, cross_prod
        ),
        Err(e) => println!("Error: {}", e),
    }
    match vec_exterior_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(result) => println!(
            "{:?} exterior_prod {:?} = {:?}",
            vec_test1, vec_test2, result
        ),
        Err(e) => println!("Error: {}", e),
    }
    match vec_tensor_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(mat_res) => println!(
            "{:?} tensor_prod {:?} = {:?}",
            vec_test1, vec_test2, mat_res
        ),
        Err(e) => println!("Error: {}", e),
    }
    match vec_wedge_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(resoult) => println!("{:?} wedge_prod{:?} = {:?}", vec_test1, vec_test2, resoult),
        Err(e) => println!("Error: {}", e),
    }
    match vec_alg_prod(vec_test1.clone(), vec_test2.clone()) {
        Ok(resoult) => println!("{:?} alg_prod {:?} = {:?}", vec_test1, vec_test2, resoult),
        Err(e) => println!("Error: {}", e),
    }
}
