use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    i64,
    io::{BufRead, BufReader},
    ops::Index,
    path::Path,
};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Reindeer {
    pos: (usize, usize),
    orientation: Orientation,
}

impl Reindeer {
    fn at_pos(&self, grid: &[char], bounds: (usize, usize)) -> char {
        let (r, c) = self.pos;
        grid[r * bounds.1 + c]
    }

    fn step_forward(&self) -> Reindeer {
        let (r, c) = self.pos;
        let (x, y) = match self.orientation {
            Orientation::North => (r - 1, c),
            Orientation::South => (r + 1, c),
            Orientation::East => (r, c + 1),
            Orientation::West => (r, c - 1),
        };
        Reindeer {
            pos: (x, y),
            orientation: self.orientation,
        }
    }

    fn turn_left(&self) -> Reindeer {
        let orientation = match self.orientation {
            Orientation::North => Orientation::West,
            Orientation::East => Orientation::North,
            Orientation::South => Orientation::East,
            Orientation::West => Orientation::South,
        };
        Reindeer {
            pos: self.pos,
            orientation,
        }
    }
    fn turn_right(&self) -> Reindeer {
        let orientation = match self.orientation {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        };
        Reindeer {
            pos: self.pos,
            orientation,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day16");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);
    let mut grid: Vec<char> = Vec::new();
    let mut nrows = 0;
    for line in reader.lines() {
        nrows += 1;
        grid.append(&mut line?.chars().collect::<Vec<char>>());
    }
    let bounds = (nrows, grid.len() / nrows);

    let start_idx = grid.iter().position(|&c| c == 'S').unwrap();
    let reindeer = Reindeer {
        pos: (start_idx / bounds.1, start_idx % bounds.1),
        orientation: Orientation::East,
    };

    let mut paths: HashSet<(usize, usize)> = HashSet::new();
    let mut seen: HashMap<Reindeer, i64> = HashMap::new();

    let mut queue: VecDeque<(Reindeer, i64, Vec<(usize, usize)>)> = VecDeque::new();
    queue.push_back((reindeer, 0, Vec::new()));
    let mut min = i64::MAX;
    loop {
        if queue.is_empty() {
            break;
        }
        let (r, score, path) = queue.pop_front().unwrap();
        if let Some(prev) = seen.get(&r) {
            if score > *prev {
                continue;
            }
        }
        seen.insert(r, score);

        match r.at_pos(grid.as_slice(), bounds) {
            'E' => {
                if score < min {
                    min = score;
                    paths.clear();
                }
                if score == min {
                    paths.extend(path);
                }
                continue;
            }
            '#' => continue,
            _ => (),
        }

        let mut new_path = path.clone();
        new_path.push(r.pos);

        queue.push_back((r.step_forward(), score + 1, new_path.clone()));
        queue.push_back((r.turn_left(), score + 1000, new_path.clone()));
        queue.push_back((r.turn_right(), score + 1000, new_path));
    }
    println!("Part 1: {}", min);
    println!("Part 2: {}", paths.len() + 1);

    Ok(())
}
