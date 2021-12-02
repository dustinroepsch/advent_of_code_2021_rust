pub fn problem_one() {
    let problem_text = include_str!("inputs/problem_1.txt");
    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let total = nums
        .windows(2)
        .filter(|window| {
            let lhs = window[0];
            let rhs = window[1];
            lhs < rhs
        })
        .count();
    println!("Problem One: {}", total);
}

pub fn problem_one_part_two() {
    let problem_text = include_str!("inputs/problem_1.txt");

    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let sums: Vec<i32> = nums.windows(3).map(|x| x.iter().sum()).collect();

    let total = sums
        .windows(2)
        .filter(|window| {
            let lhs = window[0];
            let rhs = window[1];
            lhs < rhs
        })
        .count();

    println!("Problem Two: {}", total);
}
