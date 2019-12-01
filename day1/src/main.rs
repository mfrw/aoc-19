use std::fs;
use std::str::FromStr;

pub fn calculate_fule_req(module_mass: u128) -> u128 {
    (module_mass / 3) - 2
}

pub fn solve(input: String) -> u128 {
    let mut total_fuel_req: u128 = 0;

    for line in input.lines() {
        let module_mass: u128 = u128::from_str(line).unwrap();
        total_fuel_req += calculate_fule_req(module_mass);
    }
    total_fuel_req
}

fn main() {
    println!(
        "{:?}",
        solve(fs::read_to_string("input.txt").expect("Failed to read input file."))
    );
}
