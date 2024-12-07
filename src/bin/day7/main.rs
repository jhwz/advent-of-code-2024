use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day7");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    fn matches_sum(total: i64, curr: i64, rest: &[i64]) -> bool {
        if rest.len() == 0 {
            return total == curr;
        }
        matches_sum(total, curr * rest[0], &rest[1..])
            || matches_sum(total, curr + rest[0], &rest[1..])
    }

    fn matches_sum2(total: i64, curr: i64, rest: &[i64]) -> bool {
        if rest.len() == 0 {
            return total == curr;
        }
        matches_sum2(total, curr * rest[0], &rest[1..])
            || matches_sum2(total, curr + rest[0], &rest[1..])
            || matches_sum2(
                total,
                (curr.to_string() + rest[0].to_string().as_str())
                    .parse::<i64>()
                    .unwrap(),
                &rest[1..],
            )
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let total = tokens[0].trim_end_matches(":").parse::<i64>()?;
        let values: Vec<i64> = tokens[1..]
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if matches_sum(total, 0, values.as_slice()) {
            part1 += total;
        }
        if matches_sum2(total, values[0], &values[1..]) {
            part2 += total;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
