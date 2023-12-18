use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input(input)
            .iter()
            .fold(0, |acc, &step| acc + hash(step)),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let steps = parse_input(input);
    let mut boxes = vec![Vec::<(&str, usize)>::new(); 256];

    steps.iter().for_each(|step| {
        let op_char = step.chars().find(|&c| c == '-' || c == '=').unwrap();
        let (lens_label, focal_length) = step.split_once(op_char).unwrap();
        let lens_box = &mut boxes[hash(lens_label)];
        let lens_pos = lens_box.iter().position(|&(l, _)| l == lens_label);
        match (op_char, lens_pos) {
            ('=', Some(i)) => lens_box[i] = (lens_label, focal_length.parse().unwrap()),
            ('=', None) => lens_box.push((lens_label, focal_length.parse().unwrap())),
            ('-', Some(i)) => {
                lens_box.remove(i);
            }
            _ => {}
        };
    });

    let sum = (0..boxes.len())
        .flat_map(|lens_box_index| {
            (0..boxes[lens_box_index].len()).map(move |lens_index| (lens_box_index, lens_index))
        })
        .map(|(lens_box_index, lens_index)| {
            (lens_box_index + 1) * (lens_index + 1) * boxes[lens_box_index][lens_index].1
        })
        .sum();

    Some(sum)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim_end().split(',').collect_vec()
}

fn hash(s: &str) -> usize {
    s.bytes().fold(0, |a, c| ((a + c as usize) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
