use crate::problems::ProblemSet;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops::Add;

use std::str::FromStr;
use thiserror::Error;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(Error, Debug)]
enum PointParseError {
    #[error("Couldn't split the string on ,")]
    UnableToSplit,
    #[error("Couldn't parse point parts into ints.")]
    InvalidIntError(#[from] ParseIntError),
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(PointParseError::UnableToSplit)?;

        let x: i32 = x.trim().parse()?;
        let y: i32 = y.trim().parse()?;

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

        let mut points = HashSet::new();
        let mut current_point = min(self.a, self.b);
        let ending_point = max(self.a, self.b);
        let direction = if current_point.y < ending_point.y {
            Point::new(1, 1)
        } else {
            Point::new(1, -1)
        };

        points.insert(current_point);
        while current_point != ending_point {
            points.insert(current_point);
            current_point = current_point + direction;
        }
        points.insert(current_point);

        points
    }
}

#[derive(Error, Debug)]
enum LineSegmentParseError {
    #[error("Couldn't parse points")]
    InvalidPoints(#[from] PointParseError),
    #[error("Couldn't split the string on \"->\"")]
    UnableToSplit,
}

impl FromStr for LineSegment {
    type Err = LineSegmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once("->")
            .ok_or(LineSegmentParseError::UnableToSplit)?;

        let a: Point = a.parse()?;
        let b: Point = b.parse()?;

        Ok(LineSegment::new(a, b))
    }
}

fn count_duplicate_points(segments: Vec<LineSegment>) -> usize {
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

    seen_atleast_twice.len()
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let segments: Vec<LineSegment> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .filter(|seg: &LineSegment| seg.is_horiz() || seg.is_vert())
        .collect();

    count_duplicate_points(segments).to_string()
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
    let segments: Vec<LineSegment> = problem_text
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    count_duplicate_points(segments).to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_5.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "6113");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(PROBLEM_TEXT), "20373");
    }
}
