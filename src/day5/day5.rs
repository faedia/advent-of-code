use clap::Parser;
use std::fs::read_to_string;

fn part1<'a, I>(input: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    input
        .map(|s| {
            let vowels = s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3;
            let not_contain =
                !s.contains("ab") && !s.contains("cd") && !s.contains("pq") && !s.contains("xy");
            let repeat = s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b);
            vowels && not_contain && repeat
        })
        .filter(|&b| b)
        .count()
}

fn part2<'a, I>(input: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    input
        .map(|s| {
            let pairs = s.chars().zip(s.chars().skip(1));
            println!("{:?}", pairs.collect::<Vec<(char, char)>>());
            false
        })
        .filter(|&b| b)
        .count()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(input_str.lines());
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(input_str.lines());
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test() {
        assert_eq!(super::part1("ugknbfddgicrmopn".lines()), 1);
        assert_eq!(super::part1("jchzalrnumimnmhp".lines()), 0);
        assert_eq!(super::part1("haegwjzuvuyypxyu".lines()), 0);
        assert_eq!(super::part1("dvszwmarrgswjxmb".lines()), 0);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2("xyxy".lines()), 1);
        assert_eq!(super::part2("aabcdefgaa".lines()), 1);
        assert_eq!(super::part2("aaa".lines()), 1);
        assert_eq!(super::part2("qjhvhtzxzqqjkmpb".lines()), 1);
        assert_eq!(super::part2("uurcxstgmygtbstg".lines()), 0);
        assert_eq!(super::part2("ieodomkazucvgmuy".lines()), 0);
    }
}
