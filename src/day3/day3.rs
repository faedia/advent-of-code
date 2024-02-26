use std::{collections::HashSet, fs::read_to_string};

use clap::Parser;

fn next_pos(pos: (i32, i32), c: char) -> (i32, i32) {
    match c {
        '^' => (pos.0, pos.1 + 1),
        'v' => (pos.0, pos.1 - 1),
        '<' => (pos.0 - 1, pos.1),
        '>' => (pos.0 + 1, pos.1),
        _ => pos,
    }
}

fn part1(s: &str) -> usize {
    s.chars()
        .fold((HashSet::from([(0, 0)]), (0, 0)), |mut acc, c| {
            let pos = next_pos(acc.1, c);
            acc.0.insert(pos);
            (acc.0, pos)
        })
        .0
        .into_iter()
        .count()
}

fn part2(s: &str) -> usize {
    s.chars()
        .fold(
            (HashSet::from([(0, 0)]), (0, 0), (0, 0), false),
            |mut acc, c| {
                let pos = next_pos(if acc.3 { acc.2 } else { acc.1 }, c);
                acc.0.insert(pos);
                if acc.3 {
                    (acc.0, acc.1, pos, false)
                } else {
                    (acc.0, pos, acc.2, true)
                }
            },
        )
        .0
        .into_iter()
        .count()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(&input_str);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&input_str);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test() {
        assert_eq!(super::part1(">"), 2);
        assert_eq!(super::part1("^>v<"), 4);
        assert_eq!(super::part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2("^v"), 3);
        assert_eq!(super::part2("^>v<"), 3);
        assert_eq!(super::part2("^v^v^v^v^v"), 11);
    }
}
