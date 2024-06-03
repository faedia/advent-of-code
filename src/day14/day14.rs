use clap::Parser;
use itertools::Itertools;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Parser)]
struct Cli {
    input: String,
}
#[derive(PartialEq, Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
}

struct ReindeerState {
    flying: bool,
    next_change: i32,
    distance: i32,
    points: i32,
}

fn parse(s: &str) -> Vec<Reindeer> {
    s.lines()
        .map(|line| {
            let split = line.split(' ').collect_vec();
            Reindeer {
                name: split[0].to_string(),
                speed: split[3].parse().unwrap(),
                fly_time: split[6].parse().unwrap(),
                rest_time: split[13].parse().unwrap(),
            }
        })
        .collect()
}

fn part1(reindeers: &Vec<Reindeer>, time: i32) -> i32 {
    let mut state = HashMap::new();
    for reindeer in reindeers {
        state.insert(
            reindeer.name.clone(),
            ReindeerState {
                flying: false,
                next_change: 0,
                distance: 0,
                points: 0,
            },
        );
    }

    for t in 0..time {
        for reindeer in reindeers {
            let current_state: &mut ReindeerState = state.get_mut(&reindeer.name).unwrap();
            if current_state.next_change == t {
                if current_state.flying {
                    current_state.next_change = t + reindeer.rest_time;
                } else {
                    current_state.next_change = t + reindeer.fly_time;
                }
                current_state.flying = !current_state.flying;
            }
            if current_state.flying {
                current_state.distance = current_state.distance + reindeer.speed;
            }
        }
    }

    state.values().map(|s| s.distance).max().unwrap()
}

fn part2(reindeers: &Vec<Reindeer>, time: i32) -> i32 {
    let mut state = HashMap::new();
    for reindeer in reindeers {
        state.insert(
            reindeer.name.clone(),
            ReindeerState {
                flying: false,
                next_change: 0,
                distance: 0,
                points: 0,
            },
        );
    }

    for t in 0..time {
        let mut leaders: Vec<String> = vec![];

        for reindeer in reindeers {
            {
                let current_state: &mut ReindeerState = state.get_mut(&reindeer.name).unwrap();
                if current_state.next_change == t {
                    if current_state.flying {
                        current_state.next_change = t + reindeer.rest_time;
                    } else {
                        current_state.next_change = t + reindeer.fly_time;
                    }
                    current_state.flying = !current_state.flying;
                }
                if current_state.flying {
                    current_state.distance = current_state.distance + reindeer.speed;
                }
            }

            if let Some(leader_state) = leaders.get(0).and_then(|name| state.get(name)) {
                let current_state = state.get(&reindeer.name).unwrap();
                if leader_state.distance == current_state.distance {
                    leaders.push(reindeer.name.clone());
                } else if leader_state.distance < current_state.distance {
                    leaders.clear();
                    leaders.push(reindeer.name.clone());
                }
            } else {
                leaders.push(reindeer.name.clone());
            }
        }

        for leader in leaders {
            let current_state = state.get_mut(&leader).unwrap();
            current_state.points = current_state.points + 1;
        }
    }

    state.values().map(|s| s.points).max().unwrap()
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let reindeers = parse(&input_str);
    let part1_result = part1(&reindeers, 2503);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&reindeers, 2503);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_test() {
        assert_eq!(
            super::parse(
                "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."
            ),
            vec![
                super::Reindeer {
                    name: "Comet".to_string(),
                    speed: 14,
                    fly_time: 10,
                    rest_time: 127
                },
                super::Reindeer {
                    name: "Dancer".to_string(),
                    speed: 16,
                    fly_time: 11,
                    rest_time: 162
                }
            ]
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            super::part1(
                &vec![
                    super::Reindeer {
                        name: "Comet".to_string(),
                        speed: 14,
                        fly_time: 10,
                        rest_time: 127
                    },
                    super::Reindeer {
                        name: "Dancer".to_string(),
                        speed: 16,
                        fly_time: 11,
                        rest_time: 162
                    }
                ],
                1000
            ),
            1120
        )
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            super::part2(
                &vec![
                    super::Reindeer {
                        name: "Comet".to_string(),
                        speed: 14,
                        fly_time: 10,
                        rest_time: 127
                    },
                    super::Reindeer {
                        name: "Dancer".to_string(),
                        speed: 16,
                        fly_time: 11,
                        rest_time: 162
                    }
                ],
                1000
            ),
            689
        )
    }
}
