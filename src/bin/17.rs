use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(17);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    None = 0,
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}

pub fn part_one(input: &str) -> Option<i32> {
    let blocks = parse_input(input);
    Some(path_find(blocks, 1, 3))
}

pub fn part_two(input: &str) -> Option<i32> {
    let blocks = parse_input(input);
    Some(path_find(blocks, 4, 10))
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

// dijkstras for path finding
fn path_find(grid: Vec<Vec<u32>>, minstep: isize, maxstep: isize) -> i32 {
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::from_iter([(0, (0, 0, Direction::None))]);
    while let Some((cost, (row, col, direction))) = queue.pop() {
        if (row, col) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }
        if dists
            .get(&(row, col, direction))
            .is_some_and(|&c| -cost > c)
        {
            continue;
        }
        for new_direction in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if direction == new_direction || direction_opposites(direction, new_direction) {
                continue;
            }
            let mut next_cost = -cost;
            for dist in 1..=maxstep {
                let (new_row, new_col, _) = step_pos(row, col, new_direction, dist);
                if new_row >= grid.len() || new_col >= grid[0].len() {
                    continue;
                }
                next_cost += (grid[new_row][new_col]) as i32;
                if dist < minstep {
                    continue;
                }
                let key = (new_row, new_col, new_direction);
                if next_cost < *dists.get(&key).unwrap_or(&i32::MAX) {
                    dists.insert(key, next_cost);
                    queue.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
}

fn direction_opposites(dir1: Direction, dir2: Direction) -> bool {
    match (dir1, dir2) {
        (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up)
        | (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left) => true,
        _ => false,
    }
}

fn step_pos(
    row: usize,
    col: usize,
    direction: Direction,
    dist: isize,
) -> (usize, usize, Direction) {
    let (delta_vert, delta_horz) = [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)][direction as usize];
    (
        (row as isize + delta_vert * dist) as usize,
        (col as isize + delta_horz * dist) as usize,
        direction,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
