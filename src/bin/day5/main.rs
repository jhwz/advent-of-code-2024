use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day5");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    let mut orders: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        if line.contains("|") {
            let values = line
                .split("|")
                .map(|v| v.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let min = std::cmp::min(values[0], values[1]);
            let curr = orders.entry(min).or_insert(Vec::new());
            curr.push((values[0], values[1]));
        } else {
            updates.push(
                line.split(",")
                    .map(|v| v.trim().parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }

    let sorter = |a: &usize, b: &usize| -> Ordering {
        let order = orders.get(std::cmp::min(a, b)).and_then(|vec| {
            vec.iter()
                .find(|(v1, v2)| (v1 == a && v2 == b) || (v1 == b && v2 == a))
        });

        match order {
            Some((v1, _)) if v1 == a => Ordering::Less,
            Some(_) => Ordering::Greater,
            None => Ordering::Equal,
        }
    };

    let mut part1 = 0;
    let mut part2 = 0;
    for mut update in updates {
        let issorted = update.is_sorted_by(|a, b| sorter(a, b).is_le());
        if issorted {
            part1 += update[update.len() / 2];
            continue;
        }

        update.sort_by(sorter);
        part2 += update[update.len() / 2];
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
