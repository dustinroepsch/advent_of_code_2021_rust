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

    pub fn most_popular_bit(&self) -> char {
        assert_ne!(self.zero_count, self.one_count);
        if self.one_count > self.zero_count {
            return '1';
        }
        '0'
    }

    pub fn least_popular_bit(&self) -> char {
        assert_ne!(self.zero_count, self.one_count);
        if self.one_count <= self.zero_count {
            return '1';
        }
        '0'
    }
}

impl Default for BitCounter {
    fn default() -> Self {
        BitCounter::new()
    }
}

#[must_use]
pub fn part_a(problem_text: &str) -> String {
    let bitstrings: Vec<&str> = problem_text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    let bit_length = bitstrings.get(0).unwrap().len();
    let mut counts: Vec<BitCounter> = Vec::with_capacity(bit_length);
    for _ in 0..bit_length {
        counts.push(BitCounter::default())
    }

    for bitstring in bitstrings {
        for (idx, bit) in bitstring.chars().enumerate() {
            counts[idx].add_bit(bit);
        }
    }

    let gamma_rate: String = counts
        .iter()
        .map(|count| count.most_popular_bit())
        .collect();
    let gamma_rate = u32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate: String = counts
        .iter()
        .map(|count| count.least_popular_bit())
        .collect();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate, 2).unwrap();

    (gamma_rate * epsilon_rate).to_string()
}

#[must_use]
pub fn part_b(problem_text: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_3.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "3923414");
    }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(PROBLEM_TEXT), "todo");
    // }
}
