advent_of_code::solution!(10);

use itertools::Itertools;

fn connection_open(tile: &char, direction: (isize, isize)) -> bool {
    let tile_identity = match tile {
        // up, down, left, right
        '|' => [true, true, false, false],
        '-' => [false, false, true, true],
        'L' => [true, false, false, true],
        'J' => [true, false, true, false],
        '7' => [false, true, true, false],
        'F' => [false, true, false, true],
        'S' => [true, true, true, true],
        _ => [false, false, false, false],
    };
    match direction {
        (-1, 0) => tile_identity[0],
        (1, 0) => tile_identity[1],
        (0, -1) => tile_identity[2],
        (0, 1) => tile_identity[3],
        (_, _) => unreachable!(),
    }
}

fn find_loop(map: &Vec<Vec<char>>, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut visited = vec![start];
    let (mut y, mut x) = start;

    loop {
        let old_size = visited.len();
        for &(dy, dx) in &[(1, 0), (-1, 0), (0, -1), (0, 1)] {
            // Lower bound
            let maybe_new_y = y as isize + dy;
            let maybe_new_x = x as isize + dx;
            if 0 > maybe_new_y || 0 > maybe_new_x {
                continue;
            }
            let new_y = maybe_new_y as usize;
            let new_x = maybe_new_x as usize;
            // Upper bound
            if new_x > map[0].len() || new_y >= map.len() {
                continue;
            }

            // Check Good connections
            if connection_open(&map[y][x], (dy, dx))
                && connection_open(&map[new_y][new_x], (-dy, -dx))
            {
                if visited.len() > 2 && new_y == start.0 && new_x == start.1 {
                    return Some(visited);
                } else if !visited.contains(&(new_y, new_x)) {
                    visited.push((new_y, new_x));
                    y = new_y;
                    x = new_x;
                    break;
                }
            }
        }
        // No progress
        if old_size == visited.len() {
            return None;
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let loop_coords = parse_input(input).1;
    Some(loop_coords.len() / 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, loop_coords) = parse_input(input);
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        let mut inside_loop = false;
        for (x, value) in row.iter().enumerate() {
            if y > 0 && loop_coords.contains(&(y, x)) && connection_open(value, (-1, 0)) {
                if value == &'S' && !connection_open(&map[y - 1][x], (1, 0)) {
                    continue;
                }
                inside_loop = !inside_loop;
            } else if !loop_coords.contains(&(y, x)) && inside_loop {
                sum += 1;
            }
        }
    }
    Some(sum)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut start: Option<(usize, usize)> = None;
    let map = input
        .lines()
        .enumerate()
        .filter_map(|(y, l)| {
            let row: Vec<char> = l.chars().collect();
            if let Some(x) = row.iter().position(|&c| c == 'S') {
                start = Some((y, x));
            }
            (!l.is_empty()).then(|| row)
        })
        .collect_vec();

    let loop_coords = find_loop(&map, start.unwrap()).unwrap();

    (map, loop_coords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            &advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .0,
        );
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            &advent_of_code::template::read_file("examples", DAY)
                .split_once("\n\n")
                .unwrap()
                .1,
        );
        assert_eq!(result, Some(10));
    }
}
