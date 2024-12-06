use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn index(v: (i32, i32), ncols: i32) -> usize {
    (v.0 * ncols + v.1) as usize
}
fn add(p: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (p.0 + dir.0, p.1 + dir.1)
}

fn walk_grid(
    grid: &[char],
    start: (i32, i32),
    startdir: (i32, i32),
    nrows: i32,
    ncols: i32,
) -> (HashSet<((i32, i32), (i32, i32))>, bool) {
    let mut pos = start;
    let mut dir = startdir;
    let mut positions: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    loop {
        if !positions.insert((pos, dir)) {
            return (positions, true); // In a loop
        }

        let mut nextpos = add(pos, dir);
        if nextpos.0 < 0 || nextpos.0 >= nrows || nextpos.1 < 0 || nextpos.1 >= ncols {
            return (positions, false);
        }
        while grid[index(nextpos, ncols)] == '#' {
            dir = (dir.1, dir.0 * -1);
            nextpos = add(pos, dir);
        }
        pos = nextpos;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day6");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    let mut start: (i32, i32) = (-1, -1);
    let mut dir: (i32, i32) = (0, 1);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let nrows = lines.len() as i32;
    let ncols = lines[0].len() as i32;
    let mut grid = vec![' '; (nrows * ncols) as usize];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i * ncols as usize + j] = c;

            match c {
                '>' => dir = (0, 1),
                'v' => dir = (1, 0),
                '<' => dir = (0, -1),
                '^' => dir = (-1, 0),
                _ => continue,
            }
            start = (i as i32, j as i32);
            grid[i * ncols as usize + j] = 'X';
        }
    }

    let (positions, inloop) = walk_grid(grid.as_slice(), start, dir, nrows, ncols);
    if inloop {
        panic!("In a loop");
    }
    let mut unique_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut loops = 0;
    for (pos, _) in positions {
        if unique_positions.insert(pos) {
            grid[index(pos, ncols)] = '#';
            let (_, inloop) = walk_grid(grid.as_slice(), start, dir, nrows, ncols);
            if inloop {
                loops += 1;
            }
            grid[index(pos, ncols)] = '.';
        }
    }
    println!("Part 1: {}", unique_positions.len());
    println!("Part 2: {}", loops);

    Ok(())
}
