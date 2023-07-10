use std::time::Instant;
use array2d::Array2D;
use rayon::prelude::*;

pub fn multiply_matrix_rayon(n: usize, parallelism: usize) {
    rayon::ThreadPoolBuilder::new().num_threads(parallelism).build_global().unwrap();

    let mut matrix1 = Array2D::filled_with(0., n, n);
    let mut matrix2 = Array2D::filled_with(0., n, n);
    for i in 0..n {
        for j in 0..n {
            matrix1[(i, j)] = rand::random();
            matrix2[(i, j)] = rand::random();
        }
    }

    let start = Instant::now();

    let coordinates: Vec<(usize, usize)> = (0..n)
        .flat_map(|y|
            (0..n)
                .clone()
                .map(move |x| (x, y)))
        .collect();

    let matrix_result = coordinates.par_iter()
        .map(|(x, y)| {
            let value = multiply(&matrix1, &matrix2, x.clone(), y.clone());
            (x.clone(), y.clone(), value)
        })
        .collect::<Vec<_>>();

    println!("Results len: {:?}", matrix_result.len());
    println!("Time: {:?}", start.elapsed());
}

pub fn multiply_matrix_sequential(n: usize) {
    let mut matrix1 = Array2D::filled_with(0., n, n);
    let mut matrix2 = Array2D::filled_with(0., n, n);
    let mut matrix_result = Array2D::filled_with(0., n, n);
    for i in 0..n {
        for j in 0..n {
            matrix1[(i, j)] = rand::random();
            matrix2[(i, j)] = rand::random();
        }
    }

    let start = Instant::now();

    let coordinates: Vec<(usize, usize)> = (0..n)
        .flat_map(|y|
            (0..n)
                .clone()
                .map(move |x| (x, y)))
        .collect();

    for (x, y) in coordinates {
        matrix_result[(x, y)] = multiply(&matrix1, &matrix2, x, y);
    }

    println!("Results len: {:?}", matrix_result.num_elements());
    println!("Time: {:?}", start.elapsed());
}

fn multiply(a: &Array2D<f64>, b: &Array2D<f64>, x: usize, y: usize) -> f64 {
    let mut value: f64 = 0.;

    assert_eq!(a.num_columns(), b.num_rows());

    for i in 0..a.num_columns() {
        value += a[(i, y)] * b[(x, i)]
    }

    return value;
}
