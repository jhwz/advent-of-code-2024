use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day8");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    let mut coords: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let mut nrows: isize = 0;
    let mut ncols: isize = 0;
    for line in reader.lines() {
        let line = line?;
        ncols = line.len() as isize;
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => continue,
                _ => {
                    let v = coords.entry(c).or_insert(vec![]);
                    v.push((nrows, col as isize));
                }
            }
        }
        nrows += 1;
    }

    let mut antinodes: Vec<bool> = vec![false; (nrows * ncols) as usize];
    for v in coords.values() {
        for v1 in v.as_slice() {
            for v2 in v.as_slice() {
                if v1 == v2 {
                    continue;
                }
                // calculate the antinode
                let antinode = (v2.0 + (v2.0 - v1.0), v2.1 + (v2.1 - v1.1));
                if antinode.0 < 0 || antinode.0 >= nrows || antinode.1 < 0 || antinode.1 >= ncols {
                    continue;
                }

                let idx = antinode.0 * ncols + antinode.1;
                antinodes[idx as usize] = true;
            }
        }
    }

    println!("Part 1: {}", antinodes.iter().filter(|&x| *x).count());

    let mut antinodes_part2: Vec<bool> = vec![false; (nrows * ncols) as usize];
    for v in coords.values() {
        for v1 in v.as_slice() {
            for v2 in v.as_slice() {
                if v1 == v2 {
                    continue;
                }
                let rowdiff = v2.0 - v1.0;
                let coldiff = v2.1 - v1.1;
                let mut antinode = (v2.0, v2.1);
                loop {
                    let idx = antinode.0 * ncols + antinode.1;
                    antinodes_part2[idx as usize] = true;

                    antinode.0 += rowdiff;
                    antinode.1 += coldiff;
                    if antinode.0 < 0
                        || antinode.0 >= nrows
                        || antinode.1 < 0
                        || antinode.1 >= ncols
                    {
                        break;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", antinodes_part2.iter().filter(|&x| *x).count());

    Ok(())
}
