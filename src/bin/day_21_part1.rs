use itertools::Itertools;
use regex::Regex;
use std::{fs, thread, time};

fn main() {
    let filename = "day21_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

trait Dice {
    fn roll(&mut self) -> u8;
    fn total_rolls(&self) -> u128;
}

struct DeterministicDice {
    next_roll: u8,
    rolls: u128,
}

impl DeterministicDice {
    fn new() -> Self {
        Self {
            next_roll: 1,
            rolls: 0,
        }
    }
}

impl Dice for DeterministicDice {
    fn roll(&mut self) -> u8 {
        let next = self.next_roll;

        self.next_roll += 1; // Increment the roll

        if self.next_roll > 100 {
            // Rollover after 100.
            self.next_roll = 1;
        }

        self.rolls += 1; // Track the roll

        dbg!(next)
    }

    fn total_rolls(&self) -> u128 {
        self.rolls
    }
}

#[derive(Debug)]
struct Player {
    position: u8, // value from 1 - 10
    score: u128,  // Score from moving
}

impl Player {
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    fn wins(&self) -> bool {
        self.score >= 1000
    }

    fn move_player(&mut self, roll: u128) {
        let change_pos = (roll % 10) as u8; // this tracks how much we actually move, every 10 is a non-move.

        self.position += change_pos; // Move around the board

        self.position %= 10; // Adjust for passing 10.

        if self.position == 0 {
            self.position = 10;
        } // Adjust for 0 being 10.

        dbg!(self.position);

        self.score += self.position as u128; // Boost score by position we landed at.
    }
}

fn do_the_thing(input: &str) -> u128 {
    let re = Regex::new(r#"Player (\d) starting position: (\d+)"#).unwrap();

    let mut players = input
        .lines()
        .map(|s| {
            let captures = re.captures(s).unwrap();
            Player::new(captures.get(2).unwrap().as_str().parse::<u8>().unwrap())
        })
        .collect_vec();

    let mut dice = DeterministicDice::new();
    'game: loop {
        '_turn: for player in players.iter_mut() {
            let roll = (0..3).map(|_| dice.roll() as u128).sum();
            player.move_player(roll);

            if player.wins() {
                break 'game;
            }
            // thread::sleep(time::Duration::from_secs(1));
        }
    }

    let loser = players
        .iter()
        .inspect(|p| println!("{:?}", p))
        .map(|p| p.score)
        .min()
        .unwrap();
    let rolls = dice.total_rolls();

    loser * rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"Player 1 starting position: 4
        Player 2 starting position: 8"};

        let result = do_the_thing(input);

        assert_eq!(739785, result);
    }

    #[test]
    fn test_start_at_10() {
        let mut player = Player::new(10);

        let roll = 1+2+3;
        player.move_player(roll);

        assert_eq!(player.position, 6);
    }
}
