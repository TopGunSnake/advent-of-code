use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let filename = "day21_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
struct Player {
    position: u8, // value from 1 - 10
    score: u128,  // Score from moving
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
struct Game {
    players: Vec<Player>,
    turn: bool,
    finished: bool,
}

impl Game {
    fn new(players: &[Player]) -> Self {
        Self {
            players: players.to_vec(),
            turn: true,
            finished: false,
        }
    }

    // A game generates new games, or itself if it wins.
    fn next(&self) -> Vec<Game> {
        // First, if this game has ended, we return itself.
        if self.players.iter().any(|p| p.wins()) {
            return vec![self.clone()];
        }

        // Otherwise, we simulate the new games (universes)
        let mut new_games = Vec::new();

        for roll in (1..=3)
            .cartesian_product(1..=3)
            .cartesian_product(1..=3)
            .map(|((r1, r2), r3)| r1 + r2 + r3)
        {
            let mut new_game = self.clone();

            let active_player = if new_game.turn {
                new_game.players.get_mut(0).unwrap()
            } else {
                new_game.players.get_mut(1).unwrap()
            };

            active_player.move_player(roll); // we simulate this game
            if active_player.wins() {
                // We mark if the game has won.
                new_game.finished = true;
            }
            new_game.turn = !new_game.turn; // We track that the game turn has changed.

            new_games.push(new_game);
        }

        new_games
    }
}
impl Player {
    fn new(position: u8) -> Self {
        Self { position, score: 0 }
    }

    fn wins(&self) -> bool {
        self.score >= 21
    }

    fn move_player(&mut self, roll: u128) {
        let change_pos = (roll % 10) as u8; // this tracks how much we actually move, every 10 is a non-move.

        self.position += change_pos; // Move around the board

        self.position %= 10; // Adjust for passing 10.

        if self.position == 0 {
            self.position = 10;
        } // Adjust for 0 being 10.

        self.score += self.position as u128; // Boost score by position we landed at.
    }
}

fn do_the_thing(input: &str) -> u128 {
    let re = Regex::new(r#"Player (\d) starting position: (\d+)"#).unwrap();

    let players = input
        .lines()
        .map(|s| {
            let captures = re.captures(s).unwrap();
            Player::new(captures.get(2).unwrap().as_str().parse::<u8>().unwrap())
        })
        .collect_vec();

    let game = Game::new(&players); // Our origin.

    let mut universes = HashMap::<Game, u128>::new(); // A map of game to count of games.
    universes.insert(game, 1);
    // let mut universes = vec![game];
    let mut iteration = 0;
    while universes.iter().any(|(game, _)| !game.finished) {
        // Until all games are tracked as finished.
        let new_universes = universes
            .into_iter()
            .flat_map(|universe| {
                let origin_game = universe.0;
                let count = universe.1;

                origin_game
                    .next() // Either this game (if it wins), or the children of this game if it hasn't ended.
                    .into_iter()
                    .map(move |game| (game, count))
            }) // We now have an iterator of the games that now exist.
            .sorted_by_key(|(game, _count)| game.clone())
            // .inspect(|(game, count)| {
            //     println!(
            //         "Games {}, Score: {}, {}",
            //         count, game.players[0].score, game.players[1].score
            //     )
            // })
            .coalesce(|previous, current| {
                if previous.0 == current.0 {
                    Ok((previous.0, previous.1 + current.1))
                } else {
                    Err((previous, current))
                }
            });

        universes = new_universes.collect();
        println!(
            "Processing iteration {}. New count of universes is {}. Unique is {}",
            iteration,
            universes.iter().fold(0, |acc, item| acc + *item.1),
            universes.len()
        );
        // println!("Universes: {:?}", universes.values());
        // std::thread::sleep(std::time::Duration::from_secs(1));
        iteration += 1;
    }
    // Now we need the winningest player.
    let scores = universes
        .iter()
        .map(|(game, count)| (game.players[0].wins(), count)) // (player, wins) over the possible game states
        .fold((0u128, 0u128), |acc, (player, count)| {
            if player {
                (acc.0 + count, acc.1)
            } else {
                (acc.0, acc.1 + count)
            }
        });

    scores.0.max(scores.1)
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

        assert_eq!(444356092776315, result);
    }
}
