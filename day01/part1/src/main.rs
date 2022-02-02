use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let data = fs::read_to_string(Path::new("../input.txt")).unwrap();
    let now = Instant::now();

    let result = data
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .map(|n| (n/3)-2)
        .sum::<i32>();

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}
