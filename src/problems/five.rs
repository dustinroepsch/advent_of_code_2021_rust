use crate::problems::ProblemSet;
use std::cmp::{max, min, Ordering};
use std::collections::HashSet;
use std::num::ParseIntError;

use std::str::FromStr;
use thiserror::Error;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Error, Debug)]
enum PointParseError {
    #[error("Not enough parts after splitting by \",\" (expected: 2, found: {0}).")]
    NotEnoughParts(usize),
    #[error("Too many parts after splitting by \",\" (expected: 2, found: {0}).")]
    TooManyParts(usize),
    #[error("Couldn't parse point parts into ints.")]
    InvalidIntError(#[from] ParseIntError),
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if let Some(err) = match parts.len().cmp(&2) {
            Ordering::Less => Some(PointParseError::NotEnoughParts(parts.len())),
            Ordering::Greater => Some(PointParseError::TooManyParts(parts.len())),
            Ordering::Equal => None,
        } {
            return Err(err);
        }

        let x: i32 = parts[0].trim().parse()?;
        let y: i32 = parts[1].trim().parse()?;

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

    fn points(&self) -> HashSet<Point> {
        if self.is_horiz() {
            let y = self.a.y;
            let start_x = min(self.a.x, self.b.x);
            let end_x = max(self.a.x, self.b.x);
            return (start_x..=end_x).map(|x| Point::new(x, y)).collect();
        }

        if self.is_vert() {
            let x = self.a.x;
            let start_y = min(self.a.y, self.b.y);
            let end_y = max(self.a.y, self.b.y);
            return (start_y..=end_y).map(|y| Point::new(x, y)).collect();
        }

        panic!("Only vertical and horiz are supported");
    }
}

#[derive(Error, Debug)]
enum LineSegmentParseError {
    #[error("Not enough parts after splitting by \"->\" (expected: 2, found: {0}).")]
    NotEnoughParts(usize),
    #[error("Too many parts after splitting by \"->\" (expected: 2, found: {0}).")]
    TooManyParts(usize),
    #[error("Couldn't parse points")]
    InvalidPoints(#[from] PointParseError),
}

impl FromStr for LineSegment {
    type Err = LineSegmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("->").collect();

        if let Some(err) = match parts.len().cmp(&2) {
            Ordering::Less => Some(LineSegmentParseError::NotEnoughParts(parts.len())),
            Ordering::Greater => Some(LineSegmentParseError::TooManyParts(parts.len())),
            Ordering::Equal => None,
        } {
            return Err(err);
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
        .filter_map(|line| line.parse().ok())
        .filter(|seg: &LineSegment| seg.is_horiz() || seg.is_vert())
        .collect();

    let mut all_points = HashSet::new();
    let mut seen_atleast_twice = HashSet::new();

    for segment in segments {
        for point in segment.points() {
            if all_points.contains(&point) {
                seen_atleast_twice.insert(point);
            }
            all_points.insert(point);
        }
    }
    seen_atleast_twice.len().to_string()
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
        assert_eq!(super::part_a(PROBLEM_TEXT), "6113");
    }
}
