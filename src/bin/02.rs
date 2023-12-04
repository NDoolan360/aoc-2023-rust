advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = get_games(input);
    Some(
        games
            .iter()
            .enumerate()
            .filter_map(|(index, game)| {
                let bad_reveal = game.iter().flatten().any(|&(size, name)| match name {
                    "red" => size > 12,
                    "green" => size > 13,
                    "blue" => size > 14,
                    _ => panic!("Unexpected: {:#?}", name),
                });
                if bad_reveal {
                    None
                } else {
                    Some((index as u32) + 1)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = get_games(input);
    Some(
        games
            .iter()
            .map(|game| {
                let maxs = game
                    .iter()
                    .flatten()
                    .fold((0, 0, 0), |mut acc, &(size, name)| {
                        match name {
                            "red" => acc.0 = acc.0.max(size),
                            "green" => acc.1 = acc.1.max(size),
                            "blue" => acc.2 = acc.2.max(size),
                            _ => panic!("Unexpected: {:#?}", name),
                        };
                        acc
                    });
                (maxs.0 * maxs.1 * maxs.2) as u32
            })
            .sum(),
    )
}

fn get_games(input: &str) -> Vec<Vec<Vec<(usize, &str)>>> {
    input
        .lines()
        .map(|line| {
            let (_, info) = line.split_once(": ").unwrap();
            let reveals = info.split("; ");
            reveals
                .map(|reveal| {
                    reveal
                        .split(", ")
                        .map(|item| {
                            let (amt, colour) = item.split_once(' ').unwrap();
                            (amt.parse::<usize>().unwrap(), colour)
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
