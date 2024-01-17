use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks = parse_input(input);
    let adjacent = organise_bricks(&mut bricks);

    let safe_bricks = (0..bricks.len()).filter(|&brick| {
        let mut falling = HashSet::new();
        disintegrate_all(&adjacent, &mut falling, brick);
        falling.len() == 1
    });

    Some(safe_bricks.count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bricks = parse_input(input);
    let adjacent = organise_bricks(&mut bricks);

    let would_fall = (0..bricks.len()).map(|brick| {
        let mut falling = HashSet::new();
        disintegrate_all(&adjacent, &mut falling, brick);
        falling.len() - 1
    });
    Some(would_fall.sum())
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize, usize, usize, usize)> {
    let mut lines = input
        .trim_end()
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_ascii_digit())
                .map(|w| w.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect::<Vec<_>>();
    lines.sort_by_key(|&(_, _, z1, _, _, _)| z1);
    lines
}

fn organise_bricks(
    bricks: &mut Vec<(usize, usize, usize, usize, usize, usize)>,
) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut adjacent = vec![(HashSet::new(), HashSet::new()); bricks.len()];
    let mut space = HashMap::new();
    for brick in 0..bricks.len() {
        let (x1, y1, mut z1, x2, y2, mut z2) = bricks[brick];
        while z1 > 1
            && (x1..=x2)
                .cartesian_product(y1..=y2)
                .all(|(x, y)| !space.contains_key(&(x, y, z1 - 1)))
        {
            z2 -= 1;
            z1 -= 1;
        }
        for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
            for z in z1..=z2 {
                space.insert((x, y, z), brick);
            }
            if let Some(&spot) = space.get(&(x, y, z1 - 1)) {
                adjacent[spot].0.insert(brick);
                adjacent[brick].1.insert(spot);
            }
        }
        bricks[brick] = (x1, y1, z1, x2, y2, z2);
    }
    return adjacent;
}

fn disintegrate_all(
    adjacent: &[(HashSet<usize>, HashSet<usize>)],
    falling: &mut HashSet<usize>,
    i: usize,
) {
    falling.insert(i);
    for &above in &adjacent[i].0 {
        if adjacent[above].1.iter().all(|x| falling.contains(x)) {
            disintegrate_all(adjacent, falling, above);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
