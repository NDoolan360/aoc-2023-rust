advent_of_code::solution!(19);

use std::collections::HashMap;

type Category = char;
type Operation = char;
type Rating = HashMap<Category, usize>;
type Conditions<'a> = Vec<(Option<(Category, Operation, usize)>, &'a str)>;

pub fn part_one(input: &str) -> Option<usize> {
    let (workflows, ratings) = parse_input(input);
    let sum = ratings
        .iter()
        .filter_map(|categories| {
            is_accepted(&workflows, categories).then_some(categories.values().sum::<usize>())
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (workflows, _) = parse_input(input);
    let ranges = HashMap::from_iter("xmas".chars().map(|c| (c, (1..=4000).collect())));
    Some(ranges_accepted(&workflows, "in", ranges))
}

fn parse_input(input: &str) -> (HashMap<&str, Conditions>, Vec<Rating>) {
    let (raw_workflows, raw_ratings) = input.trim_end().split_once("\n\n").unwrap();
    let workflows = HashMap::from_iter(raw_workflows.lines().map(|l| {
        let (label, rest) = l.split_once("{").unwrap();
        let conditions = rest
            .split(",")
            .map(|item| match item.split_once(':') {
                Some((rest, label)) => {
                    let (category, condition) = rest.split_at(1);
                    let (op, value) = condition.split_at(1);
                    (
                        Some((
                            category.chars().last().unwrap(),
                            op.chars().last().unwrap(),
                            value.parse().unwrap(),
                        )),
                        label,
                    )
                }
                None => (None, item.trim_end_matches(|c| c == '}')),
            })
            .collect();
        (label, conditions)
    }));
    let ratings = raw_ratings
        .lines()
        .map(|l| {
            HashMap::from_iter("xmas".chars().zip(l.splitn(4, ",").map(|item| {
                item.trim_matches(|c: char| !c.is_digit(10))
                    .parse()
                    .unwrap()
            })))
        })
        .collect();
    (workflows, ratings)
}

fn is_accepted(workflows: &HashMap<&str, Conditions>, rating: &Rating) -> bool {
    let mut curr = "in";
    loop {
        let workflow = &workflows[curr];
        curr = workflow
            .iter()
            .find_map(|&(condition, dest_workflow)| match condition {
                Some((category, op, value)) => match op {
                    '<' if rating[&category] < value => Some(dest_workflow),
                    '>' if rating[&category] > value => Some(dest_workflow),
                    _ => None,
                },
                None => Some(dest_workflow),
            })
            .unwrap();
        match curr {
            "A" => return true,
            "R" => return false,
            _ => continue,
        }
    }
}

fn ranges_accepted(
    workflows: &HashMap<&str, Conditions>,
    curr: &str,
    mut ranges: HashMap<Category, Vec<usize>>,
) -> usize {
    match curr {
        "R" => 0,
        "A" => ranges.iter().map(|(_, v)| v.len()).product(),
        _ => workflows[curr].iter().fold(0, |acc, (condition, label)| {
            let mut ranges_to_check = ranges.clone();
            if let Some((category, op, value)) = *condition {
                let (successes, fails) = ranges[&category].iter().partition(|&rating| match op {
                    '<' => rating < &value,
                    '>' => rating > &value,
                    _ => unreachable!(),
                });
                ranges_to_check.insert(category, successes);
                ranges.insert(category, fails);
            }
            acc + ranges_accepted(&workflows, label, ranges_to_check)
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
