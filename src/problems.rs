use crate::util::VecExtension;

#[must_use]
pub fn problem_one() -> usize {
    let problem_text = include_str!("inputs/problem_1.txt");

    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    nums.count_increasing_pairs()
}

#[must_use]
pub fn problem_one_part_two() -> usize {
    let problem_text = include_str!("inputs/problem_1.txt");

    let nums: Vec<i32> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let totals: Vec<i32> = nums.windows(3).map(|x| x.iter().sum()).collect();

    totals.count_increasing_pairs()
}

pub mod two {
    use crate::problems::two::Direction::{Down, Forward, Up};

    enum Direction {
        Forward,
        Down,
        Up,
    }

    impl TryFrom<&str> for Direction {
        type Error = ();

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if value == "forward" {
                return Ok(Forward);
            }
            if value == "down" {
                return Ok(Down);
            }
            if value == "up" {
                return Ok(Up);
            }
            Err(())
        }
    }

    #[must_use] pub fn problem_two() -> i32 {
        let problem_text = include_str!("inputs/problem_2.txt");

        let plans: Vec<(Direction, i32)> = problem_text
            .lines()
            .map(|line| line.trim().split(' ').collect())
            .map(|arr: Vec<&str>| {
                let direction: Direction = arr[0].try_into().unwrap();
                let amount: i32 = arr[1].parse().unwrap();
                (direction, amount)
            })
            .collect();

        let mut horizontal = 0;
        let mut depth = 0;

        for (direction, amount) in plans {
            match direction {
                Down => depth += amount,
                Forward => horizontal += amount,
                Up => depth -= amount,
            };
        }
        horizontal * depth
    }
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
