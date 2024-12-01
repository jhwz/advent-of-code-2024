use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day1");

    // read the input to two vectors
    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if numbers.len() != 2 {
            panic!("Expected two numbers per line");
        }
        vec1.push(numbers[0]);
        vec2.push(numbers[1]);
    }

    vec1.sort();
    vec2.sort();

    let mut disance = 0;
    for i in 0..vec1.len() {
        disance += (vec1[i] - vec2[i]).abs();
    }
    println!("Part 1 disance: {}", disance);

    let mut similarity = 0;
    for val1 in vec1 {
        let count = vec2.iter().filter(|&x| *x == val1).count();
        similarity += val1 * count as i32;
    }
    println!("Part 2 similarity: {}", similarity);

    Ok(())
}
