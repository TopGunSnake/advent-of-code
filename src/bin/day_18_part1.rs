use std::fs;

use either::Either;

fn main() {
    let filename = "day18_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

fn do_the_thing(input: &str) -> u128 {
    0
}

#[derive(PartialEq, Eq, Debug)]
struct SailfishNumber {
    depth: usize,
    left: Either<Box<SailfishNumber>, u128>,
    right: Either<Box<SailfishNumber>, u128>,
}

impl SailfishNumber {
    fn from_str(depth: usize, input: &str) -> Self {
        // Sailfish numbers are [Either<SailFish, u128>, Either<SailFish, u128>]
        let mut bracket_count = 0;
        let mut split_index = 0;
        for (i, c) in input.chars().enumerate() {
            match (i, c) {
                (_, '[') => bracket_count += 1, // We've moved in deeper.
                (_, ']') => bracket_count -= 1, // We've left a nested number.
                (index, ',') if bracket_count == 1 => {
                    split_index = index;
                    break;
                } // We've found the comma that splits the left and right.
                _ => (),                        // Nothing has happened.
            }
        }

        let (left, right) = (
            &input[1..split_index],
            &input[split_index + 1..input.len() - 1],
        );

        let left = left.parse::<u128>().map_or_else(
            |_| Either::Left(Box::new(SailfishNumber::from_str(depth + 1, left))),
            Either::Right,
        );
        let right = right.parse::<u128>().map_or_else(
            |_| Either::Left(Box::new(SailfishNumber::from_str(depth + 1, right))),
            Either::Right,
        );
        SailfishNumber { depth, left, right }
    }

    fn reduce(&mut self) {
        // While we can do anything in the list:
        loop {
            // If any pair is nested inside four pairs, the leftmost such pair explodes.
            while self.explode().is_some() {
                continue;
            }
            // If any regular number is 10 or greater, the leftmost such regular number splits.
            if self.split() {
                continue;
            }

            // Else, we are done reducing
            break;
        }
    }

    // Alters the SailFish, returning the pair that exploded if it explodes
    fn explode(&mut self) -> Option<(u128, u128)> {
        /*
        the pair's left value is added to the first regular number to the left of the exploding pair (if any),
        and the pair's right value is added to the first regular number to the right of the exploding pair (if any).
        Only applies to a regular number pair
        */
        if self.left.is_right() && self.right.is_right() && self.depth >= 4 {
            // This is a regular node inside at least four pairs.
            Some((self.left.right().unwrap(), self.right.right().unwrap()))
        } else {
            let left_attempt = self.left.as_mut().map_left(|left| left.explode());
            let right_attempt = self.right.as_mut().map_left(|left| left.explode());

            // If the left attempt returns an Either::Left, then we need to add the right side to any regular at self.right.
            if left_attempt.is_left() {
                self.left = Either::Right(left_attempt.left().unwrap().unwrap().0);
                if self.right.is_right() {
                    self.right = Either::Right(
                        self.right.right().unwrap() + left_attempt.left().unwrap().unwrap().1,
                    );
                }
            } else if right_attempt.is_left() {
                self.right = Either::Right(right_attempt.left().unwrap().unwrap().1);
                if self.left.is_right() {
                    self.left = Either::Right(
                        self.left.right().unwrap() + right_attempt.left().unwrap().unwrap().0,
                    );
                }
            }
            None
        }
    }

    // Returns true if we splitted
    fn split(&mut self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;
    #[test]
    fn test_build() {
        let input = "[1,2]";

        let result = SailfishNumber::from_str(0, input);
    }

    #[test]
    fn test_split() {}

    #[test_case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]")]
    #[test_case(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
    )]
    #[test_case("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]")]
    fn test_explode(input: &str, expected: &str) {
        let mut input = SailfishNumber::from_str(0, input);
        let expected = SailfishNumber::from_str(0, expected);

        input.explode();

        assert_eq!(input, expected);
    }

    #[test]
    fn test_equal() {
        let text = "[[1,2],3]";

        let left = SailfishNumber::from_str(0, text);
        let right = SailfishNumber::from_str(0, text);

        assert_eq!(left, right);
    }
}
