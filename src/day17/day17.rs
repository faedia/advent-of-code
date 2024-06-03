use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn parse(s: &str) -> Vec<u32> {
    let mut v = s
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    v.sort();
    v
}

fn part1(containers: &[u32], expected: u32) -> usize {
    let mut count = 0;
    for (idx, item) in containers.iter().enumerate() {
        if *item == expected {
            count += 1
        } else if *item < expected {
            count += part1(&containers[idx + 1..], expected - *item)
        }
    }
    count
}

fn inner_part2(containers: &[u32], expected: u32) -> (usize, usize) {
    let mut m = usize::MAX - 1;
    let mut count = 0;

    for (idx, item) in containers.iter().enumerate() {
        if *item == expected {
            if m == 1 {
                count += 1;
            } else {
                m = 1;
                count = 1;
            }
        } else if *item < expected {
            let (inner_m, inner_count) = inner_part2(&containers[idx + 1..], expected - *item);
            if inner_m + 1 == m {
                count += inner_count;
            } else if inner_m + 1 < m {
                m = inner_m + 1;
                count = inner_count;
            }
        }
    }

    (m, count)
}

fn part2(containers: &[u32], expected: u32) -> usize {
    inner_part2(containers, expected).1
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let parsed = parse(&input_str);
    let part1_result = part1(&parsed, 150);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parsed, 150);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse1_test() {
        assert_eq!(super::part1(&vec![20, 15, 10, 5, 5], 25), 4);
    }

    #[test]
    fn parse2_test() {
        assert_eq!(super::inner_part2(&vec![20, 15, 10, 5, 5], 25), (2, 3));
    }
}
