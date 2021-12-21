use crate::problems::ProblemSet;
use itertools::Itertools;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

fn find_cost(positions: &[i32], target_position: i32) -> usize {
    positions
        .iter()
        .map(|p| (target_position - p).abs() as usize)
        .sum()
}

#[must_use] pub fn part_a(problem_text: &str) -> String {
    let positions: Vec<i32> = problem_text
        .trim()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .sorted()
        .collect();
    let median = positions[positions.len() / 2];
    find_cost(&positions, median).to_string()
}

#[must_use] pub fn part_b(_problem_text: &str) -> String {
    "hello world".to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_7.txt");
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "345035");
    }
}
