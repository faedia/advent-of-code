use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn do_line(s: &str) -> i64 {
    let mut in_str = false;
    let mut in_escape = false;
    let mut in_hex_code = false;
    let mut hex_acc = 0;
    let mut acc = 0;
    for c in s.chars() {
        match c {
            '\"' => {
                if in_escape {
                    acc = acc + 1;
                    in_escape = false;
                } else {
                    in_str = !in_str;
                }
            }
            '\\' => {
                if in_escape {
                    acc = acc + 1;
                    in_escape = false;
                } else if in_str {
                    in_escape = true;
                }
            }
            'x' => {
                if in_escape {
                    in_hex_code = true;
                    hex_acc = 0
                } else {
                    acc = acc + 1;
                }
            }
            '0'..='9' | 'a'..='f' => {
                if in_hex_code {
                    if hex_acc == 1 {
                        acc = acc + 1;
                        in_escape = false;
                        in_hex_code = false;
                        hex_acc = 0;
                    } else {
                        hex_acc = 1;
                    }
                } else if in_str {
                    acc = acc + 1;
                }
            }
            _ => {
                if in_str {
                    acc = acc + 1;
                }
            }
        }
    }

    acc
}

fn part1(s: &str) -> i64 {
    let mut acc = 0;
    for line in s.lines() {
        acc = acc + (line.len() as i64 - do_line(line));
    }
    acc
}

fn expand_str(s: &str) -> i64 {
    let mut acc = 2;
    for c in s.chars() {
        match c {
            '\"' => acc = acc + 2,
            '\\' => acc = acc + 2,
            _ => acc = acc + 1,
        }
    }

    acc
}

fn part2(s: &str) -> i64 {
    let mut acc: i64 = 0;
    for line in s.lines() {
        acc = acc + (expand_str(line) - line.len() as i64)
    }

    acc
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
        assert_eq!(
            super::part1("\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"\n"),
            12
        );
        assert_eq!(super::part1("\"m\""), 2);
        assert_eq!(super::part1("\"hxilg\\\\\""), 3);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2("\"\""), 4);
        assert_eq!(super::part2("\"abc\""), 4);
        assert_eq!(super::part2("\"aaa\\\"aaa\""), 6);
        assert_eq!(super::part2("\"\\x27\""), 5);
    }
}
