advent_of_code::solution!(9);

use itertools::Itertools;

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect()
}

fn next_step(values: &Vec<i32>) -> Vec<i32> {
    values
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| b - a)
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let values = input.lines().map(|l| {
        let mut v = vec![parse_line(l)];
        while v.last().unwrap().iter().any(|&x| x != 0) {
            v.push(next_step(v.last().unwrap()));
        }
        v.iter().rev().fold(0, |a, xs| xs[xs.len() - 1] + a)
    });
    Some(values.sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let values = input.lines().map(|l| {
        let mut v = vec![parse_line(l)];
        while v.last().unwrap().iter().any(|&x| x != 0) {
            v.push(next_step(v.last().unwrap()));
        }
        v.iter().rev().fold(0, |b, xs| xs[0] - b)
    });
    Some(values.sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
