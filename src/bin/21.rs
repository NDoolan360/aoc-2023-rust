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
    solve_steps(input, 64)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid = input
        .trim_end()
        .lines()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
    let start = (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .find(|&(r, c)| grid[r][c] == 'S')
        .map(|(r, c)| (r, c))
        .unwrap();
    (grid, start)
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> usize {
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
                if grid[new_row][new_col] != '#' {
                    next_positions.insert((new_row, new_col));
                }
            }
        }
        (positions, next_positions) = (next_positions, positions);
    }
    positions.len()
}

pub fn solve_steps(input: &str, steps: usize) -> Option<usize> {
    let (grid, start) = parse_input(input);
    let end_positions = bfs(&grid, start, steps);
    Some(end_positions)
}

fn step(row: usize, col: usize, direction: Direction) -> (usize, usize) {
    let (delta_vert, delta_horz) = [(-1, 0), (0, 1), (1, 0), (0, -1)][direction as usize];
    (
        (row as isize + delta_vert) as usize,
        (col as isize + delta_horz) as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_steps(&advent_of_code::template::read_file("examples", DAY), 6);
        assert_eq!(result, Some(16));
    }
}
