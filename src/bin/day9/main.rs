use std::{
    fs::{self},
    isize,
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day9");

    let str = fs::read_to_string(base.join("input.txt"))?;
    let line = str.trim_ascii();
    // build the file list
    let mut values: Vec<i64> = Vec::new();
    let mut file = true;
    let mut id = 0;
    for c in line.chars() {
        let value = c.to_string().parse::<i64>()?;
        if file {
            for _ in 0..value {
                values.push(id as i64);
            }
            id += 1;
        } else {
            for _ in 0..value {
                values.push(-1);
            }
        }
        file = !file;
    }

    part1(values.clone());
    part2(values);

    Ok(())
}

fn checksum(values: &[i64]) -> usize {
    let mut sum = 0;
    for i in 0..values.len() {
        if values[i] == -1 {
            continue;
        }
        sum += i * values[i] as usize;
    }
    return sum;
}

fn part1(mut values: Vec<i64>) {
    let mut search = 0;
    let mut curr = values.len();
    while curr > 0 {
        curr -= 1;
        let value = values[curr];
        if value == -1 {
            continue;
        } else if curr <= search {
            break;
        }

        values[curr] = -1;

        while values[search] != -1 {
            search += 1;
        }
        values[search] = value;
    }

    println!("Part 1: {}", checksum(values.as_slice()));
}

fn part2(mut values: Vec<i64>) {
    // let mut values: Vec<i64> = vec![
    //     0, 0, -1, -1, -1, 1, 1, 1, -1, -1, -1, 2, -1, -1, -1, 3, 3, 3, -1, 4, 4, -1, 5, 5, 5, 5,
    //     -1, 6, 6, 6, 6, -1, 7, 7, 7, -1, 8, 8, 8, 8, 9, 9,
    // ];

    let mut free: Vec<(i64, i64)> = Vec::new();
    let mut cursor = 0;
    while cursor < values.len() {
        while cursor < values.len() && values[cursor] != -1 {
            cursor += 1;
        }
        let start = cursor;
        while cursor < values.len() && values[cursor] == -1 {
            cursor += 1;
        }
        free.push((start as i64, cursor as i64));
    }

    let mut cursor = values.len() as isize;
    let mut last = i64::MAX;
    loop {
        cursor -= 1;
        if cursor < 0 {
            break;
        }
        let value = values[cursor as usize];
        if value == -1 || value >= last {
            continue;
        }
        let cursor_end = cursor + 1;
        while cursor >= 0 && values[cursor as usize] == value {
            cursor -= 1;
        }
        cursor += 1;
        let size = (cursor_end - (cursor)) as i64;

        for i in 0..free.len() {
            let (start, end) = free[i];
            if start >= cursor as i64 {
                break;
            }
            if end - start < size {
                continue;
            }
            for j in start..start + size {
                values[j as usize] = value;
            }
            free[i] = (start + size, end);
            for j in cursor..cursor_end {
                values[j as usize] = -1;
            }
            break;
        }
        last = value;
    }

    println!("Part 2: {}", checksum(values.as_slice()));
}
