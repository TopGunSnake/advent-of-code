use std::fs;

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
        println!("Processing step {}", i+1);
    }
    poly.get_result()
}

#[derive(Debug)]
struct Polymer {
    data: Vec<char>,
    rules: Vec<(char, char, char)>,
}

impl Polymer {
    fn new(input: &str) -> Self {
        use regex::Regex;

        let mut lines = input.lines();
        let data = lines.next().unwrap().chars().collect_vec();
        let rule_regex = Regex::new(r#"([A-Z]{1})([A-Z]{1}) -> ([A-Z]{1})"#).unwrap();

        let rules = lines
            .filter_map(|s| rule_regex.captures(s))
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().chars().next().unwrap(),
                    cap.get(2).unwrap().as_str().chars().next().unwrap(),
                    cap.get(3).unwrap().as_str().chars().next().unwrap(),
                )
            })
            .collect_vec();
        Self { data, rules }
    }

    fn iterate(self) -> Self {
        let rules = self.rules;

        let inserts = self
            .data
            .iter()
            .tuple_windows()
            .map(|(&left, &right)| {
                let insert = rules
                    .iter()
                    .filter_map(|&rule| match rule {
                        (l, r, insert) if l == left && r == right => Some(insert),
                        _ => None,
                    })
                    .at_most_one()
                    .unwrap();
                insert
            })
            .collect_vec(); // Now I've got a vector of Option<char>, where each can be inserted between the characters.

        let mut data = Vec::new();
        for (d, i) in zip(self.data.iter(), inserts.iter()) {
            data.push(*d); // We push the character,
            if let Some(c) = *i {
                data.push(c)
            } // If we have an insert, we push it
        }
        data.push(*self.data.last().unwrap()); // Since the inserts are from slices, there are n-1 in inserts, and n in data, so we add the last character.

        Self { data, rules }
    }

    fn get_result(&self) -> u128 {
        // Gets the quantity of most common and least commons characters in data, and returns the difference.
        let groups = self.data.iter().sorted().group_by(|&x| x);

        if let Some((min, max)) = groups
            .into_iter()
            .map(|(key, group)| group.count())
            // .inspect(|g| println!("Group Counts: {:?}", g))
            .minmax_by_key(|&count| count)
            .into_option()
        {
            (max - min).try_into().unwrap()
        } else {
            0
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

        let result = do_the_thing(input, 10);
        assert_eq!(1588, result);

        let result = do_the_thing(input, 40);
        assert_eq!(2188189693529, result);
    }
}
