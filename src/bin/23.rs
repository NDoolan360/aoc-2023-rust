use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input)
}

pub fn part_two(input: &str) -> Option<usize> {
    let desloped_input = input.replace(|c| ['<', '>', 'v', '^'].contains(&c), ".");
    solve(desloped_input.as_str())
}

fn parse_input(
    input: &str,
) -> (
    HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    Vec<Vec<char>>,
) {
    let grid = input
        .trim_end()
        .lines()
        .map(|l: &str| l.chars().collect_vec())
        .collect_vec();

    let mut graph = HashMap::<_, Vec<_>>::new();
    for (row, col) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let neighbors = match grid[row][col] {
            '#' => continue,
            '.' => vec![(-1, 0), (0, 1), (1, 0), (0, -1)],
            '>' => vec![(0, 1)],
            '<' => vec![(0, -1)],
            'v' => vec![(1, 0)],
            '^' => vec![(-1, 0)],
            _ => unreachable!(),
        };
        let entry = graph.entry((row, col)).or_insert(vec![]);
        for (delta_row, delta_col) in neighbors {
            let (new_row, new_col) = (
                (row as isize + delta_row) as usize,
                (col as isize + delta_col) as usize,
            );
            if grid
                .get(new_row)
                .and_then(|row| row.get(new_col))
                .is_some_and(|&t| t != '#')
            {
                entry.push((new_row, new_col, 1));
            }
        }
    }

    (graph, grid)
}

fn solve(input: &str) -> Option<usize> {
    let (mut graph, grid) = parse_input(input);

    contract_corridors(&mut graph);

    let indexes = graph
        .keys()
        .enumerate()
        .map(|(i, &pos)| (pos, i))
        .collect::<HashMap<_, _>>();

    let start = indexes[&(0, 1)];
    let goal = indexes[&(grid.len() - 1, grid[0].len() - 2)];
    let mut seen = vec![false; graph.len()];

    dfs(&index_graph(graph, indexes), &mut seen, goal, start)
}

fn contract_corridors(graph: &mut HashMap<(usize, usize), Vec<(usize, usize, usize)>>) {
    let corridors = graph
        .iter()
        .filter(|(_, n)| n.len() == 2)
        .map(|(&node, _)| node)
        .collect::<Vec<_>>();
    for (row, col) in corridors {
        let neighbors = graph.remove(&(row, col)).unwrap();
        let (row_1, col_1, dist_1) = neighbors[0];
        let (row_2, col_2, dist_2) = neighbors[1];
        let point_1 = graph.get_mut(&(row_1, col_1)).unwrap();
        if let Some(i) = point_1
            .iter()
            .position(|&(row_p1, col_p1, _)| (row_p1, col_p1) == (row, col))
        {
            point_1[i] = (row_2, col_2, dist_1 + dist_2);
        }
        let point_2 = graph.get_mut(&(row_2, col_2)).unwrap();
        if let Some(i) = point_2
            .iter()
            .position(|&(row_p2, col_p2, _)| (row_p2, col_p2) == (row, col))
        {
            point_2[i] = (row_1, col_1, dist_1 + dist_2);
        }
    }
}

fn index_graph(
    graph: HashMap<(usize, usize), Vec<(usize, usize, usize)>>,
    indexes: HashMap<(usize, usize), usize>,
) -> Vec<Vec<(usize, usize)>> {
    let mut index_graph = vec![Vec::new(); graph.len()];
    for (pos, neighbors) in &graph {
        index_graph[indexes[pos]] = neighbors
            .iter()
            .map(|&(r, c, d)| (indexes[&(r, c)], d))
            .collect();
    }

    index_graph
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    seen: &mut Vec<bool>,
    goal: usize,
    curr: usize,
) -> Option<usize> {
    if curr == goal {
        return Some(0);
    }
    let mut max_dist = None;
    for &(next, d) in &graph[curr] {
        if !seen[next] {
            seen[next] = true;
            if let Some(dist) = dfs(graph, seen, goal, next) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[next] = false;
        }
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
