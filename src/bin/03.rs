use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let gears = solve(input).1;

    let sum_gear_ratios = gears
        .values()
        .filter(|s| s.len() == 2)
        .map(|s| s[0] * s[1])
        .sum();

    Some(sum_gear_ratios)
}
pub fn solve(input: &str) -> (u32, HashMap<(isize, isize), Vec<u32>>) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut sum_parts = 0;
    let mut gears = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut symbol_found = false;
        let mut gear_loc = None;

        for (x, &value) in row.iter().enumerate() {
            if value.is_ascii_digit() {
                number.push(value);

                // Check neighbours
                let neighbours = get_neighbours(&grid, x as isize, y as isize);
                for (neighbour, new_x, new_y) in neighbours {
                    if neighbour != '.' && !neighbour.is_ascii_digit() {
                        symbol_found = true;
                    }
                    if neighbour == '*' {
                        gear_loc = Some((new_x, new_y))
                    }
                }
            }
            if !value.is_ascii_digit() || x >= (row.len() - 1) {
                if !number.is_empty() && symbol_found {
                    let part_num = number.iter().collect::<String>().parse::<u32>().unwrap();
                    sum_parts += part_num;
                    if let Some(confirmed_gear) = gear_loc {
                        gears.entry(confirmed_gear).or_insert(vec![]).push(part_num);
                    }
                }
                number.clear();
                symbol_found = false;
                gear_loc = None;
            }
        }
    }

    (sum_parts, gears)
}

fn get_neighbours(grid: &[Vec<char>], x: isize, y: isize) -> Vec<(char, isize, isize)> {
    let mut neighbours = Vec::new();

    for i in -1..=1 {
        for j in -1..=1 {
            // Skip the center point
            if i == 0 && j == 0 {
                continue;
            }

            let new_x = x + i;
            let new_y = y + j;

            if new_x >= 0 && new_y >= 0 {
                // Check if the new indices are within bounds
                if let Some(row) = grid.get(new_y as usize) {
                    if let Some(&neighbour) = row.get(new_x as usize) {
                        neighbours.push((neighbour, new_x, new_y));
                    }
                }
            }
        }
    }
    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
