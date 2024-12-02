use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day2");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    fn is_safe(numbers: &[i32]) -> bool {
        let sign = (numbers[0] - numbers[1]).signum();
        for i in 0..numbers.len() - 1 {
            let curr = numbers[i];
            let next = numbers[i + 1];
            let mut diff = curr - next;
            if diff.signum() != sign {
                return false;
            }
            diff = diff.abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        return true;
    }

    let mut safe = 0;
    let mut safe_with_one_bad_level = 0;
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if is_safe(numbers.as_slice()) {
            safe += 1;
            safe_with_one_bad_level += 1;
            continue;
        }
        for i in 0..numbers.len() {
            let mut numbers = numbers.clone();
            numbers.remove(i);
            if is_safe(numbers.as_slice()) {
                safe_with_one_bad_level += 1;
                break;
            }
        }
    }

    println!("Part 1 safe: {}", safe);

    println!("Part 2 safe: {}", safe_with_one_bad_level);

    Ok(())
}
