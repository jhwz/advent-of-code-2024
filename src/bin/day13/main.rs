use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn solve(self) -> Option<i64> {
        let (x1, y1) = self.button_a;
        let (x2, y2) = self.button_b;
        let (x, y) = self.prize;

        let n2_num = x1 * y - y1 * x;
        let n2_den = x1 * y2 - x2 * y1;
        if n2_num % n2_den != 0 {
            return None;
        }
        let n2 = n2_num / n2_den;

        let n1_num = x - x2 * n2;
        let n1_den = x1;
        if n1_num % n1_den != 0 {
            return None;
        }
        let n1 = n1_num / n1_den;
        Some(n1 * 3 + n2)
    }
}

fn read_input(add_prize: i64) -> Result<Vec<Machine>, Box<dyn std::error::Error>> {
    let base: &Path = Path::new("src/bin/day13");
    let file = File::open(base.join("input.txt"))?;
    let reader = BufReader::new(file);

    fn parse_line(line: &str, idx: usize) -> (i64, i64) {
        let parts = line
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|s| s.trim()[idx..].parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        (parts[0], parts[1])
    }

    let mut machines = Vec::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for section in lines.chunks(4) {
        if section.len() < 3 {
            break;
        }
        let mut m = Machine {
            button_a: parse_line(section[0].as_str(), 1),
            button_b: parse_line(section[1].as_str(), 1),
            prize: parse_line(section[2].as_str(), 2),
        };
        m.prize.0 += add_prize;
        m.prize.1 += add_prize;
        machines.push(m);
    }
    Ok(machines)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let machines_part1 = read_input(0)?;
    let mut part1 = 0;
    for machine in machines_part1 {
        match machine.solve() {
            Some(presses) => part1 += presses,
            None => (),
        }
    }
    println!("Part 1: {}", part1);

    let machines_part2 = read_input(10000000000000)?;
    let mut part2 = 0;
    for machine in machines_part2 {
        match machine.solve() {
            Some(presses) => part2 += presses,
            None => (),
        }
    }
    println!("Part 2: {}", part2);

    Ok(())
}
