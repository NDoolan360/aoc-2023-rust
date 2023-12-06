use std::ops::Range;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let (seeds, maps) = parse_input(input);
    let out = seeds.into_iter().map(|seed| {
        maps.iter().fold(seed, |seed, map| {
            map.iter()
                .find_map(|(source, dest)| {
                    let offset = source.start - dest.start;
                    dest.contains(&seed).then_some(seed + offset)
                })
                .unwrap_or(seed)
        })
    });
    Some(out.min().unwrap())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (seed_pairs, maps) = parse_input(input);
    let mut seeds_ranges = seeds_to_ranges(seed_pairs);

    for map in maps.iter() {
        let mut new_ranges: Vec<Range<i64>> = vec![];

        for range in seeds_ranges.iter() {
            let mut curr = range.clone();
            for (source, dest) in map.iter() {
                let offset = source.start - dest.start;
                if curr.start <= curr.end && curr.start <= dest.end && dest.start <= curr.end {
                    if curr.start < dest.start {
                        new_ranges.push(curr.start..dest.start);
                        curr.start = dest.start;
                    }
                    if curr.end <= dest.end {
                        new_ranges.push(curr.start + offset..curr.end + offset);
                        curr.start = curr.end + 1;
                    } else {
                        new_ranges.push(curr.start + offset..dest.end + offset);
                        curr.start = dest.end;
                    }
                }
            }
            if curr.start <= curr.end {
                new_ranges.push(curr);
            }
        }
        seeds_ranges = new_ranges;
    }

    seeds_ranges.iter().map(|r| r.start).min()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<(Range<i64>, Range<i64>)>>) {
    let (seed_raw, maps_raw) = input.split_once("\n\n").unwrap();
    let seeds = seed_raw
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let maps = maps_raw
        .split("\n\n")
        .map(|map| {
            map.lines()
                .skip(1)
                .map(|l| {
                    let nums = l
                        .split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<_>>();
                    (nums[0]..nums[0] + nums[2], nums[1]..nums[1] + nums[2])
                })
                .sorted_by(|(_, a_dest), (_, b_dest)| a_dest.start.cmp(&b_dest.start))
                .collect()
        })
        .collect();

    (seeds, maps)
}

fn seeds_to_ranges(seed_pairs: Vec<i64>) -> Vec<Range<i64>> {
    seed_pairs.chunks(2).map(|s| s[0]..s[0] + s[1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
