use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
    str::Lines,
    time::{self, SystemTime},
};

#[derive(Parser)]
struct Cli {
    input: String,
}

fn parse_replacements(lines: &mut Lines) -> HashMap<String, Vec<String>> {
    let mut replacements: HashMap<String, Vec<String>> = HashMap::new();

    lines.take_while(|s| *s != "").for_each(|s| {
        let (key, value) = s.split(" => ").collect_tuple().unwrap();
        if let Some(v) = replacements.get_mut(key) {
            v.push(value.to_string());
        } else {
            replacements.insert(key.to_string(), vec![value.to_string()]);
        }
    });

    replacements
}

fn parse(s: &str) -> (HashMap<String, Vec<String>>, String) {
    let mut lines = s.lines();
    let replacements = parse_replacements(&mut lines);

    (replacements, lines.next().unwrap().to_string())
}

fn part1(init: &(HashMap<String, Vec<String>>, String)) -> usize {
    let mut possible: HashSet<String> = HashSet::new();
    let start_string = &init.1;

    for (key, value) in &init.0 {
        let matches = start_string.match_indices(key);
        for m in matches {
            for v in value {
                let mut inner_string = start_string.clone();
                inner_string.replace_range(m.0..m.0 + key.len(), v);
                possible.insert(inner_string);
            }
        }
    }
    possible.len()
}

fn inverse_map(m: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();

    for (key, values) in m {
        for new_key in values {
            if let Some(v) = result.get_mut(new_key) {
                v.push(key.clone());
            } else {
                result.insert(new_key.clone(), vec![key.clone()]);
            }
        }
    }

    result
}

fn part2(init: &(HashMap<String, Vec<String>>, String), start: String) -> usize {
    let mut seen_set: HashSet<String> = HashSet::from([init.1.clone()]);
    let mut queue: VecDeque<(String, usize)> = VecDeque::from([(init.1.clone(), 0)]);
    let mut current_steps = 0;
    let mut current_step_time = SystemTime::now();

    let inv = inverse_map(&init.0);

    while queue.front().unwrap().0 != start {
        let (current_string, size) = queue.pop_front().unwrap();
        if current_steps != size {
            println!(
                "Completed step {} in {:?}",
                size,
                current_step_time.elapsed()
            );
            current_step_time = SystemTime::now();
            current_steps = size;
        }

        println!("size {} current {}", size, current_string.len());

        for (key, values) in &inv {
            let mut matches = current_string.match_indices(key);
            if let Some((idx, _)) = matches.next() {
                let value = values.first().unwrap();

                let mut inner_string = current_string.clone();
                inner_string.replace_range(idx..idx + key.len(), value);
                seen_set.insert(inner_string.clone());
                queue.push_back((inner_string, size + 1));
                break;
            }
        }
    }

    queue.front().unwrap().1
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let parsed = parse(&input_str);
    let part1_result = part1(&parsed);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parsed, "e".to_string());
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse1_test() {
        let parsed = super::parse(
            "H => HO
H => OH
O => HH

HOH",
        );
        assert_eq!(super::part1(&parsed), 4);
    }

    #[test]
    fn parse2_test() {
        let parsed = super::parse(
            "e => H
e => O
H => HO
H => OH
O => HH

HOH",
        );
        assert_eq!(super::part2(&parsed, "e".to_string()), 3);
    }
}
