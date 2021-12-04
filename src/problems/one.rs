use crate::problems::ProblemSet;
use crate::util::VecExtension;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    format!("{}", nums.count_increasing_pairs())
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let totals: Vec<i32> = nums.windows(3).map(|x| x.iter().sum()).collect();

    format!("{}", totals.count_increasing_pairs())
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_1.txt");
    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "1529");
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(PROBLEM_TEXT), "1567");
    }
}
