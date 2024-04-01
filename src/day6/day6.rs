use clap::Parser;
use std::fs::read_to_string;

#[derive(PartialEq, Debug, Clone)]
struct Range {
    from: (usize, usize),
    to: (usize, usize),
}

#[derive(PartialEq, Debug, Clone)]
enum CommandKind {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(PartialEq, Debug, Clone)]
struct Command {
    kind: CommandKind,
    range: Range,
}

fn parse(s: &str) -> Command {
    let split = s.split(',').collect::<Vec<&str>>();

    let range_to_y = split[2].parse::<usize>().unwrap();
    let through_split = split[1].split(" through ").collect::<Vec<&str>>();
    let range_to_x = through_split[1].parse::<usize>().unwrap();
    let range_from_y = through_split[0].parse::<usize>().unwrap();

    let (command_str, range_from_x_str) = split[0].rsplit_once(' ').unwrap();
    let range_from_x = range_from_x_str.parse::<usize>().unwrap();

    let range = Range {
        from: (range_from_x, range_from_y),
        to: (range_to_x, range_to_y),
    };

    if command_str == "turn on" {
        Command {
            kind: CommandKind::TurnOn,
            range: range,
        }
    } else if command_str == "turn off" {
        Command {
            kind: CommandKind::TurnOff,
            range: range,
        }
    } else if command_str == "toggle" {
        Command {
            kind: CommandKind::Toggle,
            range: range,
        }
    } else {
        panic!("Unknown command type {}", command_str);
    }
}

fn parse_all(s: &str) -> Vec<Command> {
    s.lines().map(parse).collect()
}

fn part1(commands: Vec<Command>) -> usize {
    let mut state = [[false; 1000]; 1000];

    for command in commands {
        for y in command.range.from.1..command.range.to.1 + 1 {
            for x in command.range.from.0..command.range.to.0 + 1 {
                match command.kind {
                    CommandKind::TurnOn => {
                        state[y][x] = true;
                    }
                    CommandKind::TurnOff => {
                        state[y][x] = false;
                    }
                    CommandKind::Toggle => {
                        state[y][x] = !state[y][x];
                    }
                }
            }
        }
    }

    state.iter().flatten().filter(|is_on| **is_on).count()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(parse_all(&input_str));
    println!("Solution for part 1: {}", part1_result);
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_test() {
        assert_eq!(
            super::parse("turn on 0,0 through 999,999"),
            super::Command {
                kind: super::CommandKind::TurnOn,
                range: super::Range {
                    from: (0, 0),
                    to: (999, 999)
                }
            }
        );
        assert_eq!(
            super::parse("toggle 0,0 through 999,0"),
            super::Command {
                kind: super::CommandKind::Toggle,
                range: super::Range {
                    from: (0, 0),
                    to: (999, 0)
                }
            }
        );
        assert_eq!(
            super::parse("turn off 499,499 through 500,500"),
            super::Command {
                kind: super::CommandKind::TurnOff,
                range: super::Range {
                    from: (499, 499),
                    to: (500, 500)
                }
            }
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            super::part1(
                [super::Command {
                    kind: super::CommandKind::TurnOn,
                    range: super::Range {
                        from: (0, 0),
                        to: (999, 999)
                    }
                }]
                .to_vec()
            ),
            1000 * 1000
        );
        assert_eq!(
            super::part1(
                [super::Command {
                    kind: super::CommandKind::Toggle,
                    range: super::Range {
                        from: (0, 0),
                        to: (999, 0)
                    }
                }]
                .to_vec()
            ),
            1000
        );
        assert_eq!(
            super::part1(
                [super::Command {
                    kind: super::CommandKind::TurnOff,
                    range: super::Range {
                        from: (499, 499),
                        to: (500, 500)
                    }
                }]
                .to_vec()
            ),
            0
        );
    }
}
