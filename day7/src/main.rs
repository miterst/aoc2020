use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction::{Incoming, Outgoing};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

// Add wrapper to cache the indices
struct DiGraphWrapper<Node: Eq + Hash, Edge> {
    graph: DiGraph<Node, Edge>,
    index_mapping: HashMap<Node, NodeIndex>,
}

impl<Node: Eq + Hash + Clone, Edge> DiGraphWrapper<Node, Edge> {
    fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_mapping: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: Node) -> NodeIndex {
        if let Some(existing_index) = self.index_mapping.get(&node) {
            *existing_index
        } else {
            let index = self.graph.add_node(node.clone());
            self.index_mapping.insert(node, index);
            index
        }
    }

    fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, weight: Edge) -> EdgeIndex {
        self.graph.add_edge(from, to, weight)
    }
}

fn main() {
    let bag_colors = include_str!("input").lines().fold(
        DiGraphWrapper::<String, i32>::new(),
        |mut bag_colors, line| {
            let parsed_line: Vec<&str> = line.split(" bags contain ").collect();

            let bag_color = parsed_line[0];
            let node = bag_colors.add_node(bag_color.to_string());

            let inner_bags: Vec<&str> = parsed_line[1].split(", ").collect();

            for inner_bag in inner_bags {
                if let (Some(bag_count), color) = parse_inner_bag(inner_bag) {
                    let inner_node = bag_colors.add_node(color);
                    bag_colors.add_edge(node, inner_node, bag_count);
                }
            }

            bag_colors
        },
    );

    println!("Part1 sol {}", part1(&bag_colors));
    println!("Part2 sol {}", part2(&bag_colors));
}

fn part1(digraph_wrapper: &DiGraphWrapper<String, i32>) -> usize {
    let staring_node = digraph_wrapper.index_mapping.get("shiny gold").unwrap();
    let mut stack: Vec<NodeIndex> = vec![*staring_node];
    let mut seen = HashSet::new();

    while let Some(node) = stack.pop() {
        for n in digraph_wrapper.graph.neighbors_directed(node, Incoming) {
            if !seen.contains(&n) {
                stack.push(n);
            }
        }
        seen.insert(node);
    }

    seen.len() - 1
}

fn part2(digraph_wrapper: &DiGraphWrapper<String, i32>) -> i32 {
    let staring_node = digraph_wrapper.index_mapping.get("shiny gold").unwrap();
    let mut stack = vec![(1, *staring_node)];
    let mut total = 0;

    while let Some((num_of_bags, node)) = stack.pop() {
        total += num_of_bags;
        for e in digraph_wrapper.graph.edges_directed(node, Outgoing) {
            stack.push((num_of_bags * e.weight(), e.target()))
        }
    }

    total - 1
}

fn parse_inner_bag(dep: &str) -> (Option<i32>, String) {
    let dep = dep
        .strip_suffix(" bag")
        .or_else(|| dep.strip_suffix(" bag."))
        .or_else(|| dep.strip_suffix(" bags."))
        .or_else(|| dep.strip_suffix(" bags"))
        .unwrap();

    let num_of_inner_bags = dep
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse()
        .ok();

    let color = dep
        .chars()
        .skip_while(|c| c.is_numeric() || c.is_whitespace())
        .collect();

    (num_of_inner_bags, color)
}
