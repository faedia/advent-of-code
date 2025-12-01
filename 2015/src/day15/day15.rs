use clap::Parser;
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(PartialEq, Debug, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse(s: &str) -> Vec<Ingredient> {
    s.lines()
        .map(|line| {
            let split = line
                .split(' ')
                .map(|s| s.trim_end_matches(','))
                .collect_vec();
            Ingredient {
                capacity: split[2].parse().unwrap(),
                durability: split[4].parse().unwrap(),
                flavor: split[6].parse().unwrap(),
                texture: split[8].parse().unwrap(),
                calories: split[10].parse().unwrap(),
            }
        })
        .collect()
}

fn part1(ingredients: &Vec<Ingredient>, amount: usize) -> i64 {
    let mut max = 0;
    for a in 0..=amount {
        for b in 0..=(amount - a) {
            for c in 0..=(amount - a - b) {
                let d = amount - a - b - c;
                let capacity = ingredients[0].capacity * a as i64
                    + ingredients[1].capacity * b as i64
                    + ingredients[2].capacity * c as i64
                    + ingredients[3].capacity * d as i64;
                let durability = ingredients[0].durability * a as i64
                    + ingredients[1].durability * b as i64
                    + ingredients[2].durability * c as i64
                    + ingredients[3].durability * d as i64;
                let flavor = ingredients[0].flavor * a as i64
                    + ingredients[1].flavor * b as i64
                    + ingredients[2].flavor * c as i64
                    + ingredients[3].flavor * d as i64;
                let texture = ingredients[0].texture * a as i64
                    + ingredients[1].texture * b as i64
                    + ingredients[2].texture * c as i64
                    + ingredients[3].texture * d as i64;
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                    continue;
                }
                let score = capacity * durability * flavor * texture;
                if score > max {
                    max = score;
                }
            }
        }
    }

    max
}

fn part2(ingredients: &Vec<Ingredient>, amount: usize) -> i64 {
    let mut max = 0;
    for a in 0..=amount {
        for b in 0..=(amount - a) {
            for c in 0..=(amount - a - b) {
                let d = amount - a - b - c;
                let capacity = ingredients[0].capacity * a as i64
                    + ingredients[1].capacity * b as i64
                    + ingredients[2].capacity * c as i64
                    + ingredients[3].capacity * d as i64;
                let durability = ingredients[0].durability * a as i64
                    + ingredients[1].durability * b as i64
                    + ingredients[2].durability * c as i64
                    + ingredients[3].durability * d as i64;
                let flavor = ingredients[0].flavor * a as i64
                    + ingredients[1].flavor * b as i64
                    + ingredients[2].flavor * c as i64
                    + ingredients[3].flavor * d as i64;
                let texture = ingredients[0].texture * a as i64
                    + ingredients[1].texture * b as i64
                    + ingredients[2].texture * c as i64
                    + ingredients[3].texture * d as i64;
                let calories = ingredients[0].calories * a as i64
                    + ingredients[1].calories * b as i64
                    + ingredients[2].calories * c as i64
                    + ingredients[3].calories * d as i64;
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 || calories != 500 {
                    continue;
                }
                let score = capacity * durability * flavor * texture;
                if score > max {
                    max = score;
                }
            }
        }
    }

    max
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let parsed = parse(&input_str);
    let part1_result = part1(&parsed, 100);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&parsed, 100);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {
    use crate::Ingredient;

    #[test]
    fn parse_test() {
        assert_eq!(
            super::parse(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
            ),
            vec![
                Ingredient {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8
                },
                Ingredient {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3
                }
            ]
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            super::part1(
                &vec![
                    Ingredient {
                        capacity: -1,
                        durability: -2,
                        flavor: 6,
                        texture: 3,
                        calories: 8
                    },
                    Ingredient {
                        capacity: 2,
                        durability: 3,
                        flavor: -2,
                        texture: -1,
                        calories: 3
                    },
                    Ingredient {
                        capacity: 0,
                        durability: 0,
                        flavor: 0,
                        texture: 0,
                        calories: 0
                    },
                    Ingredient {
                        capacity: 0,
                        durability: 0,
                        flavor: 0,
                        texture: 0,
                        calories: 0
                    }
                ],
                100
            ),
            62842880
        );
    }
}
