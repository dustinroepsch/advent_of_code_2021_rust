use crate::problems::two::Direction::{Down, Forward, Up};
use crate::problems::ProblemSet;

pub const PROBLEM_SET: ProblemSet = ProblemSet {
    part_a: self::part_a,
    part_b: self::part_b,
};

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

#[must_use]
pub fn part_a(problem_text: &str) -> String {
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
    let mut aim = 0;

    for (direction, amount) in plans {
        match direction {
            Down => {
                aim += amount;
            }
            Forward => {
                horizontal += amount;
                depth += aim * amount;
            }
            Up => {
                aim -= amount;
            }
        };
    }
    format!("{}", (horizontal * depth))
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
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
    format!("{}", (horizontal * depth))
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_2.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "2089174012");
    }
    #[test]
    fn part_b() {
        assert_eq!(super::part_b(PROBLEM_TEXT), "1989265");
    }
}
