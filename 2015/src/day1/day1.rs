use std::fs::read_to_string;

use clap::Parser;

fn part1(s: &str) -> i64 {
    s.chars()
        .fold(0, |count, c| if c == '(' { count + 1 } else { count - 1 })
}

fn part2(s: &str) -> usize {
    s.chars()
        .fold((0, 0), |state, c| {
            if state.0 == -1 {
                state
            } else {
                (state.0 + if c == '(' { 1 } else { -1 }, state.1 + 1)
            }
        })
        .1
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
        assert_eq!(super::part1("(())"), 0);
        assert_eq!(super::part1("()()"), 0);
        assert_eq!(super::part1("((("), 3);
        assert_eq!(super::part1("(()(()("), 3);
        assert_eq!(super::part1("))((((("), 3);
        assert_eq!(super::part1("())"), -1);
        assert_eq!(super::part1("))("), -1);
        assert_eq!(super::part1(")))"), -3);
        assert_eq!(super::part1(")())())"), -3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2(")"), 1);
        assert_eq!(super::part2("()())"), 5);
    }
}
