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
    let mut has_pair = false;

    for i in 1..digits.len() {
        let lefthand = digits[i - 1];
        let righthand = digits[i];
        is_increasing &= righthand >= lefthand;
        let l_neighbour = i
            .checked_sub(2)
            .map_or(None, |j| digits.get(j))
            .map_or(-1, |x| *x);
        let r_neighbour = i
            .checked_add(1)
            .map_or(None, |j| digits.get(j))
            .map_or(-1, |x| *x);

        if lefthand == righthand && lefthand != l_neighbour && righthand != r_neighbour {
            has_pair = true;
        }
    }

    return is_increasing && has_pair;
}
