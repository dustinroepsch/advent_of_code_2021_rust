use crate::problems::ProblemSet;
use std::cmp::Ordering;

pub const PROBLEM_SET: ProblemSet = ProblemSet { part_a, part_b };

#[derive(Copy, Clone)]
struct BitCounter {
    zero_count: u32,
    one_count: u32,
}

impl BitCounter {
    pub fn new() -> Self {
        Self {
            zero_count: 0,
            one_count: 0,
        }
    }
    pub fn add_bit(&mut self, bit: char) {
        if bit == '0' {
            self.zero_count += 1;
        }

        if bit == '1' {
            self.one_count += 1;
        }
    }

    pub fn most_popular_bit_or(self, default: char) -> char {
        match self.one_count.cmp(&self.zero_count) {
            Ordering::Less => '0',
            Ordering::Greater => '1',
            Ordering::Equal => default,
        }
    }

    pub fn least_popular_bit_or(self, default: char) -> char {
        match &self.one_count.cmp(&self.zero_count) {
            Ordering::Less => '1',
            Ordering::Greater => '0',
            Ordering::Equal => default,
        }
    }
}

impl Default for BitCounter {
    fn default() -> Self {
        BitCounter::new()
    }
}

struct BitStringDecoder {
    counts: Vec<BitCounter>,
}

impl BitStringDecoder {
    fn new(bit_length: usize) -> Self {
        let mut counts: Vec<BitCounter> = Vec::with_capacity(bit_length);
        for _ in 0..bit_length {
            counts.push(BitCounter::default());
        }
        Self { counts }
    }

    fn add(&mut self, bitstring: &str) {
        for (idx, bit) in bitstring.chars().enumerate() {
            self.counts[idx].add_bit(bit);
        }
    }

    fn add_all(&mut self, bitstrings: &[&str]) {
        for bs in bitstrings {
            self.add(bs);
        }
    }

    fn gamma_str(&self) -> String {
        self.counts
            .iter()
            .map(|count| count.most_popular_bit_or('0'))
            .collect()
    }

    fn gamma_rate(&self) -> u32 {
        u32::from_str_radix(&self.gamma_str(), 2).unwrap()
    }

    fn epsilon_str(&self) -> String {
        self.counts
            .iter()
            .map(|count| count.least_popular_bit_or('0'))
            .collect()
    }

    fn epsilon_rate(&self) -> u32 {
        u32::from_str_radix(&self.epsilon_str(), 2).unwrap()
    }

    fn oxygen_rating(samples: &[&str]) -> u32 {
        let bit_length = samples[0].len();
        let mut active_samples: Vec<&str> = samples.into();

        for idx in 0..bit_length {
            let mut counter = BitCounter::new();
            for bs in &active_samples {
                let bit = bs.chars().nth(idx).unwrap();
                counter.add_bit(bit);
            }
            let winner = counter.most_popular_bit_or('1');
            active_samples.retain(|bs| {
                let bit = bs.chars().nth(idx).unwrap();
                bit == winner
            });
            if active_samples.len() == 1 {
                break;
            }
        }
        u32::from_str_radix(active_samples[0], 2).unwrap()
    }

    fn co2_rating(samples: &[&str]) -> u32 {
        let bit_length = samples[0].len();
        let mut active_samples: Vec<&str> = samples.into();

        for idx in 0..bit_length {
            let mut counter = BitCounter::new();
            for bs in &active_samples {
                let bit = bs.chars().nth(idx).unwrap();
                counter.add_bit(bit);
            }
            let winner = counter.least_popular_bit_or('0');
            active_samples.retain(|bs| {
                let bit = bs.chars().nth(idx).unwrap();
                bit == winner
            });
            if active_samples.len() == 1 {
                break;
            }
        }
        u32::from_str_radix(active_samples[0], 2).unwrap()
    }
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let bitstrings: Vec<&str> = problem_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    let bit_length = bitstrings.get(0).unwrap().len();
    let mut decoder = BitStringDecoder::new(bit_length);
    decoder.add_all(&bitstrings);

    (decoder.gamma_rate() * decoder.epsilon_rate()).to_string()
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
    let bitstrings: Vec<&str> = problem_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let answer =
        BitStringDecoder::oxygen_rating(&bitstrings) * BitStringDecoder::co2_rating(&bitstrings);
    answer.to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_3.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "3923414");
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(PROBLEM_TEXT), "5852595");
    }
}
