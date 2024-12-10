use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day10");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let nrows = lines.len() as i32;
    let ncols = lines[0].len() as i32;
    let mut grid = vec![0 as usize; (nrows * ncols) as usize];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i * ncols as usize + j] = c.to_string().parse::<usize>()?;
        }
    }

    const OFFSETS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn recurse_part1(
        grid: &[usize],
        seen: &mut HashSet<(i32, i32)>,
        nrows: i32,
        ncols: i32,
        expect: usize,
        pos: (i32, i32),
    ) {
        if expect == 9 {
            seen.insert(pos);
            return;
        }
        for offset in OFFSETS.iter() {
            let next = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);
            if next.0 < 0 || next.0 >= nrows || next.1 < 0 || next.1 >= ncols {
                continue;
            }
            if grid[(next.0 * ncols + next.1) as usize] == expect + 1 {
                recurse_part1(grid, seen, nrows, ncols, expect + 1, (next.0, next.1));
            }
        }
    }

    fn recurse_part2(
        grid: &[usize],
        nrows: i32,
        ncols: i32,
        expect: usize,
        pos: (i32, i32),
    ) -> i64 {
        if expect == 9 {
            return 1;
        }
        let mut total = 0;
        for offset in OFFSETS.iter() {
            let next = (pos.0 as i32 + offset.0, pos.1 as i32 + offset.1);
            if next.0 < 0 || next.0 >= nrows || next.1 < 0 || next.1 >= ncols {
                continue;
            }
            if grid[(next.0 * ncols + next.1) as usize] == expect + 1 {
                total += recurse_part2(grid, nrows, ncols, expect + 1, (next.0, next.1));
            }
        }
        total
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            if grid[(i * ncols + j) as usize] == 0 {
                seen.clear();
                recurse_part1(grid.as_slice(), &mut seen, nrows, ncols, 0, (i, j));
                part1 += seen.len();

                part2 += recurse_part2(grid.as_slice(), nrows, ncols, 0, (i, j));
            }
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
