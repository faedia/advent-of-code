use clap::Parser;
use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(PartialEq, Debug)]
struct Traits {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

fn do_check(kind: &str, actual: u32, expected: u32) -> bool {
    if ["cats", "trees"].contains(&kind) {
        actual > expected
    } else if ["pomeranian", "goldfish"].contains(&kind) {
        actual < expected
    } else {
        actual == expected
    }
}

fn parse_single(s: &str) -> HashMap<String, u32> {
    let mut t = HashMap::new();

    let split = s
        .split(' ')
        .skip(2)
        .map(|s| s.trim_end_matches([':', ',']))
        .chunks(2);

    for mut chunk in &split {
        let (name, count) = chunk.next_tuple().unwrap();
        t.insert(name.to_string(), count.parse().unwrap());
    }

    t
}

fn parse(s: &str) -> Vec<HashMap<String, u32>> {
    s.lines().map(parse_single).collect::<Vec<_>>()
}

fn part1(sues: &Vec<HashMap<String, u32>>, expected_sue: HashMap<String, u32>) -> usize {
    for (idx, sue) in sues.iter().enumerate() {
        if sue
            .iter()
            .all(|(key, value)| expected_sue.get(key) == Some(value))
        {
            return idx + 1;
        }
    }

    0
}

fn part2(sues: &Vec<HashMap<String, u32>>, expected_sue: HashMap<String, u32>) -> usize {
    for (idx, sue) in sues.iter().enumerate() {
        if sue.iter().all(|(key, value)| {
            if let Some(expected) = expected_sue.get(key) {
                do_check(key, *value, *expected)
            } else {
                true
            }
        }) {
            return idx + 1;
        }
    }

    0
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let parsed = parse(&input_str);
    let part1_result = part1(
        &parsed,
        HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]),
    );
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(
        &parsed,
        HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]),
    );
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn parse_test() {
        assert_eq!(
            super::parse_single("Sue 1: cars: 9, akitas: 3, goldfish: 0"),
            HashMap::from([
                ("cars".to_string(), 9),
                ("akitas".to_string(), 3),
                ("goldfish".to_string(), 0),
            ])
        )
    }
}
