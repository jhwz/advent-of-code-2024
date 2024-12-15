use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
    ops::Index,
    path::Path,
};

#[derive(Debug, Clone)]
struct Vec2(i32, i32);

#[derive(Debug, Clone)]
struct Robot {
    pos: Vec2,
    dir: Vec2,
}

impl Robot {
    fn step_n(&mut self, n: i32, bounds: &Vec2) {
        self.pos.0 = (self.pos.0 + self.dir.0 * n).rem_euclid(bounds.0);
        self.pos.1 = (self.pos.1 + self.dir.1 * n).rem_euclid(bounds.1);
    }
}

fn in_bounds(v: i32, bounds: (i32, i32)) -> bool {
    return v >= bounds.0 && v < bounds.1;
}

fn read_input() -> Result<Vec<Robot>, Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day14");
    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);
    let mut robots: Vec<Robot> = Vec::new();
    // p=45,89 v=-10,-91
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)")?;
    for line in reader.lines() {
        let line = line?;
        let caps = re.captures(&line).unwrap();
        let pos = Vec2(caps[1].parse()?, caps[2].parse()?);
        let dir = Vec2(caps[3].parse()?, caps[4].parse()?);
        robots.push(Robot { pos, dir });
    }
    Ok(robots)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let robots = read_input()?;
    let bounds = Vec2(101, 103);

    let part1robots = robots.clone();
    let quadrants = vec![
        ((0, 50), (0, 51)),
        ((0, 50), (52, 103)),
        ((51, 101), (0, 51)),
        ((51, 101), (52, 103)),
    ];
    let mut counts = vec![0; 4];
    for mut robot in part1robots {
        robot.step_n(100, &bounds);
        for (i, quadrant) in quadrants.iter().enumerate() {
            if in_bounds(robot.pos.0, quadrant.0) && in_bounds(robot.pos.1, quadrant.1) {
                counts[i] += 1;
            }
        }
    }
    println!("Part 1: {}", counts.iter().product::<i32>());

    let mut robots = robots.clone();
    let mut seconds = 0;
    let mut _stdin_buf = String::new();
    loop {
        seconds += 1;
        if seconds % 1_000_000 == 0 {
            println!("Seconds: {}", seconds);
        }
        for robot in robots.iter_mut() {
            robot.step_n(1, &bounds);
        }
        if print_robots(robots.as_slice(), &bounds) {
            println!("Seconds: {}", seconds);
            stdin().read_line(&mut _stdin_buf)?;
        }
    }

    Ok(())
}

fn print_robots(robots: &[Robot], bounds: &Vec2) -> bool {
    let mut grid = vec![' '; (bounds.0 * bounds.1) as usize];
    for robot in robots {
        grid[(robot.pos.0 * bounds.1 + robot.pos.1) as usize] = '#'
    }
    check_grid(grid.as_slice(), bounds)
}

fn check_grid(grid: &[char], bounds: &Vec2) -> bool {
    for i in 0..bounds.0 {
        let row = &grid[(i * bounds.1) as usize..((i + 1) * bounds.1) as usize];
        if row.iter().collect::<String>().contains("#######") {
            for i in 0..bounds.0 {
                for j in 0..bounds.1 {
                    let v = grid[(i * bounds.1 + j) as usize];
                    print!("{}", v);
                }
                print!("\n")
            }
            return true;
        }
    }
    false
}
