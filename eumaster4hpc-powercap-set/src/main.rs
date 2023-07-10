use std::env;
use std::fs;
use std::process::exit;
use regex::Regex;

fn main() {
    let regex = Regex::new(r"/sys/class/powercap/intel-rapl:\d/constraint_\d_power_limit_uw").unwrap();

    let file = env::args().nth(1).expect("Missing file");
    let data = env::args().nth(2).expect("Missing data");

    if !regex.is_match(&file) {
        panic!("Unexpected file path");
    }

    fs::write(file, data).expect("Unable to set powercap");
}
