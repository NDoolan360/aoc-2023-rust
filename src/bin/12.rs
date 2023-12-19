advent_of_code::solution!(12);

use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let arrangmenet_sum = parse_input(input).iter().fold(0, |acc, (record, nums)| {
        cache.clear();
        acc + calc_arrangments(&mut cache, record, nums, None)
    });

    Some(arrangmenet_sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut cache = HashMap::new();
    let arrangmenet_sum = parse_input(input)
        .iter()
        .map(|(record, nums)| {
            let record_repeat = vec![record.iter().collect::<String>(); 5]
                .iter()
                .join("?")
                .chars()
                .collect_vec();
            let nums_repeat = vec![nums.iter().copied().collect_vec(); 5]
                .iter()
                .flatten()
                .copied()
                .collect_vec();
            (record_repeat, nums_repeat)
        })
        .fold(0, |acc, (record, nums)| {
            cache.clear();
            acc + calc_arrangments(&mut cache, &record, &nums, None)
        });

    Some(arrangmenet_sum)
}

fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (record, rest) = l.split_once(' ').unwrap();
            let nums = rest.split(',').map(|w| w.parse().unwrap()).collect();
            (record.chars().collect(), nums)
        })
        .collect()
}

fn calc_arrangments(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    record: &[char],
    nums: &[usize],
    confirmed_hash: Option<usize>,
) -> usize {
    if record.is_empty() {
        return match (confirmed_hash, nums.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == nums[0] => 1,
            _ => 0,
        };
    }
    if confirmed_hash.is_some() && nums.is_empty() {
        return 0;
    }

    let key = (record.len(), confirmed_hash.unwrap_or(0), nums.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }
    let arrangments = match (record[0], confirmed_hash) {
        ('.', Some(x)) if x != nums[0] => 0,
        ('.', Some(_)) => calc_arrangments(cache, &record[1..], &nums[1..], None),
        ('.', None) => calc_arrangments(cache, &record[1..], nums, None),
        ('#', Some(_)) => {
            calc_arrangments(cache, &record[1..], nums, confirmed_hash.map(|x| x + 1))
        }
        ('#', None) => calc_arrangments(cache, &record[1..], nums, Some(1)),
        ('?', Some(x)) => {
            let mut ans =
                calc_arrangments(cache, &record[1..], nums, confirmed_hash.map(|x| x + 1));
            if x == nums[0] {
                ans += calc_arrangments(cache, &record[1..], &nums[1..], None)
            }
            ans
        }
        ('?', None) => {
            calc_arrangments(cache, &record[1..], nums, Some(1))
                + calc_arrangments(cache, &record[1..], nums, None)
        }
        _ => unreachable!(),
    };
    cache.insert(key, arrangments);
    arrangments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
