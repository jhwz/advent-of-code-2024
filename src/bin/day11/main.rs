use std::{
    collections::HashMap,
    fs::{self},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day11");

    let text = fs::read_to_string(base.join("input.txt"))?;
    let values = text
        .split_ascii_whitespace()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    part1(values.clone());

    part2(values);

    Ok(())
}

// original solution, obviously not sufficient for larger case so part 2 optimises that
fn part1(mut values: Vec<String>) {
    for _blink in 0..25 {
        let mut next: Vec<String> = Vec::with_capacity(values.len());
        for el in values {
            match el.as_str() {
                "0" => next.push("1".to_owned()),
                _ if el.len() % 2 == 0 => {
                    let first = el[0..el.len() / 2].to_owned();
                    let mut second = el[el.len() / 2..].trim_start_matches("0");
                    if second.is_empty() {
                        second = "0";
                    }
                    next.push(first);
                    next.push(second.to_owned())
                }
                _ => {
                    let value = (el.parse::<i64>().unwrap() * 2024).to_string();
                    next.push(value)
                }
            }
        }
        values = next;
    }
    println!("Part 1: {}", values.len());
}

fn part2(values: Vec<String>) {
    fn check_level(value: &str, level: i64, cache: &mut HashMap<(String, i64), i64>) -> i64 {
        if level == 75 {
            return 1;
        }
        let key = (value.to_string(), level);
        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }
        let next = match value {
            "0" => check_level("1", level + 1, cache),
            _ if value.len() % 2 == 0 => {
                let first = &value[0..value.len() / 2];
                let mut second = value[value.len() / 2..].trim_start_matches("0");
                if second.is_empty() {
                    second = "0";
                }
                check_level(first, level + 1, cache) + check_level(second, level + 1, cache)
            }
            _ => {
                let next = value.parse::<i64>().unwrap() * 2024;
                check_level(&next.to_string(), level + 1, cache)
            }
        };
        cache.insert(key, next);
        next
    }

    let mut part2 = 0;
    let mut cache: HashMap<(String, i64), i64> = HashMap::new();
    for el in values {
        part2 += check_level(&el, 0, &mut cache);
    }

    println!("Part 2: {}", part2);
}
