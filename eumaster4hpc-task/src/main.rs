use std::env;
use std::process::exit;

mod matrix_multiplication;

fn main() {
    let task_type = env::args()
        .nth(1).expect("Missing task type");
    let task_size = env::args()
        .nth(2).expect("Missing task size")
        .parse::<usize>().unwrap();

    match task_type.as_ref() {
        "sequential" => {
            matrix_multiplication::multiply_matrix_sequential(task_size);
        }
        "rayon" => {
            let parallelism = env::args()
                .nth(3).expect("Missing parallelism")
                .parse::<usize>().unwrap();

            matrix_multiplication::multiply_matrix_rayon(task_size, parallelism);
        }
        _ => {
            println!("Unknown task type");
            exit(1);
        }
    }
}
