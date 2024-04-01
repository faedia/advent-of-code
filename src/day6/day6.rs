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

fn do_commands<B, F>(commands: Vec<Command>, mut init: B, mut f: F) -> B
where
    F: FnMut(&mut B, &Command, (usize, usize)) -> (),
{
    for command in commands {
        for y in command.range.from.1..command.range.to.1 + 1 {
            for x in command.range.from.0..command.range.to.0 + 1 {
                f(&mut init, &command, (x, y));
            }
        }
    }

    init
}

fn part1(commands: Vec<Command>) -> usize {
    let mut state = [[false; 1000]; 1000];

    do_commands(
        commands,
        &mut state,
        |state, command, (x, y)| match command.kind {
            CommandKind::TurnOn => {
                state[y][x] = true;
            }
            CommandKind::TurnOff => {
                state[y][x] = false;
            }
            CommandKind::Toggle => {
                state[y][x] = !state[y][x];
            }
        },
    )
    .iter()
    .flatten()
    .filter(|is_on| **is_on)
    .count()
}

fn part2(commands: Vec<Command>) -> i64 {
    let mut state = [[0i64; 1000]; 1000].to_vec();

    do_commands(
        commands,
        &mut state,
        |state, command, (x, y)| match command.kind {
            CommandKind::TurnOn => {
                state[y][x] = state[y][x] + 1;
            }
            CommandKind::TurnOff => {
                state[y][x] = std::cmp::max(state[y][x] - 1, 0);
            }
            CommandKind::Toggle => {
                state[y][x] = state[y][x] + 2;
            }
        },
    )
    .iter()
    .flatten()
    .sum()
}

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let part1_result = part1(input_str.lines().map(parse).collect());
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(input_str.lines().map(parse).collect());
    println!("Solution for part 2: {}", part2_result);
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

    #[test]
    fn part2_test() {
        let vec1 = [super::Command {
            kind: super::CommandKind::TurnOn,
            range: super::Range {
                from: (0, 0),
                to: (0, 0),
            },
        }]
        .to_vec();
        assert_eq!(super::part2(vec1), 1);
        assert_eq!(
            super::part2(
                [super::Command {
                    kind: super::CommandKind::Toggle,
                    range: super::Range {
                        from: (0, 0),
                        to: (999, 999)
                    }
                }]
                .to_vec()
            ),
            2000000
        );
    }
}
