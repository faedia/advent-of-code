use clap::Parser;
use std::{fs::read_to_string, mem::swap};

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(Clone)]
struct State {
    buffer: Vec<bool>,
    x: usize,
    y: usize,
}

impl State {
    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.buffer[(self.y * y) + x] = value;
    }

    fn get(&self, x: isize, y: isize) -> Option<&bool> {
        if x >= 0 && y >= 0 && (x as usize) < self.x && (y as usize) < self.y {
            self.buffer.get((self.y * (y as usize)) + (x as usize))
        } else {
            None
        }
    }

    fn get_x_y(&self, idx: usize) -> (usize, usize) {
        let x = idx % self.y;
        let y = idx / self.x;
        (x, y)
    }

    fn neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for x_off in -1..=1 {
            for y_off in -1..=1 {
                if x_off != 0 || y_off != 0 {
                    if let Some(true) = self.get((x as isize) + x_off, (y as isize) + y_off) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn parse(s: &str) -> State {
    let mut state = State {
        buffer: vec![false; s.lines().count() * s.lines().nth(0).unwrap().len()],
        x: s.lines().nth(0).unwrap().len(),
        y: s.lines().count(),
    };
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            state.set(x, y, c == '#');
        }
    }

    state
}

fn part1(init: &State, steps: u32) -> usize {
    let mut front = init.clone();
    let mut back = init.clone();

    for _ in 0..steps {
        for (idx, value) in front.buffer.iter().enumerate() {
            let (x, y) = front.get_x_y(idx);
            let neighbors = front.neighbor_count(x, y);
            back.buffer[idx] =
                (*value && (neighbors == 2 || neighbors == 3)) || (!value && neighbors == 3);
        }

        swap(&mut front, &mut back);
    }

    front.buffer.iter().filter(|p| **p).count()
}

fn part2(init: &State, steps: u32) -> usize {
    let mut front = init.clone();
    let mut back = init.clone();

    front.set(0, 0, true);
    front.set(front.x - 1, 0, true);
    front.set(0, front.y - 1, true);
    front.set(front.x - 1, front.y - 1, true);

    for _ in 0..steps {
        for (idx, value) in front.buffer.iter().enumerate() {
            let (x, y) = front.get_x_y(idx);
            let neighbors = front.neighbor_count(x, y);
            back.buffer[idx] = (*value && (neighbors == 2 || neighbors == 3))
                || (!value && neighbors == 3)
                || ((x == 0 || x == front.x - 1) && (y == 0 || y == front.y - 1));
        }

        swap(&mut front, &mut back);
    }

    front.buffer.iter().filter(|p| **p).count()
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let parsed = parse(&input_str);
    let part1_result = part1(&parsed, 100);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parsed, 100);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse1_test() {
        assert_eq!(
            super::part1(
                &super::parse(
                    ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
                ),
                4,
            ),
            4
        )
    }

    #[test]
    fn parse2_test() {
        assert_eq!(
            super::part2(
                &super::parse(
                    ".#.#.#
...##.
#....#
..#...
#.#..#
####..",
                ),
                5,
            ),
            17
        )
    }
}
