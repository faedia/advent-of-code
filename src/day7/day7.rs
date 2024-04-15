use clap::Parser;
use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

#[derive(Debug, PartialEq, Clone)]
enum Command<'a> {
    AssignLit(u16),
    AssignVar(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Not(&'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
}

fn parse_command(s: &str) -> Result<(&str, Command), String> {
    let split = s.trim().split(' ').collect::<Vec<_>>();
    match split.as_slice() {
        [a, "->", b] if matches!(a.parse::<u16>(), Ok(_)) => {
            Ok((b, Command::AssignLit(a.parse::<u16>().unwrap())))
        }
        [a, "->", b] => Ok((b, Command::AssignVar(a))),
        [a, "AND", b, "->", c] => Ok((c, Command::And(a, b))),
        [a, "OR", b, "->", c] => Ok((c, Command::Or(a, b))),
        [a, "LSHIFT", b, "->", c] if matches!(b.parse::<u16>(), Ok(_)) => {
            Ok((c, Command::LShift(a, b.parse::<u16>().unwrap())))
        }
        [a, "RSHIFT", b, "->", c] if matches!(b.parse::<u16>(), Ok(_)) => {
            Ok((c, Command::RShift(a, b.parse::<u16>().unwrap())))
        }
        ["NOT", a, "->", b] => Ok((b, Command::Not(a))),
        _ => Err(format!("Error in {}", s)),
    }
}

fn parse(s: &str) -> Result<HashMap<&str, Command>, String> {
    s.lines().map(parse_command).collect()
}

fn process_commands<'a>(commands: &'a HashMap<&str, Command>, wires: &mut HashMap<&'a str, u16>) {
    let mut deque = commands.keys().collect::<VecDeque<_>>();

    while !deque.is_empty() {
        let wire = deque.front().unwrap();
        if let Ok(n) = wire.parse::<u16>() {
            wires.insert(wire, n);
            deque.pop_front();
        } else {
            match commands.get(*wire).unwrap() {
                Command::AssignLit(n) => {
                    wires.insert(wire, *n);
                    deque.pop_front();
                }
                Command::AssignVar(a) => {
                    if let Some(n) = wires.get(a) {
                        wires.insert(wire, *n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::And(a, b) => {
                    if let Some(n_a) = wires.get(a) {
                        if let Some(n_b) = wires.get(b) {
                            wires.insert(wire, n_a & n_b);
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
                            wires.insert(wire, n_a | n_b);
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
                        wires.insert(wire, !n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::LShift(a, n) => {
                    if let Some(n_a) = wires.get(a) {
                        wires.insert(wire, n_a << n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
                Command::RShift(a, n) => {
                    if let Some(n_a) = wires.get(a) {
                        wires.insert(wire, n_a >> n);
                        deque.pop_front();
                    } else {
                        deque.push_front(a);
                    }
                }
            }
        }
    }
}

fn part1(commands: &HashMap<&str, Command>, wire: &str) -> u16 {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    process_commands(commands, &mut wires);

    *wires.get(&wire).unwrap()
}

fn part2(commands: &HashMap<&str, Command>, wire: &str) -> u16 {
    let mut wires: HashMap<&str, u16> = HashMap::new();
    process_commands(commands, &mut wires);

    let new_b = *wires.get(&wire).unwrap();

    wires.clear();
    wires.insert("b", new_b);
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
    let part1_result = part1(&parse(&input_str).unwrap(), "a");
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parse(&input_str).unwrap(), "a");
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_command_test() {
        assert_eq!(
            super::parse_command("123 -> x"),
            Ok(("x", super::Command::AssignLit(123)))
        );
        assert_eq!(
            super::parse_command("x AND y -> z"),
            Ok(("z", super::Command::And("x", "y")))
        );
        assert_eq!(
            super::parse_command("a OR b -> c"),
            Ok(("c", super::Command::Or("a", "b")))
        );
        assert_eq!(
            super::parse_command("p LSHIFT 2 -> q"),
            Ok((("q"), super::Command::LShift("p", 2)))
        );
        assert_eq!(
            super::parse_command("r RSHIFT 3 -> s"),
            Ok((("s"), super::Command::RShift("r", 3)))
        );
        assert_eq!(
            super::parse_command("NOT e -> f"),
            Ok((("f"), super::Command::Not("e")))
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
        let binding = data.unwrap();
        super::process_commands(&binding, &mut wires);
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
