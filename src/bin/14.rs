use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse_input(input);
    roll_up(&mut grid);

    Some(score_grid(&grid))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::<Vec<Vec<char>>, usize>::new();
    let mut grid = parse_input(input);

    for step in 1..=1_000_000_000 {
        for _ in 0..4 {
            roll_up(&mut grid);
            rotate_left(&mut grid);
        }
        if let Some(seen) = cache.insert(grid.clone(), step) {
            if (1000000000 - step) % (step - seen) == 0 {
                break;
            }
        }
    }

    Some(score_grid(&grid))
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect()
}

fn score_grid(grid: &Vec<Vec<char>>) -> usize {
    (0..grid.len())
        .map(|row| {
            (0..grid[0].len())
                .filter(|&col| grid[row][col] == 'O')
                .map(|_| grid.len() - row)
                .sum::<usize>()
        })
        .sum()
}

fn roll_up(grid: &mut Vec<Vec<char>>) {
    'looping: loop {
        for row in 0..grid.len() - 1 {
            for col in 0..grid[0].len() {
                if grid[row + 1][col] == 'O' && grid[row][col] == '.' {
                    grid[row][col] = 'O';
                    grid[row + 1][col] = '.';
                    continue 'looping;
                }
            }
        }
        break;
    }
}

fn rotate_left<T>(grid: &mut Vec<Vec<T>>)
where
    T: Clone,
    T: Copy,
{
    let grid_clone = grid.clone();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let new_coords = (col, grid.len() - 1 - row);
            grid[new_coords.0][new_coords.1] = grid_clone[row][col];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
