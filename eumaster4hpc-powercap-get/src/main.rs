use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"/sys/class/powercap/intel-rapl:\d/energy_uj").unwrap();

    let file = env::args().nth(1).expect("Missing file");

    if !regex.is_match(&file) {
        panic!("Unexpected file path");
    }

    let value = fs::read_to_string(file).expect("Unable to read");
    print!("{}", value);
}
