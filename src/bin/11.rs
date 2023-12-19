use distances::vectors::manhattan;
use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let universe = parse_input(input);
    let galaxies = get_galaxies(&universe);
    let expanded_galaxies = expand_universe(&universe, &galaxies, 2);
    let sum_dist = expanded_galaxies
        .iter()
        .combinations(2)
        .fold(0, |acc, galaxy_pair| {
            acc + manhattan(galaxy_pair[0], galaxy_pair[1])
        });
    Some(sum_dist)
}

pub fn part_two(input: &str) -> Option<usize> {
    let universe = parse_input(input);
    let galaxies = get_galaxies(&universe);
    let expanded_galaxies = expand_universe(&universe, &galaxies, 1000000);
    let sum_dist = expanded_galaxies
        .iter()
        .combinations(2)
        .fold(0, |acc, galaxy_pair| {
            acc + manhattan(galaxy_pair[0], galaxy_pair[1])
        });
    Some(sum_dist)
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c == '#').collect_vec())
        .collect_vec()
}

fn get_galaxies(universe: &Vec<Vec<bool>>) -> Vec<Vec<usize>> {
    universe.iter().enumerate().fold(vec![], |mut acc, (y, l)| {
        let mut line_galaxies = l
            .iter()
            .enumerate()
            .filter_map(|(x, v)| match v {
                true => Some(vec![y, x]),
                false => None,
            })
            .collect_vec();
        acc.append(&mut line_galaxies);
        acc
    })
}

fn expand_universe(
    universe: &Vec<Vec<bool>>,
    galaxies: &Vec<Vec<usize>>,
    expansion_value: usize,
) -> Vec<Vec<usize>> {
    let mut new_galaxies = galaxies.clone();
    (0..universe.len())
        .filter(|&r| universe[r].iter().all(|&c| !c))
        .rev()
        .for_each(|r| {
            new_galaxies.iter_mut().for_each(|g| {
                if g[0] > r {
                    g[0] += expansion_value - 1
                }
            })
        });
    (0..universe[0].len())
        .filter(|&c| transpose(universe)[c].iter().all(|&r| !r))
        .rev()
        .for_each(|c| {
            new_galaxies.iter_mut().for_each(|g| {
                if g[1] > c {
                    g[1] += expansion_value - 1
                }
            })
        });
    new_galaxies
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose(v: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
