use num::integer::lcm;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, false).0
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, true).1
}

fn parse_input(input: &str) -> HashMap<&str, (char, Vec<&str>)> {
    HashMap::from_iter(input.trim_end().lines().map(|l| {
        let (module, rest) = l.split_once(" -> ").unwrap();
        let dest_modules = rest.split(", ").collect();
        if module == "broadcaster" {
            (module, ('b', dest_modules))
        } else {
            (&module[1..], (module.as_bytes()[0] as char, dest_modules))
        }
    }))
}

pub fn solve(input: &str, find_rx: bool) -> (Option<u64>, Option<u64>) {
    let modules = parse_input(input);
    let (mut low, mut high) = (0, 0);

    let mut module_states = HashSet::new();
    let mut conjunctions = HashMap::<&str, HashMap<&str, bool>>::new();
    for (module, (_, connections)) in modules.clone() {
        for connection in connections {
            let Some(&(module_type, _)) = modules.get(connection) else {
                continue;
            };
            if module_type == '&' {
                conjunctions
                    .entry(connection)
                    .or_default()
                    .insert(module, false);
            }
        }
    }

    // As "rx" (the goal low-pulse module) is the output of &bq, this will only recieve a low pulse when all the inputs to bq are high.
    // The inputs to bq are &vg, &kp, &gc, &tx

    let mut bq_ins = [None; 4];

    let mut part_1 = None;
    for press in 1.. {
        let mut queue = VecDeque::from_iter([("broadcaster", "button", false)]);
        while let Some((module, prev, is_high)) = queue.pop_front() {
            if is_high {
                high += 1;
                if module == "bq" {
                    match prev {
                        "vg" => bq_ins[0] = bq_ins[0].or(Some(press as u64)),
                        "kp" => bq_ins[1] = bq_ins[1].or(Some(press as u64)),
                        "gc" => bq_ins[2] = bq_ins[2].or(Some(press as u64)),
                        "tx" => bq_ins[3] = bq_ins[3].or(Some(press as u64)),
                        _ => {}
                    }
                }
            } else {
                low += 1;
            }
            let Some((module_type, connections)) = modules.get(module) else {
                continue;
            };
            let pulse = match module_type {
                'b' => false,
                '%' => {
                    if is_high {
                        continue;
                    }
                    let on = module_states.contains(module);
                    if on {
                        module_states.remove(module);
                    } else {
                        module_states.insert(module);
                    }
                    !on
                }
                '&' => {
                    conjunctions.get_mut(module).unwrap().insert(prev, is_high);
                    !conjunctions[module].values().all(|&b| b)
                }
                _ => unreachable!(),
            };
            queue.extend(
                connections
                    .iter()
                    .map(|&connection| (connection, module, pulse)),
            );
        }
        if press == 1000 {
            part_1 = Some(low * high);
            if !find_rx {
                return (part_1, None);
            }
        }
        if bq_ins.iter().all(|o| o.is_some()) {
            let part_2 = bq_ins.iter().fold(1, |acc, press| lcm(acc, press.unwrap()));
            return (part_1, Some(part_2));
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32000000));
    }

    // Part 2 doesn't work as no example for part 2 was given.
}
