use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let data = fs::read_to_string(Path::new("day04/input.txt")).unwrap();
    let now = Instant::now();

    let mut result = 0;
    let range: Vec<i32> = data.split("-").map(|n| n.parse::<i32>().unwrap()).collect();
    if let [from, to] = range[..] {
        for pw in from..to {
            if check_criteria(&pw) {
                result += 1;
            }
        }
    }

    let elapsed = now.elapsed();
    println!("{result} {elapsed:.2?}");
}

fn check_criteria(pw: &i32) -> bool {
    let digits: Vec<i32> = pw
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect();

    let mut is_increasing = true;
    let mut has_adjacent = false;

    let mut last: i32 = -1;
    for digit in digits {
        is_increasing &= digit >= last;
        if digit == last {
            has_adjacent = true;
        }

        last = digit;
    }

    return is_increasing && has_adjacent;
}
