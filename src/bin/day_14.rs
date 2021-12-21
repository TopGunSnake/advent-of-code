use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::{zip, Itertools};

fn main() {
    let filename = "day14_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents, 10);

    println!("Result {}", result);
}

fn do_the_thing(input: &str, iters: u8) -> u128 {
    let mut poly = Polymer::new(input);

    for i in 0..iters {
        poly = poly.iterate();
        println!("Processing step {}", i + 1);
    }
    poly.get_result()
}

#[derive(Debug)]
struct Polymer {
    rules: HashMap<(char, char), PolymerPair>,
}

// Defines how a pair inserts, as well as track counts.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct PolymerPair {
    pair: (char, char),
    insert_result: Option<char>,
    count: u128,
}

impl PolymerPair {
    /// Expands this polymer pair into the left and right pairs.
    /// Returns None if this pair doesn't expand.
    fn expand(&self) -> Option<((char, char), (char, char))> {
        if self.insert_result.is_some() {
            let left = (self.pair.0, self.insert_result.unwrap());
            let right = (self.insert_result.unwrap(), self.pair.1);
            Some((left, right))
        } else {
            None
        }
    }

    // increments this pair's count
    fn increment(&mut self, inc: u128) {
        self.count += inc;
    }

    // decrements this pair's count
    fn decrement(&mut self, dec: u128) {
        self.count -= dec;
    }
}

impl Polymer {
    fn new(input: &str) -> Self {
        use regex::Regex;

        let mut lines = input.lines();
        let data = lines.next().unwrap().chars().collect_vec();

        let rule_regex = Regex::new(r#"([A-Z]{1})([A-Z]{1}) -> ([A-Z]{1})"#).unwrap();

        let mut rules = lines
            .filter_map(|s| rule_regex.captures(s))
            .map(|cap| {
                let left = cap.get(1).unwrap().as_str().chars().next().unwrap();
                let right = cap.get(2).unwrap().as_str().chars().next().unwrap();
                let insert_result = cap.get(3).unwrap().as_str().chars().next();
                (
                    (left, right),
                    PolymerPair {
                        pair: (left, right),
                        insert_result,
                        count: 0,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        for (left, right) in data.into_iter().tuple_windows() {
            if let Some(pair) = rules.get_mut(&(left, right)) {
                pair.increment(1);
            };
        }

        Self { rules }
    }

    fn iterate(self) -> Self {
        let mut rules = self.rules.clone();

        for pair in self.rules.values() {
            println!("Processing Pair: {:?}, count {}", pair.pair, pair.count);
            if let Some((left, right)) = pair.expand() {
                // This pair in the map contributes to increasing the polymer.
                let count = pair.count;

                Self::update_rules_map(&mut rules, pair.pair, count, false); // We remove each pair from the original count.
                Self::update_rules_map(&mut rules, left, count, true); // We add each new left pair to the count.
                Self::update_rules_map(&mut rules, right, count, true); // We add each new right pair to the count.
            }
        }
        Self { rules }
    }

    fn get_result(&self) -> u128 {
        todo!("This function currently counts the PolymerPairs. Instead, we need to count the specific instances in the pairs.");
        // Gets the quantity of most common and least commons characters in data, and returns the difference.
        if let Some((min, max)) = self
            .rules
            .iter()
            .map(|(_, v)| v.count)
            .minmax()
            .into_option()
        {
            dbg!(max) - dbg!(min)
        } else {
            0
        }
    }

    fn update_rules_map(
        rules: &mut HashMap<(char, char), PolymerPair>,
        pair: (char, char),
        count: u128,
        increment: bool,
    ) {
        if let Some(x) = rules.get_mut(&pair) {
            // We have this rule in our map, we need to adjust it.
            if increment {
                x.increment(count);
            } else {
                x.decrement(count);
            }
        } else {
            // This rule is not defined, so we need to add the pair manually.
            let new_pair = PolymerPair {
                pair,
                insert_result: None,
                count: 0,
            };
            rules.insert(pair, new_pair);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;

    // #[test_case()]
    // fn test_specific_cases(input: &str, expected: &str) {
    //     let result = calculate_auto_complete(input);

    //     assert_eq!(expected, result);
    // }

    #[test]
    fn test_example() {
        let input = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C"};

        // let result = do_the_thing(input, 3);
        // assert_eq!(1, result);

        let result = do_the_thing(input, 10);
        assert_eq!(1588, result);

        // let result = do_the_thing(input, 40);
        // assert_eq!(2188189693529, result);
    }
}
