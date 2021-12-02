use crate::util::VecExtension;

#[must_use] pub fn problem_one() -> usize {
    let problem_text = include_str!("inputs/problem_1.txt");

    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    

    nums.count_increasing_pairs()
}

#[must_use] pub fn problem_one_part_two() -> usize {
    let problem_text = include_str!("inputs/problem_1.txt");

    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let totals: Vec<i32> = nums.windows(3).map(|x| x.iter().sum()).collect();
    

    totals.count_increasing_pairs() 
}

#[cfg(test)]
mod tests {
    #[test]
    fn problem_one() {
        assert_eq!(super::problem_one(), 1529);
    }
    #[test]
    fn problem_one_part_two() {
        assert_eq!(super::problem_one_part_two(), 1567);
    }
}
