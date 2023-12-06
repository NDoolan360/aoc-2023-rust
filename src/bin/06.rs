use std::str::SplitAsciiWhitespace;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<i64> {
    let raw_lines = parse_lines(input);
    let mut lines = raw_lines
        .iter()
        .map(|l| l.clone().map(|x| x.parse::<i64>().unwrap()).collect());
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    let num_options: i64 = race_win_options(time, distance);

    Some(num_options)
}

pub fn part_two(input: &str) -> Option<i64> {
    let raw_lines = parse_lines(input);
    let mut lines = raw_lines
        .iter()
        .map(|l| l.clone().collect::<String>().parse::<i64>().unwrap());
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    let num_options: i64 = race_win_options(vec![time], vec![distance]);

    Some(num_options)
}

fn parse_lines(input: &str) -> Vec<SplitAsciiWhitespace> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(":").unwrap().1.trim().split_ascii_whitespace())
        .collect()
}

fn race_win_options(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(t_max, max_dist)| {
            (1..t_max).fold(0, |acc, t_charge| {
                let dist = t_charge * (t_max - t_charge);
                if dist > max_dist {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
