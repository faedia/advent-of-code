use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn single_iter(s: &str) -> String {
    let mut current_char = s.chars().next().unwrap();
    let mut current_count = 1;
    let mut seq = vec![];
    for char in s.chars().skip(1) {
        if char == current_char {
            current_count = current_count + 1;
        } else {
            seq.push((current_char, current_count));
            current_char = char;
            current_count = 1;
        }
    }

    seq.push((current_char, current_count));

    let mut ret = "".to_string();

    for (char, count) in seq {
        ret.push_str(&count.to_string());
        ret.push(char);
    }

    ret
}

fn apply(s: &str, count: u64) -> String {
    let mut current_string = String::from(s);
    for _ in 0..count {
        current_string = single_iter(&current_string);
    }

    current_string
}

fn part1(s: &str) -> usize {
    apply(s, 40).len()
}

fn part2(s: &str) -> usize {
    apply(s, 50).len()
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
        assert_eq!(super::single_iter("1"), "11");
        assert_eq!(super::single_iter("11"), "21");
        assert_eq!(super::single_iter("1211"), "111221");
        assert_eq!(super::single_iter("111221"), "312211");
    }
}
