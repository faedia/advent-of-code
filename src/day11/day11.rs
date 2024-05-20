use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn check_password(s: &Vec<u8>) -> bool {
    s.iter()
        .zip(s.iter().skip(1).zip(s.iter().skip(2)))
        .any(|(a, (b, c))| a + 1 == *b && b + 1 == *c)
        && s.iter().all(|c| !"iol".contains(*c as char))
        && s.iter()
            .zip(s.iter().skip(1))
            .enumerate()
            .any(|(idx, (a, b))| {
                a == b
                    && s.iter()
                        .zip(s.iter().skip(1))
                        .skip(idx + 2)
                        .any(|(a1, b1)| a1 == b1)
            })
}

fn increment(s: &mut [u8]) {
    for idx in (0..s.len()).rev() {
        if s[idx] != b'z' {
            s[idx] = s[idx] + 1;
            if "iol".contains(s[idx] as char) {
                s[idx] = s[idx] + 1
            }
            break;
        } else {
            s[idx] = b'a'
        }
    }
}

fn part1(s: &str) -> String {
    let mut s_bytes = s.as_bytes().to_vec();
    increment(&mut s_bytes);
    while !check_password(&s_bytes) {
        increment(&mut s_bytes);
    }
    String::from_utf8(s_bytes).unwrap()
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(&input_str);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part1(&part1_result);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use crate::{check_password, increment};

    fn check_password_str(s: &str) -> bool {
        check_password(&s.as_bytes().to_vec())
    }

    #[test]
    fn part1_test() {
        assert!(!check_password_str("hijklmmn"));
        assert!(!check_password_str("abbceffg"));
        assert!(!check_password_str("abbcegjk"));
        assert!(check_password_str("abcdffaa"));
        assert!(check_password_str("ghjaabcc"));
        assert!(!check_password_str("ghjaabcd"));
        assert!(check_password_str("ghjaabcc"));
        assert!(check_password_str("abcaaccd"));
        assert!(check_password_str("abcaaaad"));
        assert!(!check_password_str("abcaaand"));
        let mut bs = b"hijklmmn".to_vec();
        increment(&mut bs);
        assert_eq!(bs, b"hijklmmp".to_vec());
        bs = b"hijklmmz".to_vec();
        increment(&mut bs);
        bs = b"hzzzzzzz".to_vec();
        increment(&mut bs);
        assert_eq!(bs, b"jaaaaaaa".to_vec());
    }
}
