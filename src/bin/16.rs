advent_of_code::solution!(16);

type Beam = (usize, usize, Direction);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let coverage = beam_coverage(&grid, (0, 0, Direction::Right));
    Some(coverage)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let mut starting_positions: Vec<Beam> = (0..grid.len())
        .flat_map(|row| {
            [
                (row, 0, Direction::Right),
                (row, grid[0].len() - 1, Direction::Left),
            ]
        })
        .collect();
    starting_positions.extend((0..grid[0].len()).flat_map(|col| {
        [
            (0, col, Direction::Down),
            (grid.len() - 1, col, Direction::Up),
        ]
    }));

    Some(
        starting_positions
            .iter()
            .map(|&start| beam_coverage(&grid, start))
            .max()
            .unwrap(),
    )
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim_end()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn beam_coverage(grid: &Vec<Vec<char>>, start: Beam) -> usize {
    let mut seen: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
    let mut active_beams: Vec<Beam> = vec![start];

    while !active_beams.is_empty() {
        let mut new_beams: Vec<Beam> = vec![];
        for (row, col, direction) in active_beams {
            if row >= grid.len() || col >= grid[0].len() || seen[row][col].contains(&direction) {
                continue;
            }
            seen[row][col].push(direction);
            match (grid[row][col], direction) {
                ('-', Direction::Up | Direction::Down) => new_beams.extend([
                    step_beam(row, col, Direction::Left),
                    step_beam(row, col, Direction::Right),
                ]),
                ('|', Direction::Left | Direction::Right) => new_beams.extend([
                    step_beam(row, col, Direction::Up),
                    step_beam(row, col, Direction::Down),
                ]),
                ('/', Direction::Up) => new_beams.push(step_beam(row, col, Direction::Right)),
                ('/', Direction::Down) => new_beams.push(step_beam(row, col, Direction::Left)),
                ('/', Direction::Left) => new_beams.push(step_beam(row, col, Direction::Down)),
                ('/', Direction::Right) => new_beams.push(step_beam(row, col, Direction::Up)),
                ('\\', Direction::Up) => new_beams.push(step_beam(row, col, Direction::Left)),
                ('\\', Direction::Down) => new_beams.push(step_beam(row, col, Direction::Right)),
                ('\\', Direction::Left) => new_beams.push(step_beam(row, col, Direction::Up)),
                ('\\', Direction::Right) => new_beams.push(step_beam(row, col, Direction::Down)),
                _ => new_beams.push(step_beam(row, col, direction)),
            }
        }
        active_beams = new_beams;
    }

    seen.iter()
        .flat_map(|row| row)
        .filter(|point| !point.is_empty())
        .count()
}

fn step_beam(row: usize, col: usize, direction: Direction) -> (usize, usize, Direction) {
    let (delta_vert, delta_horz) = [(-1, 0), (0, 1), (1, 0), (0, -1)][direction as usize];
    (
        (row as isize + delta_vert) as usize,
        (col as isize + delta_horz) as usize,
        direction,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
