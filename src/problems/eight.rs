use crate::problems::ProblemSet;
use std::error::Error;
use std::str::FromStr;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Debug)]
struct SignalPattern(String);

impl SignalPattern {
    pub fn is_trivial(&self) -> bool {
        //1
        self.0.len() == 2
        //4
        || self.0.len() == 4
        //7
        || self.0.len() == 3
        //8
        || self.0.len() == 7
    }
}

#[derive(Debug)]
struct Entry {
    _signal_patterns: Vec<SignalPattern>,
    output_value: Vec<SignalPattern>,
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (signal_patterns, output_value) = s.split_once('|').ok_or("Couldn't split on |")?;
        let signal_patterns: Vec<SignalPattern> = signal_patterns
            .split_whitespace()
            .map(|s| SignalPattern(s.to_string()))
            .collect();
        let output_value: Vec<SignalPattern> = output_value
            .split_whitespace()
            .map(|s| SignalPattern(s.to_string()))
            .collect();
        Ok(Self {
            _signal_patterns: signal_patterns,
            output_value,
        })
    }
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let entries: Vec<Entry> = problem_text
        .lines()
        .filter_map(|line| line.parse::<Entry>().ok())
        .collect();

    let mut total = 0;

    for entry in entries {
        for signal in entry.output_value {
            if signal.is_trivial() {
                total += 1;
            }
        }
    }

    total.to_string()
}

#[must_use]
pub fn part_b(_problem_text: &str) -> String {
    "hello, world".to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_8.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "452");
    }
}
