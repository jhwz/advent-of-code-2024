use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day4");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    // read the file into an array, with an element for each character
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let nrows = lines.len() as i32;
    let ncols = lines[0].len() as i32;
    let mut grid = vec![' '; (nrows * ncols) as usize];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i * ncols as usize + j] = c;
        }
    }

    // now find all the instances of XMAS
    let xmas = ['X', 'M', 'A', 'S'];
    let dirs: &[i32] = &[-1, 0, 1];
    let mut part1_matches = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            for x in dirs {
                for y in dirs {
                    for (idx, v) in xmas.iter().enumerate() {
                        let r = i + (idx as i32 * x);
                        let c = j + (idx as i32 * y);
                        if r < 0 || r >= nrows || c < 0 || c >= ncols {
                            break;
                        }
                        if grid[(r * ncols + c) as usize] != *v {
                            break;
                        }
                        if idx == xmas.len() - 1 {
                            part1_matches += 1;
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", part1_matches);

    let mut part2_matches = 0;
    fn check_pair(grid: &[char], ncols: i32, r1: i32, c1: i32, r2: i32, c2: i32) -> bool {
        let v1 = grid[(r1 * ncols + c1) as usize];
        let v2 = grid[(r2 * ncols + c2) as usize];
        return v1 == 'M' && v2 == 'S' || v1 == 'S' && v2 == 'M';
    }
    for i in 1..(nrows - 1) {
        for j in 1..(ncols - 1) {
            let idx = i * ncols + j;
            if grid[idx as usize] != 'A' {
                continue;
            }
            if check_pair(grid.as_slice(), ncols, i - 1, j - 1, i + 1, j + 1)
                && check_pair(grid.as_slice(), ncols, i - 1, j + 1, i + 1, j - 1)
            {
                part2_matches += 1;
            }
        }
    }
    println!("Part 2: {}", part2_matches);

    Ok(())
}
