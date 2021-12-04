pub mod one;
pub mod three;
pub mod two;

pub struct ProblemSet {
    pub part_a: fn(&str) -> String,
    pub part_b: fn(&str) -> String,
}
