advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input).0
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input).1
}

fn solve(input: &str) -> (Option<u32>, Option<u32>) {
    let cards = get_cards(input);

    let mut points = 0;
    let mut card_scores = vec![1; cards.len()];

    for (i, sets) in cards.iter().enumerate() {
        let mine = sets.first().unwrap();
        let winning = sets.last().unwrap();
        let matches = mine.iter().filter(|&v| winning.contains(v)).count();
        if matches > 0 {
            points += 2_u32.pow((matches - 1) as u32);

            for matches_left in 0..matches {
                card_scores[i + matches_left + 1] += card_scores[i];
            }
        }
    }

    println!("{:#?}", card_scores);

    (Some(points), Some(card_scores.iter().sum()))
}

fn get_cards(input: &str) -> Vec<Vec<Vec<u32>>> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(':')
                .filter(|l| !l.is_empty())
                .nth(1)
                .expect("There should be a list of tickets after the : in each line")
                .split('|')
                .map(|list| {
                    list.trim()
                        .split(' ')
                        .filter(|w| !w.is_empty())
                        .map(|v| v.parse::<u32>().unwrap())
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
