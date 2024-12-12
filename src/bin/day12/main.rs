use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day12");
    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    // read the file into an array, with an element for each character
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let bounds = (lines.len() as i32, lines[0].len() as i32);
    let mut grid = vec![' '; (bounds.0 * bounds.1) as usize];
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            grid[i * bounds.1 as usize + j] = c;
        }
    }
    part1(&grid, bounds);
    part2(&grid, bounds);

    Ok(())
}

fn out_of_bounds(pos: Pos, bounds: (i32, i32)) -> bool {
    pos.0 < 0 || pos.0 >= bounds.0 || pos.1 < 0 || pos.1 >= bounds.1
}

struct Plot {
    area: i32,
    perimeter: i32,
}
impl Plot {
    fn new() -> Self {
        Plot {
            area: 0,
            perimeter: 0,
        }
    }
    fn add(&mut self, other: Plot) {
        self.area += other.area;
        self.perimeter += other.perimeter;
    }
    fn price(&self) -> i32 {
        self.area * self.perimeter
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Pos(i32, i32);

impl Pos {
    fn index(&self, bounds: (i32, i32)) -> usize {
        (self.0 * bounds.1 + self.1) as usize
    }

    fn left(&self, dir: &(i32, i32)) -> Pos {
        match dir {
            (0, 1) => Pos(self.0 + 1, self.1),
            (0, -1) => Pos(self.0 - 1, self.1),
            (1, 0) => Pos(self.0, self.1 - 1),
            (-1, 0) => Pos(self.0, self.1 + 1),
            _ => panic!("Invalid direction"),
        }
    }

    fn move_dir(&self, dir: &(i32, i32)) -> Pos {
        Pos(self.0 + dir.0, self.1 + dir.1)
    }
    fn move_back(&self, dir: &(i32, i32)) -> Pos {
        Pos(self.0 - dir.0, self.1 - dir.1)
    }
}

fn part2(grid: &Vec<char>, bounds: (i32, i32)) {
    fn out_of_region(pos: Pos, region: char, grid: &Vec<char>, bounds: (i32, i32)) -> bool {
        out_of_bounds(pos, bounds) || grid[pos.index(bounds)] != region
    }

    fn walk_region(
        pos: Pos,
        dir: &(i32, i32),
        region: char,
        grid: &Vec<char>,
        bounds: (i32, i32),
        seen: &mut HashSet<Pos>,
    ) -> Plot {
        let mut plot = Plot::new();
        if out_of_region(pos, region, grid, bounds) {
            // if the point to the left is also a bound, don't increment the perimeter
            if !(out_of_region(pos.left(dir), region, grid, bounds)
                && !out_of_region(pos.move_back(dir).left(dir), region, grid, bounds))
            {
                plot.perimeter = 1;
            }
            return plot;
        }
        if seen.contains(&pos) {
            return plot;
        }
        plot.area += 1;
        seen.insert(pos);
        for dir in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = pos.move_dir(dir);
            plot.add(walk_region(next, dir, region, grid, bounds, seen));
        }
        plot
    }

    // find all the regions
    let mut seen: HashSet<Pos> = HashSet::new();
    let mut part2 = 0;
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            let pos = Pos(i, j);
            if seen.contains(&pos) {
                continue;
            }
            let plot = walk_region(
                pos,
                &(0, 0),
                grid[(i * bounds.1 + j) as usize],
                &grid,
                bounds,
                &mut seen,
            );
            part2 += plot.price();
        }
    }
    println!("Part 2: {part2}");
}

fn part1(grid: &Vec<char>, bounds: (i32, i32)) {
    fn walk_region(
        i: i32,
        j: i32,
        curr: char,
        grid: &Vec<char>,
        bounds: (i32, i32),
        seen: &mut HashSet<(usize, usize)>,
    ) -> Plot {
        let mut plot = Plot::new();
        if out_of_bounds(Pos(i, j), bounds) || grid[(i * bounds.1 + j) as usize] != curr {
            plot.perimeter = 1;
            return plot;
        }
        if seen.contains(&(i as usize, j as usize)) {
            return plot;
        }
        plot.area += 1;
        seen.insert((i as usize, j as usize));
        for offset in &[-1, 1] {
            plot.add(walk_region(i + offset, j, curr, grid, bounds, seen));
            plot.add(walk_region(i, j + offset, curr, grid, bounds, seen));
        }
        plot
    }

    // find all the regions
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut part1 = 0;
    for i in 0..bounds.0 {
        for j in 0..bounds.1 {
            if seen.contains(&(i as usize, j as usize)) {
                continue;
            }
            let plot = walk_region(
                i,
                j,
                grid[(i * bounds.1 + j) as usize],
                &grid,
                bounds,
                &mut seen,
            );
            part1 += plot.price();
        }
    }
    println!("Part 1: {}", part1);
}
