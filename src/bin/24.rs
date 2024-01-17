use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<usize> {
    let hailstones = parse_input(input);
    Some(x_y_intersections(
        hailstones,
        200_000_000_000_000.0,
        400_000_000_000_000.0,
    ))
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn parse_input(input: &str) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            l.split(['@', ','])
                .map(|w| w.trim().parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn x_y_intersections(
    hailstones: Vec<(f64, f64, f64, f64, f64, f64)>,
    min_bound: f64,
    max_bound: f64,
) -> usize {
    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(&(x1, y1, _, dx1, dy1, _), &(x2, y2, _, dx2, dy2, _))| {
            let m1 = dy1 / dx1;
            let m2 = dy2 / dx2;
            if (m2 - m1).abs() <= f64::EPSILON {
                return false;
            }
            let x = (m1 * x1 - m2 * x2 + y2 - y1) / (m1 - m2);
            let y = (m1 * m2 * (x2 - x1) + m2 * y1 - m1 * y2) / (m2 - m1);
            if dx1.signum() != (x - x1).signum() || dx2.signum() != (x - x2).signum() {
                return false;
            }
            [x, y].iter().all(|v| (min_bound..=max_bound).contains(v))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let intersections = x_y_intersections(parse_input(input), 7.0, 27.0);

        assert_eq!(intersections, 2);
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(47));
    // }
}
