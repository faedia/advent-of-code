use clap::Parser;
use md5::{Digest, Md5};
use std::fs::read_to_string;

fn part1(s: &str) -> usize {
    for i in 0..std::usize::MAX {
        let mut hasher = Md5::new();
        hasher.update(s);
        hasher.update(i.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] < 16 {
            return i;
        }
    }
    0
}

fn part2(s: &str) -> usize {
    for i in 0..std::usize::MAX {
        let mut hasher = Md5::new();
        hasher.update(s);
        hasher.update(i.to_string());
        let result = hasher.finalize_reset();
        if result[0] == 0 && result[1] == 0 && result[2] == 0 {
            return i;
        }
    }
    0
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
        assert_eq!(super::part1("abcdef"), 609043);
        assert_eq!(super::part1("pqrstuv"), 1048970);
    }
}
