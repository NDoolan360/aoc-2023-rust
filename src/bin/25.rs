use std::collections::HashMap;

use petgraph::{Graph, Undirected};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = parse_input(input);
    if let Some((cut, part)) = stoer_wagner_min_cut(&graph, |_| Ok::<usize, usize>(1)).unwrap() {
        assert!(cut == 3);
        let groups = vec![part.len(), (graph.node_count() - part.len())];
        Some(groups.iter().product())
    } else {
        None
    }
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn parse_input(input: &str) -> Graph<String, i32, Undirected> {
    let h: HashMap<String, Vec<String>> = input
        .trim_end()
        .lines()
        .map(|l| {
            let (parent, children) = l.split_once(": ").unwrap();
            (
                parent.to_string(),
                children
                    .split_ascii_whitespace()
                    .map(|c| c.to_string())
                    .collect(),
            )
        })
        .collect();

    let mut nodes = HashMap::new();
    h.iter().fold(
        Graph::<String, i32, Undirected>::new_undirected(),
        |mut graph, (k, children)| {
            let src = *nodes
                .entry(k)
                .or_insert_with(|| graph.add_node(k.to_string()));
            for child in children {
                let dst = *nodes
                    .entry(child)
                    .or_insert_with(|| graph.add_node(child.to_string()));
                graph.add_edge(src, dst, 1);
            }
            graph
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
