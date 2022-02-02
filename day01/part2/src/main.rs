use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let data = fs::read_to_string(Path::new("../input.txt")).unwrap();
    let now = Instant::now();

    let result = data
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .map(|n| get_fuel_costs(n))
        .sum::<i32>();

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}

fn get_fuel_costs(mass: i32) -> i32 {
    let res = (mass / 3i32) - 2;
    if res > 0 {
        res + get_fuel_costs(res)
    } else {
        0
    }
}