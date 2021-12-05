use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Cell {
    number: i32,
    has_been_called: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new(0)
    }
}

impl Cell {
    fn new(number: i32) -> Self {
        Self {
            number,
            has_been_called: false,
        }
    }
}

#[derive(Debug)]
pub struct BingoBoard {
    layout: [[Cell; 5]; 5],
}

impl BingoBoard {
    fn with_layout(lines: Vec<&str>) -> Self {
        let mut layout: [[Cell; 5]; 5] = Default::default();
        for (row_idx, line) in lines.iter().enumerate() {
            for (col_idx, num) in line.split_whitespace().enumerate() {
                layout[row_idx][col_idx] = Cell::new(num.parse().unwrap());
            }
        }
        Self { layout }
    }

    fn call(&mut self, n: i32) {
        for row in &mut self.layout {
            for cell in row.iter_mut() {
                if cell.number == n {
                    cell.has_been_called = true
                }
            }
        }
    }
    fn is_winner(&self) -> bool {
        //rows
        for row in 0..5 {
            if self.layout[row].iter().all(|cell| cell.has_been_called) {
                return true;
            }
        }
        //cols
        for col in 0..5 {
            if (0..5)
                .map(|row| self.layout[row][col])
                .all(|cell| cell.has_been_called)
            {
                return true;
            }
        }

        false
    }

    fn get_score(&self) -> i32 {
        self.layout
            .iter()
            .flatten()
            .filter(|cell| !cell.has_been_called)
            .map(|cell| cell.number)
            .sum()
    }
}

#[must_use]
pub fn part_a(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let call_order: Vec<i32> = lines[0].split(',').filter_map(|s| s.parse().ok()).collect();

    let mut boards: Vec<BingoBoard> = Vec::new();
    for board in &lines
        .iter()
        .skip(1)
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .chunks(5)
    {
        let board: Vec<&str> = board.collect();
        boards.push(BingoBoard::with_layout(board));
    }

    for n in call_order {
        for board in &mut boards {
            board.call(n);
            if board.is_winner() {
                return (board.get_score() * n).to_string();
            }
        }
    }
    "uh oh".to_string()
}

#[must_use]
pub fn part_b() -> String {
    "world".into()
}

#[cfg(test)]
mod tests {
    const PROBLEM_TEXT: &str = include_str!("inputs/problem_4.txt");

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(PROBLEM_TEXT), "32844");
    }

    // #[test]
    // fn part_b() {
    //     assert_eq!(super::part_b(PROBLEM_TEXT), "1567");
    // }
}
