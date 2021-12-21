use std::fs;

use itertools::{enumerate, Itertools};

fn main() {
    let filename = "day4_input.txt";
    let input = fs::read_to_string(filename).expect("File read error");

    println!("{}", input);

    let result = do_the_thing(&input);

    println!("Final result: {}", result);
}

fn do_the_thing(input: &str) -> u32 {
    let (sequence, mut boards) = parse(input);
    println!("Sequence: {:?}", sequence);
    println!("Boards: {:?}", boards);

    let (winning_number, unmarked_sum) = simulate_boards(&sequence, &mut boards);

    winning_number * unmarked_sum
}

fn simulate_boards(seq: &[u32], boards: &mut [Board]) -> (u32, u32) {
    let mut winning_number = 0;
    let mut unmarked_sum = 0;
    let mut winner_tracker = Vec::new();
    for number in seq {
        println!("Number called: {}", number);

        for (i, board) in boards.iter_mut().enumerate() {
            let wins = board.mark(*number);
            // println!("Boards: {:?}", board);
            if wins {
                if winner_tracker.contains(&i) {
                    continue;
                } else {
                    winner_tracker.push(i);

                    // Update the latest winner
                    winning_number = *number;
                    unmarked_sum = board.unmarked_sum();

                    // println!("Winning Board: {:?}", board);
                    println!("Results: {} {}", winning_number, unmarked_sum);
                }
            }
        }
    }
    (winning_number, unmarked_sum)
}

fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();
    let seq = if let Some(l) = lines.next() {
        l.split_terminator(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_vec()
    } else {
        panic!("Malformed first line!");
    };

    let mut boards = Vec::new();

    let mut counter = 0;
    let mut board = Board::default();

    for line in lines {
        println!("{:?}", line);
        if line.chars().count() > 1 {
            // Line has data
            let result = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .map(|n| (n, false))
                .collect_vec();
            for (i, item) in enumerate(result) {
                board.board_numbers[5 * counter + i] = item;
            }
            counter += 1;
        }

        if counter == 5 {
            // We have completed a board:
            boards.push(board);
            board = Board::default();
            counter = 0;
        }
    }
    (seq, boards)
}

#[derive(Debug, Default)]
struct Board {
    board_numbers: [(u32, bool); 25],
}

impl Board {
    fn unmarked_sum(&self) -> u32 {
        let sum = self
            .board_numbers
            .iter()
            .filter(|(_, x)| !x)
            .map(|x| x.0)
            .sum();
        sum
    }

    fn mark(&mut self, number: u32) -> bool {
        let found = self.board_numbers.iter().find_position(|x| x.0 == number);

        match found {
            Some((x, _)) => {
                self.board_numbers[x].1 = true;
                Self::check_wins(self.board_numbers, x)
            }
            None => false,
        }
    }

    fn check_wins(board_numbers: [(u32, bool); 25], position: usize) -> bool {
        // We get a clue for what cases to check based on the position.
        let (x, y) = (position % 5, position / 5);
        println!("Evaluating ({}, {})", x, y);
        // check row (is contiguous)
        let mut count = 0;
        let row = board_numbers
            .iter()
            .enumerate()
            .filter(|&(i, _)| i / 5 == y)
            .map(|(_, &(_, x))| {
                // println!("{}", x);
                count += 1;
                x
            })
            .all(|x| x);
        println!("Row count {}", count);
        // check col (is not contiguous)
        let col = board_numbers
            .iter()
            .enumerate()
            .filter(|&(i, _)| i % 5 == x)
            .map(|(_, &(_, x))| x)
            .all(|x| x);
        println!("{} {} -> {}", row, col, row || col);
        row || col
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_example() {
        let input = indoc! {"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        "};

        let result = do_the_thing(input);

        assert_eq!(1924, result);
    }
}
