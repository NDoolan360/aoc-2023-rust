advent_of_code::solution!(18);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    _None = 0,
    Up = 1,
    Right = 2,
    Down = 3,
    Left = 4,
}

pub fn part_one(input: &str) -> Option<isize> {
    let steps = parse_input(input)
        .iter()
        .map(|&(dir, dist, _)| (dir, dist))
        .collect();
    Some(calc_area(&steps))
}

pub fn part_two(input: &str) -> Option<isize> {
    let steps = parse_input(input)
        .iter()
        .map(|(_, _, col)| {
            let (head, tail) = col.split_at(5);
            let direction = match tail.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => unreachable!(),
            };
            let distance = usize::from_str_radix(head, 16).unwrap();
            (direction, distance)
        })
        .collect();
    Some(calc_area(&steps))
}

fn parse_input(input: &str) -> Vec<(Direction, usize, String)> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let (raw_dir, rest) = l.split_once(' ').unwrap();
            let (raw_dist, raw_colour) = rest.split_once(' ').unwrap();
            let dir = match raw_dir {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let dist = raw_dist.parse().unwrap();
            let colour = raw_colour
                .chars()
                .filter(|&c| c != '(' && c != '#' && c != ')')
                .collect::<String>();
            (dir, dist, colour)
        })
        .collect()
}

fn calc_area(steps: &Vec<(Direction, usize)>) -> isize {
    let mut area = 0;
    let mut col: isize = 0;
    let mut row: isize = 0;
    // Shoelace formula
    for &(direction, distance) in steps {
        let (old_row, old_col) = (row, col);
        (row, col, _) = step_pos(row, col, direction, distance as isize);
        area += (col + old_col) * (row - old_row) + distance as isize;
    }
    area / 2 + 1
}

fn step_pos(
    row: isize,
    col: isize,
    direction: Direction,
    dist: isize,
) -> (isize, isize, Direction) {
    let (delta_vert, delta_horz) = [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)][direction as usize];
    (
        row as isize + delta_vert * dist,
        col as isize + delta_horz * dist,
        direction,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
