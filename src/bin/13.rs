use std::iter::zip;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let grids = parse_input(input);
    let reflections_sum = grids
        .iter()
        .fold(0, |acc, grid| acc + reflections(&grid, 0));

    Some(reflections_sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grids = parse_input(input);
    let reflections_sum = grids
        .iter()
        .fold(0, |acc, grid| acc + reflections(&grid, 1));

    Some(reflections_sum)
}

fn parse_input(input: &str) -> Vec<Vec<Vec<bool>>> {
    input
        .split("\n\n")
        .map(|grid| {
            grid.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

fn reflections(grid: &Vec<Vec<bool>>, max_smudges: usize) -> usize {
    if let Some(h_reflections) = horizontal_reflections(grid, max_smudges) {
        h_reflections * 100
    } else if let Some(v_reflections) = horizontal_reflections(&transpose(&grid), max_smudges) {
        v_reflections
    } else {
        0
    }
}

fn horizontal_reflections(grid: &Vec<Vec<bool>>, max_smudges: usize) -> Option<usize> {
    'row: for y in 0..grid.len() - 1 {
        let mut smudge_count = diff_row(&grid[y], &grid[y + 1]);
        if smudge_count <= max_smudges {
            let min_distance_to_edge = y.min(grid.len() - y - 2);
            for d in 1..=min_distance_to_edge {
                smudge_count += diff_row(&grid[y - d], &grid[y + d + 1]);
                if smudge_count > max_smudges {
                    continue 'row;
                }
            }
            if max_smudges > 0 && smudge_count == 0 {
                continue;
            }
            return Some(y + 1);
        }
    }

    None
}

fn diff_row(row1: &Vec<bool>, row2: &Vec<bool>) -> usize {
    zip(row1, row2).filter(|&(v1, v2)| v1 ^ v2).count()
}

// https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose(v: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
