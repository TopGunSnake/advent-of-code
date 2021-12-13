use std::fs;

use itertools::Itertools;

fn main() {
    let filename = "day8_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

fn do_the_thing(input: &str) -> u32 {
    let data = input.lines().map(|s| {
        s.split_terminator(" | ") // Left is signal pattern, right is 4 digit output
    });

    for datum in data {
        if let Some((patterns, display)) = datum.collect_tuple() {
            // patterns.split_whitespace()
        }
    }
    0
}

fn parse_input(input: &str) -> () {
    let data = input.lines().map(|s| {
        s.split_terminator(" | ") // Left is signal pattern, right is 4 digit output
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        let result = do_the_thing(input);

        assert_eq!(26, result);
    }
}
