advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::HashMap;
use num::integer::lcm;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, Node>) {
    let (steps, nodes) = input.split_once("\n\n").unwrap();
    let nodes_map = HashMap::from_iter(nodes.lines().map(|line| {
        let parts = line.split_whitespace().collect_vec();
        let matches: &[_] = &['(', ',', ')'];
        let left = parts[2].trim_matches(matches);
        let right = parts[3].trim_matches(matches);
        (parts[0], Node { left, right })
    }));

    (steps.chars().collect_vec(), nodes_map)
}

fn steps(
    instructs: &Vec<char>,
    nodes: &HashMap<&str, Node>,
    start: &str,
    end_ends_with: &str,
) -> u64 {
    let mut step_count = 0;
    let mut current_node = start;

    for &instruction in instructs.iter().cycle() {
        let node = nodes.get(current_node).unwrap();
        current_node = match instruction {
            'L' => node.left,
            'R' => node.right,
            _ => unreachable!(),
        };
        step_count += 1;
        if current_node.ends_with(end_ends_with) {
            return step_count;
        }
    }
    unreachable!();
}

pub fn part_one(input: &str) -> Option<u64> {
    let (instructs, nodes) = parse_input(input);
    let dist_to_end = steps(&instructs, &nodes, "AAA", "ZZZ");
    Some(dist_to_end)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructs, nodes) = parse_input(input);
    let start_nodes = nodes.keys().filter(|&name| name.ends_with('A'));
    let dists_to_end = start_nodes.map(|start| steps(&instructs, &nodes, start, "Z"));
    let lcm = dists_to_end.fold(1, |acc, dist| lcm(acc, dist));
    Some(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            &advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(6));
    }
}
