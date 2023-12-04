advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(first_last_digit_concat).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .replace("zero", "zero0zero")
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .lines()
            .map(first_last_digit_concat)
            .sum(),
    )
}

fn first_last_digit_concat(in_string: &str) -> u32 {
    let digits: Vec<u32> = in_string
        .chars()
        .filter_map(|char| char.to_digit(10))
        .collect();
    if let Some(first) = digits.first() {
        first * 10 + digits.last().unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            &advent_of_code::template::read_file("examples", DAY)
                .split("\n\n")
                .nth(0)
                .unwrap(),
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(
            &advent_of_code::template::read_file("examples", DAY)
                .split("\n\n")
                .nth(1)
                .unwrap(),
        );
        assert_eq!(result, Some(281));
    }
}
