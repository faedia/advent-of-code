use clap::Parser;
use itertools::Itertools;
use serde_json::Value;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn part1(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.iter().fold(0, |acc, v| acc + part1(v)),
        Value::Object(map) => map.values().fold(0, |acc, v| acc + part1(v)),
    }
}

fn part2(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.iter().fold(0, |acc, v| acc + part2(v)),
        Value::Object(map) => {
            if map.values().contains(&Value::String("red".to_string())) {
                0
            } else {
                map.values().fold(0, |acc, v| acc + part2(v))
            }
        }
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let input_json = serde_json::from_str(&input_str).unwrap();
    let part1_result = part1(&input_json);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&input_json);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_test() {
        assert_eq!(super::part1(&serde_json::from_str("[1,2,3]").unwrap()), 6);
        assert_eq!(
            super::part1(&serde_json::from_str("{\"a\":2,\"b\":4}").unwrap()),
            6
        );
        assert_eq!(super::part1(&serde_json::from_str("[[[3]]]").unwrap()), 3);
        assert_eq!(
            super::part1(&serde_json::from_str("{\"a\":{\"b\":4},\"c\":-1}").unwrap()),
            3
        );
        assert_eq!(
            super::part1(&serde_json::from_str("{\"a\":[-1,1]}").unwrap()),
            0
        );
        assert_eq!(
            super::part1(&serde_json::from_str("[-1,{\"a\":1}]").unwrap()),
            0
        );
        assert_eq!(super::part1(&serde_json::from_str("[]").unwrap()), 0);
        assert_eq!(super::part1(&serde_json::from_str("{}").unwrap()), 0);
    }

    #[test]
    fn part2_test() {
        assert_eq!(super::part2(&serde_json::from_str("[1,2,3]").unwrap()), 6);
        assert_eq!(
            super::part2(&serde_json::from_str("[1,{\"c\":\"red\",\"b\":2},3]").unwrap()),
            4
        );
        assert_eq!(
            super::part2(&serde_json::from_str("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}").unwrap()),
            0
        );
        assert_eq!(
            super::part2(&serde_json::from_str("[1,\"red\",5]").unwrap()),
            6
        );
    }
}
