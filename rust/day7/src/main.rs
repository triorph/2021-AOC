use day7::CrabSetup;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input_data.txt").expect("Need input_data.txt to exist");
    let mut input_str: String = String::new();
    f.read_to_string(&mut input_str).unwrap();
    let crab_setup = CrabSetup::new(include_str!("../input_data.txt"));
    let day_a = crab_setup.calculate_day_a();
    println!("Day a result: {}", day_a);
    let crab_setup = CrabSetup::new(include_str!("../input_data.txt"));
    let day_b = crab_setup.calculate_day_b();
    println!("Day b result: {}", day_b);
}
