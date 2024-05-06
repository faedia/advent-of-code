use clap::Parser;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};

#[derive(Parser)]
struct Cli {
    input: String,
}

#[derive(Debug)]
struct Graph {
    transitions: HashMap<u64, HashMap<u64, u64>>,
    node_id_to_string: HashMap<u64, String>,
    string_to_node_id: HashMap<String, u64>,
    next_id: u64,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            transitions: HashMap::new(),
            node_id_to_string: HashMap::new(),
            string_to_node_id: HashMap::new(),
            next_id: 1,
        }
    }

    fn get_or_add(&mut self, s: &str) -> u64 {
        if let Some(id) = self.string_to_node_id.get(s) {
            *id
        } else {
            self.string_to_node_id.insert(s.to_string(), self.next_id);
            self.node_id_to_string.insert(self.next_id, s.to_string());
            let id = self.next_id;
            self.next_id = self.next_id + 1;
            id
        }
    }

    fn add_transition(&mut self, src: &str, dest: &str, cost: u64) {
        let src_id = self.get_or_add(src);
        let dest_id = self.get_or_add(dest);
        if let Some(dests) = self.transitions.get_mut(&src_id) {
            dests.insert(dest_id, cost);
        } else {
            self.transitions
                .insert(src_id, HashMap::from([(dest_id, cost)]));
        }
        if let Some(dests) = self.transitions.get_mut(&dest_id) {
            dests.insert(src_id, cost);
        } else {
            self.transitions
                .insert(dest_id, HashMap::from([(src_id, cost)]));
        }
    }
}

fn parse(s: &str) -> Graph {
    let mut g = Graph::new();
    let re = Regex::new(r"([A-Za-z]+) to ([A-Za-z]+) = ([0-9]+)").unwrap();
    for line in s.lines() {
        let caps = re.captures(line).unwrap();
        let src = caps.get(1).unwrap().as_str();
        let dest = caps.get(2).unwrap().as_str();
        let cost = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
        g.add_transition(src, dest, cost)
    }
    g
}

fn cost_of_route(route: &Vec<&u64>, g: &Graph) -> u64 {
    let mut sum = 0;
    for (src, dest) in route.iter().zip(route.iter().skip(1)) {
        sum = sum + g.transitions.get(src).unwrap().get(dest).unwrap();
    }
    sum
}

fn part1(g: &Graph) -> u64 {
    let permutations = g
        .node_id_to_string
        .keys()
        .permutations(g.node_id_to_string.keys().count());

    let mut min_cost = u64::max_value();

    for route in permutations {
        let cost = cost_of_route(&route, g);
        min_cost = std::cmp::min(cost, min_cost);
    }

    min_cost
}

fn part2(g: &Graph) -> u64 {
    let permutations = g
        .node_id_to_string
        .keys()
        .permutations(g.node_id_to_string.keys().count());

    let mut max_cost = u64::min_value();

    for route in permutations {
        let cost = cost_of_route(&route, g);
        max_cost = std::cmp::max(cost, max_cost);
    }

    max_cost
}

fn main() {
    let args: Cli = Cli::parse();
    let input_str = read_to_string(args.input).unwrap();
    let graph = parse(&input_str);
    let part1_result = part1(&graph);
    println!("Solution for part 1: {}", part1_result);
    let part2_result = part2(&graph);
    println!("Solution for part 2: {}", part2_result);
}

#[cfg(test)]
mod tests {

    #[test]
    fn part1_test() {
        let g = super::parse(
            "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141",
        );
        assert_eq!(super::part1(&g), 605);
        assert_eq!(super::part2(&g), 982);
    }
}
