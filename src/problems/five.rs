use crate::problems::ProblemSet;

use std::str::FromStr;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err(());
        }

        let x: i32 = parts[0].trim().parse().map_err(|_| ())?;
        let y: i32 = parts[1].trim().parse().map_err(|_| ())?;

        Ok(Point::new(x, y))
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct LineSegment {
    a: Point,
    b: Point,
}

impl LineSegment {
    fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    fn is_horiz(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_vert(&self) -> bool {
        self.a.x == self.b.x
    }
}

impl FromStr for LineSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("->").collect();
        if parts.len() != 2 {
            return Err(());
        }
        let a: Point = parts[0].parse()?;
        let b: Point = parts[1].parse()?;

        Ok(LineSegment::new(a, b))
    }
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let segments: Vec<LineSegment> = problem_text
        .lines()
        .filter_map(|line| {
            let seg: LineSegment = line.parse().ok()?;
            Some(seg)
        })
        .filter(|seg| seg.is_horiz() || seg.is_vert())
        .collect();

    format!("{:?}", segments)
}

#[must_use]
pub fn part_b(_problem_text: &str) -> String {
    "hello_world".to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_5.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "1529");
    }
}
