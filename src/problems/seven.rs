use crate::problems::ProblemSet;
use itertools::Itertools;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

fn find_cost(positions: &[i64], target_position: i64) -> usize {
    positions
        .iter()
        .map(|p| {
            let diff = (target_position - p).abs();
            let diff: usize = diff.try_into().expect("unexpected int error");
            diff
        })
        .sum()
}

fn natural_sum(upper_bound: usize) -> usize {
    (upper_bound * (upper_bound + 1)) / 2
}

fn find_exp_cost(positions: &[i64], target_position: i64) -> usize {
    positions
        .iter()
        .map(|p| (target_position - p).abs().try_into().unwrap())
        .map(natural_sum)
        .sum()
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let positions: Vec<i64> = problem_text
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect();
    let median = positions[positions.len() / 2];
    find_cost(&positions, median).to_string()
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
    let positions: Vec<i64> = problem_text
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect();
    let total: i64 = positions.iter().sum();
    let len: i64 = positions.len().try_into().unwrap();
    let average = total / len;
    find_exp_cost(&positions, average).to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_7.txt");
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "345035");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(PROBLEM_TEXT), "97038163");
    }
}
