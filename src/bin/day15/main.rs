use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            '^' => Some(Move::Up),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            '>' => Some(Move::Right),
            _ => None,
        }
    }

    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Move::Up => (pos.0 - 1, pos.1),
            Move::Down => (pos.0 + 1, pos.1),
            Move::Left => (pos.0, pos.1 - 1),
            Move::Right => (pos.0, pos.1 + 1),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day15");

    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let idx = lines.iter().position(|l| l.is_empty()).unwrap();

    part1(lines.as_slice(), idx);
    part2(lines.as_slice(), idx);

    Ok(())
}

fn part2(lines: &[String], idx: usize) {
    let mut grid: Vec<char> = Vec::new();
    for line in lines[..idx].iter() {
        grid.append(
            &mut line
                .chars()
                .flat_map(|c| match c {
                    '@' => vec!['@', '.'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '#' => vec!['#', '#'],
                    _ => panic!("Invalid character {c}"),
                })
                .collect::<Vec<char>>(),
        );
    }

    let mut moves: Vec<Move> = Vec::new();
    for line in lines[idx + 1..].iter() {
        for c in line.chars() {
            moves.push(Move::from_char(c).unwrap());
        }
    }

    let bounds = (idx, lines[0].len() * 2);
    let at_idx = grid.iter().position(|&c| c == '@').unwrap();

    let mut curr = (at_idx / bounds.1, at_idx % bounds.1);

    for m in moves.iter() {
        let next_pos = m.step(curr);
        let next_idx = next_pos.0 * bounds.1 + next_pos.1;
        let curr_idx = curr.0 * bounds.1 + curr.1;

        match grid[next_idx] {
            '.' => {
                grid[curr_idx] = '.';
                grid[next_idx] = '@';
                curr = next_pos;
            }
            '[' | ']' => match m {
                Move::Up | Move::Down => {
                    fn can_move(
                        grid: &mut Vec<char>,
                        pos: (usize, usize),
                        bounds: (usize, usize),
                        m: &Move,
                    ) -> bool {
                        let idx = pos.0 * bounds.1 + pos.1;
                        match grid[idx] {
                            '[' | ']' => {
                                let next1 = m.step(pos);
                                let offset = if grid[idx] == '[' { 1 } else { -1 } as i64;
                                let next2 = (next1.0, ((next1.1 as i64) + offset) as usize);

                                return can_move(grid, next1, bounds, m)
                                    && can_move(grid, next2, bounds, m);
                            }
                            '.' => return true,
                            _ => {}
                        }
                        false
                    }

                    fn do_move(
                        grid: &mut Vec<char>,
                        pos: (usize, usize),
                        bounds: (usize, usize),
                        m: &Move,
                    ) {
                        let idx = pos.0 * bounds.1 + pos.1;
                        match grid[idx] {
                            '[' | ']' => {
                                let next1 = m.step(pos);
                                let offset = if grid[idx] == '[' { 1 } else { -1 } as i64;
                                let next2 = (next1.0, ((next1.1 as i64) + offset) as usize);
                                do_move(grid, next1, bounds, m);
                                do_move(grid, next2, bounds, m);
                                let idx2 = (idx as i64 + offset) as usize;
                                grid[next1.0 * bounds.1 + next1.1] = grid[idx];
                                grid[next2.0 * bounds.1 + next2.1] = grid[idx2];
                                grid[idx] = '.';
                                grid[idx2] = '.';
                            }
                            _ => {}
                        }
                    }

                    if can_move(&mut grid, next_pos, bounds, m) {
                        do_move(&mut grid, next_pos, bounds, m);
                        grid[curr_idx] = '.';
                        grid[next_idx] = '@';
                        curr = next_pos;
                    }
                }
                Move::Left | Move::Right => {
                    fn can_move_to(
                        grid: &mut Vec<char>,
                        pos: (usize, usize),
                        bounds: (usize, usize),
                        m: &Move,
                    ) -> bool {
                        let idx = pos.0 * bounds.1 + pos.1;
                        match grid[idx] {
                            '[' | ']' => {
                                let next = m.step(pos);
                                if can_move_to(grid, next, bounds, m) {
                                    let next_value = grid[next.0 * bounds.1 + next.1];
                                    grid[next.0 * bounds.1 + next.1] = grid[idx];
                                    grid[idx] = next_value;
                                    return true;
                                }
                            }
                            '.' => return true,
                            _ => {}
                        }
                        false
                    }
                    if can_move_to(&mut grid, next_pos, bounds, m) {
                        grid[curr_idx] = '.';
                        grid[next_idx] = '@';
                        curr = next_pos;
                    }
                }
            },
            '#' => {
                // move fails, do nothing
            }
            _ => panic!("Invalid position {}", grid[next_idx]),
        }
    }

    let mut part2 = 0;
    for r in 0..bounds.0 {
        for c in 0..bounds.1 {
            if grid[r * bounds.1 + c] == '[' {
                part2 += 100 * r + c;
            }
        }
    }
    println!("Part 2: {}", part2);
}

fn part1(lines: &[String], idx: usize) {
    let mut grid: Vec<char> = Vec::new();
    for line in lines[..idx].iter() {
        grid.append(&mut line.chars().collect::<Vec<char>>());
    }

    let mut moves: Vec<Move> = Vec::new();
    for line in lines[idx + 1..].iter() {
        for c in line.chars() {
            moves.push(Move::from_char(c).unwrap());
        }
    }

    let bounds = (idx, lines[0].len());
    let at_idx = grid.iter().position(|&c| c == '@').unwrap();

    let mut curr = (at_idx / bounds.1, at_idx % bounds.1);

    'outer: for m in moves.iter() {
        let next_pos = m.step(curr);
        let next_idx = next_pos.0 * bounds.1 + next_pos.1;
        let curr_idx = curr.0 * bounds.1 + curr.1;

        match grid[next_idx] {
            '.' => {
                grid[curr_idx] = '.';
                grid[next_idx] = '@';
                curr = next_pos;
            }
            'O' => {
                // move the box
                let mut next_box_pos = m.step(next_pos);
                loop {
                    let next_box_idx = next_box_pos.0 * bounds.1 + next_box_pos.1;
                    match grid[next_box_idx] {
                        'O' => {
                            next_box_pos = m.step(next_box_pos);
                            continue;
                        }
                        '#' => continue 'outer, // box is blocked, can't move
                        '.' => break,
                        _ => panic!("Invalid box position"),
                    }
                }
                grid[next_box_pos.0 * bounds.1 + next_box_pos.1] = 'O';
                grid[next_idx] = '@';
                grid[curr_idx] = '.';
                curr = next_pos;
            }
            '#' => {
                // move fails, do nothing
            }
            _ => panic!("Invalid position {}", grid[next_idx]),
        }
    }

    let mut part1 = 0;
    for r in 0..bounds.0 {
        for c in 0..bounds.1 {
            if grid[r * bounds.1 + c] == 'O' {
                part1 += 100 * r + c;
            }
        }
    }
    println!("Part 1: {}", part1);
}
