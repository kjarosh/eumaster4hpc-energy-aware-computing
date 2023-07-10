mod matrix_multiplication;

fn main() {
    let n = 500;

    println!("Rayon:");
    matrix_multiplication::multiply_matrix_rayon(n, 1);
    println!("Sequential:");
    matrix_multiplication::multiply_matrix_sequential(n);
}
