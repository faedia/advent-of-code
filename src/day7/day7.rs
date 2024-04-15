use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Clone)]
enum Command {
    AssignLit(u16),
    AssignVar(String),
    And(String, String),
    Or(String, String),
    Not(String),
    LShift(String, u16),
    RShift(String, u16),
}

fn parse_command(s: &str) -> Result<(String, Command), String> {
    let split = s.trim().split(' ').collect::<Vec<_>>();
    match split.as_slice() {
        [a, "->", b] if matches!(a.parse::<u16>(), Ok(_)) => {
            Ok((b.to_string(), Command::AssignLit(a.parse::<u16>().unwrap())))
        }
        [a, "->", b] => Ok((b.to_string(), Command::AssignVar(a.to_string()))),
        [a, "AND", b, "->", c] => Ok((c.to_string(), Command::And(a.to_string(), b.to_string()))),
        [a, "OR", b, "->", c] => Ok((c.to_string(), Command::Or(a.to_string(), b.to_string()))),
        [a, "LSHIFT", b, "->", c] if matches!(b.parse::<u16>(), Ok(_)) => Ok((
            c.to_string(),
            Command::LShift(a.to_string(), b.parse::<u16>().unwrap()),
        )),
        [a, "RSHIFT", b, "->", c] if matches!(b.parse::<u16>(), Ok(_)) => Ok((
            c.to_string(),
            Command::RShift(a.to_string(), b.parse::<u16>().unwrap()),
        )),
        ["NOT", a, "->", b] => Ok((b.to_string(), Command::Not(a.to_string()))),
        _ => Err(format!("Error in {}", s)),
    }
}

fn parse(s: &str) -> Result<HashMap<String, Command>, String> {
    s.lines().map(parse_command).collect()
}

fn process_commands(commands: &HashMap<String, Command>, wires: &mut HashMap<String, u16>) {
    let mut deque = commands.keys().collect::<VecDeque<_>>();

    while !deque.is_empty() {
        let wire = deque.front().unwrap();
        if let Ok(n) = wire.parse::<u16>() {
            wires.insert((*wire).clone(), n);
            deque.pop_front();
        } else {
            match commands.get(*wire).unwrap() {
                Command::AssignLit(n) => {
                    wires.insert((*wire).clone(), *n);
                    deque.pop_front();
                }
                Command::AssignVar(a) => {
                    if let Some(n) = wires.get(a) {
                        wires.insert((*wire).clone(), *n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::And(a, b) => {
                    if let Some(n_a) = wires.get(a) {
                        if let Some(n_b) = wires.get(b) {
                            wires.insert((*wire).clone(), n_a & n_b);
                            deque.pop_front();
                        } else {
                            deque.push_front(b);
                        }
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::Or(a, b) => {
                    if let Some(n_a) = wires.get(a) {
                        if let Some(n_b) = wires.get(b) {
                            wires.insert((*wire).clone(), n_a | n_b);
                            deque.pop_front();
                        } else {
                            deque.push_front(b);
                        }
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::Not(a) => {
                    if let Some(n) = wires.get(a) {
                        wires.insert((*wire).clone(), !n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::LShift(a, n) => {
                    if let Some(n_a) = wires.get(a) {
                        wires.insert((*wire).clone(), n_a << n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::RShift(a, n) => {
                    if let Some(n_a) = wires.get(a) {
                        wires.insert((*wire).clone(), n_a >> n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
            }
        }
    }
}

fn part1(commands: &HashMap<String, Command>, wire: String) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    process_commands(commands, &mut wires);

    *wires.get(&wire).unwrap()
}

fn part2(commands: &HashMap<String, Command>, wire: String) -> u16 {
    let mut wires: HashMap<String, u16> = HashMap::new();
    process_commands(commands, &mut wires);

    let new_b = *wires.get(&wire).unwrap();

    wires.clear();
    wires.insert("b".to_string(), new_b);
    process_commands(commands, &mut wires);
    *wires.get(&wire).unwrap()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(&parse(&input_str).unwrap(), "a".to_string());
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parse(&input_str).unwrap(), "a".to_string());
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_command_test() {
        assert_eq!(
            super::parse_command("123 -> x"),
            Ok((String::from("x"), super::Command::AssignLit(123)))
        );
        assert_eq!(
            super::parse_command("x AND y -> z"),
            Ok((
                String::from("z"),
                super::Command::And(String::from("x"), String::from("y"))
            ))
        );
        assert_eq!(
            super::parse_command("a OR b -> c"),
            Ok((
                String::from("c"),
                super::Command::Or(String::from("a"), String::from("b"))
            ))
        );
        assert_eq!(
            super::parse_command("p LSHIFT 2 -> q"),
            Ok((
                String::from("q"),
                super::Command::LShift(String::from("p"), 2)
            ))
        );
        assert_eq!(
            super::parse_command("r RSHIFT 3 -> s"),
            Ok((
                String::from("s"),
                super::Command::RShift(String::from("r"), 3)
            ))
        );
        assert_eq!(
            super::parse_command("NOT e -> f"),
            Ok((String::from("f"), super::Command::Not(String::from("e"))))
        );
    }

    use std::collections::HashMap;

    #[test]
    fn process_test() {
        let data = super::parse(
            "123 -> x
            456 -> y
            x AND y -> d
            x OR y -> e
            x LSHIFT 2 -> f
            y RSHIFT 2 -> g
            NOT x -> h
            NOT y -> i",
        );
        assert!(data.is_ok());
        let mut wires = HashMap::new();
        super::process_commands(&data.unwrap(), &mut wires);
        assert_eq!(wires["d"], 72);
        assert_eq!(wires["e"], 507);
        assert_eq!(wires["f"], 492);
        assert_eq!(wires["g"], 114);
        assert_eq!(wires["h"], 65412);
        assert_eq!(wires["i"], 65079);
        assert_eq!(wires["x"], 123);
        assert_eq!(wires["y"], 456);
    }
}
