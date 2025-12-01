use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<u32>,
    transitions: HashMap<(u32, u32), i64>,
    names: HashMap<u32, String>,
    names_to_idx: HashMap<String, u32>,
}

fn get_or_add(s: &str, g: &mut Graph, next_idx: &mut u32) -> u32 {
    if let Some(idx) = g.names_to_idx.get(s) {
        *idx
    } else {
        g.nodes.insert(*next_idx);
        g.names.insert(*next_idx, s.to_string());
        g.names_to_idx.insert(s.to_string(), *next_idx);
        *next_idx = *next_idx + 1;
        *next_idx - 1
    }
}

fn parse(s: &str) -> Graph {
    let mut g = Graph {
        nodes: HashSet::new(),
        transitions: HashMap::new(),
        names: HashMap::new(),
        names_to_idx: HashMap::new(),
    };
    let mut current_idx = 1;

    for line in s.lines() {
        let vec = line.split(' ').collect::<Vec<_>>();
        let lhs_idx = get_or_add(vec[0], &mut g, &mut current_idx);
        let rhs_idx = get_or_add(
            vec.last().unwrap().trim_end_matches('.'),
            &mut g,
            &mut current_idx,
        );
        let value = if vec[2] == "gain" {
            vec[3].parse::<i64>().unwrap()
        } else {
            -vec[3].parse::<i64>().unwrap()
        };
        g.transitions.insert((lhs_idx, rhs_idx), value);
    }

    g
}

fn calculate(permutation: &Vec<&u32>, g: &Graph) -> i64 {
    permutation
        .iter()
        .zip(permutation.iter().skip(1))
        .map(|(a, b)| {
            *g.transitions.get(&(**b, **a)).unwrap_or(&0)
                + *g.transitions.get(&(**a, **b)).unwrap_or(&0)
        })
        .sum()
}

fn part1(g: &Graph) -> i64 {
    g.nodes
        .iter()
        .permutations(g.nodes.len())
        .map(|mut permutation| {
            permutation.push(permutation[0]);
            calculate(&permutation, g)
        })
        .max()
        .unwrap_or(0)
}

fn part2(mut g: Graph) -> i64 {
    let mut test_idx = (g.nodes.len() + 1) as u32;
    let idx = get_or_add("Oh Look Its me", &mut g, &mut test_idx);
    for node in g.nodes.iter() {
        g.transitions.insert((*node, idx), 0);
        g.transitions.insert((idx, *node), 0);
    }
    part1(&g)
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let g = parse(&input_str);
    let part1_result = part1(&g);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(g);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_test() {
        let g1 = super::parse(
            "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        );
        assert_eq!(super::part1(&g1), 330);
    }
}
