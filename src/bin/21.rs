advent_of_code::solution!(21);

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, start) = parse_input(input);
    bfs(&grid, start, 64)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, start) = parse_input(input);
    find_polynomial(&grid, start, 26501365)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (isize, isize)) {
    let grid = input
        .trim_end()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == 'S')
        .map(|(r, c)| (r as isize, c as isize))
        .unwrap();
    (grid, start)
}

fn bfs(grid: &Vec<Vec<char>>, start: (isize, isize), steps: usize) -> Option<usize> {
    let mut positions = HashSet::from_iter([start]);
    let mut next_positions = HashSet::new();
    for _ in 0..steps {
        next_positions.clear();
        for &(r, c) in &positions {
            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let (new_row, new_col) = step(r, c, direction);
                if grid[new_row as usize % grid.len()][new_col as usize % grid[0].len()] != '#' {
                    next_positions.insert((new_row, new_col));
                }
            }
        }
        (positions, next_positions) = (next_positions, positions);
    }
    Some(positions.len())
}

fn step(row: isize, col: isize, direction: Direction) -> (isize, isize) {
    let (delta_vert, delta_horz) = [(-1, 0), (0, 1), (1, 0), (0, -1)][direction as usize];
    ((row + delta_vert), (col + delta_horz))
}

// See https://en.wikipedia.org/wiki/Newton_polynomial
fn find_polynomial(grid: &Vec<Vec<char>>, start: (isize, isize), steps: usize) -> Option<usize> {
    let point_1 = bfs(grid, start, steps % grid.len()).unwrap();
    let point_2 = bfs(grid, start, steps % grid.len() + grid.len()).unwrap();
    let point_3 = bfs(grid, start, steps % grid.len() + grid.len() * 2).unwrap();
    let max_traversals = (steps / grid.len()) as isize;
    let [a, b, c] = [
        point_1 as isize,
        (point_2 - point_1) as isize,
        (point_3 - point_2) as isize,
    ];
    Some((a + b * max_traversals + (max_traversals * (max_traversals - 1) / 2) * (c - b)) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (grid, start) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(bfs(&grid, start, 1), Some(2));
        assert_eq!(bfs(&grid, start, 2), Some(4));
        assert_eq!(bfs(&grid, start, 3), Some(6));
        assert_eq!(bfs(&grid, start, 4), Some(9));
        assert_eq!(bfs(&grid, start, 5), Some(13));
        assert_eq!(bfs(&grid, start, 6), Some(16));
    }

    // #[test]
    // fn test_part_two() {
    //     let (grid, start) = parse_input(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(find_polynomial(&grid, start, 6), Some(16));
    //     assert_eq!(find_polynomial(&grid, start, 7), Some(22));
    //     assert_eq!(find_polynomial(&grid, start, 8), Some(30));
    //     assert_eq!(find_polynomial(&grid, start, 9), Some(41));
    //     assert_eq!(find_polynomial(&grid, start, 10), Some(50));
    //     assert_eq!(find_polynomial(&grid, start, 50), Some(1594));
    // }
}
