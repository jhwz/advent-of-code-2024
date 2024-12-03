use std::{
    fs::{self},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day3");
    let str = fs::read_to_string(base.join("input.txt"))?;

    let part1_re = regex::Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut part1_total = 0;
    for capture in part1_re.captures_iter(str.as_str()) {
        let x: i32 = capture.get(1).unwrap().as_str().parse()?;
        let y: i32 = capture.get(2).unwrap().as_str().parse()?;
        part1_total += x * y;
    }

    println!("Part 1: {}", part1_total);

    let part2_re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)")?;
    let mut part2_total = 0;
    let mut active = true;
    for capture in part2_re.captures_iter(str.as_str()) {
        match capture.get(0).unwrap().as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ => {
                if active {
                    let x: i32 = capture.get(1).unwrap().as_str().parse()?;
                    let y: i32 = capture.get(2).unwrap().as_str().parse()?;
                    part2_total += x * y;
                }
            }
        }
    }

    println!("Part 2: {}", part2_total);

    Ok(())
}
