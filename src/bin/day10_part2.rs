use std::fs;

use itertools::Itertools;

fn main() {
    let filename = "day10_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result {}", result);
}

#[derive(Debug)]
struct ChunkDelimiter {
    open: bool,
    character: ChunkDelimiterType,
}

#[derive(PartialEq, Eq, Debug)]
enum ChunkDelimiterType {
    Parenthesis,
    Bracket,
    Brace,
    Caret,
}

impl ChunkDelimiter {
    fn from_char(c: char) -> Self {
        match c {
            '(' => Self {
                open: true,
                character: ChunkDelimiterType::Parenthesis,
            },
            '[' => Self {
                open: true,
                character: ChunkDelimiterType::Bracket,
            },
            '{' => Self {
                open: true,
                character: ChunkDelimiterType::Brace,
            },
            '<' => Self {
                open: true,
                character: ChunkDelimiterType::Caret,
            },
            ')' => Self {
                open: false,
                character: ChunkDelimiterType::Parenthesis,
            },
            ']' => Self {
                open: false,
                character: ChunkDelimiterType::Bracket,
            },
            '}' => Self {
                open: false,
                character: ChunkDelimiterType::Brace,
            },
            '>' => Self {
                open: false,
                character: ChunkDelimiterType::Caret,
            },
            c => panic!("Invalid character found: {}", c),
        }
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn close(&self, closer: ChunkDelimiter) -> bool {
        self.character == closer.character && self.open && !closer.open
    }
}

fn do_the_thing(input: &str) -> u128 {
    let scores = input
        .lines()
        .filter(|line| drop_corrupted_lines(line)) // Drop corrupted lines
        .map(calculate_auto_complete) // Calculate individual line scores for auto completion
        .sorted()
        .collect_vec();
    let total_score = scores.get(scores.len() / 2).unwrap();
    *total_score
}

fn drop_corrupted_lines(line: &str) -> bool {
    let mut stack = Vec::new();
    for c in line.chars() {
        let case = ChunkDelimiter::from_char(c);
        // We have a delimiter. If it opens, add to stack:
        if case.is_open() {
            stack.push(case);
        } else {
            let open = stack.last();
            // If the delimiter closes, it must match the top of the stack.
            if open.map_or(false, |opening| opening.close(case)) {
                // The delimiter closes, we can pop the stack.
                stack.pop();
            } else {
                // The line is invalid, and we need to kick the result out.
                // println!(
                //     "Expected {}, but found {} instead.",
                //     open.map(|chunkdelim| chunkdelim.as_char().to_string())
                //         .unwrap_or_else(|| "".to_string()),
                //     c
                // );
                // println!("Stack State: {:?}", stack);
                return false;
            }
        }
    }

    true
}

fn calculate_auto_complete(line: &str) -> u128 {
    let mut stack = Vec::new();
    for c in line.chars() {
        let case = ChunkDelimiter::from_char(c);
        if case.is_open() {
            stack.push(case);
        } else if stack.last().unwrap().close(case) {
            stack.pop();
        }
    }
    // We now have a stack of what is left to complete, so we work from the right to the left:
    let completing_score = stack
        .iter()
        .rev()
        .map(|cd| match cd.character {
            ChunkDelimiterType::Parenthesis => 1,
            ChunkDelimiterType::Bracket => 2,
            ChunkDelimiterType::Brace => 3,
            ChunkDelimiterType::Caret => 4,
        })
        .fold(0, |acc, x| acc * 5 + x);
    completing_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use test_case::test_case;

    #[test_case("[({(<(())[]>[[{[]{<()<>>", 288957)]
    #[test_case("[(()[<>])]({[<{<<[]>>(", 5566)]
    #[test_case("(((({<>}<{<{<>}{[]{[]{}", 1480781)]
    #[test_case("{<[[]]>}<{[{[{[]{()[[[]", 995444)]
    #[test_case("<{([{{}}[<[[[<>{}]]]>[]]", 294)]
    fn test_specific_cases(input: &str, expected: u128) {
        let result = calculate_auto_complete(input);

        assert_eq!(expected, result);
    }

    #[test]
    fn test_example() {
        let input = indoc! {"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]"};

        let result = do_the_thing(input);

        assert_eq!(288957, result);
    }
}
