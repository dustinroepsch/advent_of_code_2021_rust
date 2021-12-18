use crate::problems::ProblemSet;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Debug)]
struct Lungfish {
    counter: u32,
    reset_value: u32,
}

enum Action {
    Nothing,
    Birth(Lungfish),
}

impl Lungfish {
    fn with_remaining_count(count: u32) -> Self {
        Self {
            counter: count,
            reset_value: 6,
        }
    }

    fn tick(&mut self) -> Action {
        if self.counter == 0 {
            self.counter = self.reset_value;
            return Action::Birth(Lungfish::with_remaining_count(self.reset_value + 2));
        }
        self.counter -= 1;
        Action::Nothing
    }
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let mut pool: Vec<Lungfish> = problem_text
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<u32>().ok())
        .map(Lungfish::with_remaining_count)
        .collect();

    for _ in 0..80 {
        let actions: Vec<Action> = pool.iter_mut().map(Lungfish::tick).collect();
        for action in actions {
            match action {
                Action::Birth(fish) => {
                    pool.push(fish);
                }
                Action::Nothing => {}
            }
        }
    }

    pool.len().to_string()
}

#[must_use]
pub fn part_b(_problem_text: &str) -> String {
    "hello_world".to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_6.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "390923");
    }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(PROBLEM_TEXT), "1567");
    // }
}
