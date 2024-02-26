use std::fs::read_to_string;

use clap::Parser;

fn part1(v: &Vec<(u32, u32, u32)>) -> u32 {
    v.iter()
        .map(|b| {
            let side1 = b.0 * b.1;
            let side2 = b.1 * b.2;
            let side3 = b.0 * b.2;
            let m = side1.min(side2).min(side3);
            (2 * side1) + (2 * side2) + (2 * side3) + m
        })
        .sum()
}

fn part2(v: &Vec<(u32, u32, u32)>) -> u32 {
    v.iter()
        .map(|b| {
            let mut values = [b.0, b.1, b.2];
            values.sort_unstable();
            let ribbon = (2 * values[0]) + (2 * values[1]);
            let bow = b.0 * b.1 * b.2;
            bow + ribbon
        })
        .sum()
}

fn parse(s: &str) -> Vec<(u32, u32, u32)> {
    s.lines()
        .map(|l| {
            let mut iter = l.splitn(3, "x");
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_vec = parse(&read_to_string(args.input).unwrap());
    let part1_result = part1(&input_vec);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&input_vec);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_test() {
        assert_eq!(super::parse("2x3x4\n1x1x10"), vec![(2, 3, 4), (1, 1, 10)]);
    }

    #[test]
    fn part1_test() {
        assert_eq!(super::part1(&vec![(2, 3, 4)]), 58);
        assert_eq!(super::part1(&vec![(1, 1, 10)]), 43);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2(&vec![(2, 3, 4)]), 34);
        assert_eq!(super::part2(&vec![(1, 1, 10)]), 14);
    }
}
