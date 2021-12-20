use std::fs;

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

impl ChunkDelimiterType {
    fn as_char(&self, open: bool) -> char {
        match self {
            ChunkDelimiterType::Parenthesis => {
                if open {
                    '('
                } else {
                    ')'
                }
            }
            ChunkDelimiterType::Bracket => {
                if open {
                    '['
                } else {
                    ']'
                }
            }
            ChunkDelimiterType::Brace => {
                if open {
                    '{'
                } else {
                    '}'
                }
            }
            ChunkDelimiterType::Caret => {
                if open {
                    '<'
                } else {
                    '>'
                }
            }
        }
    }
}

impl ChunkDelimiter {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '(' => Some(Self {
                open: true,
                character: ChunkDelimiterType::Parenthesis,
            }),
            '[' => Some(Self {
                open: true,
                character: ChunkDelimiterType::Bracket,
            }),
            '{' => Some(Self {
                open: true,
                character: ChunkDelimiterType::Brace,
            }),
            '<' => Some(Self {
                open: true,
                character: ChunkDelimiterType::Caret,
            }),
            ')' => Some(Self {
                open: false,
                character: ChunkDelimiterType::Parenthesis,
            }),
            ']' => Some(Self {
                open: false,
                character: ChunkDelimiterType::Bracket,
            }),
            '}' => Some(Self {
                open: false,
                character: ChunkDelimiterType::Brace,
            }),
            '>' => Some(Self {
                open: false,
                character: ChunkDelimiterType::Caret,
            }),
            _ => None,
        }
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn close(&self, closer: ChunkDelimiter) -> bool {
        self.character == closer.character && self.open && !closer.open
    }

    fn as_char(&self) -> char {
        self.character.as_char(self.open)
    }
}

fn do_the_thing(input: &str) -> u128 {
    let mut total_score = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            if let Some(case) = ChunkDelimiter::from_char(c) {
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
                        println!(
                            "Expected {}, but found {} instead.",
                            open.map(|chunkdelim| chunkdelim.as_char().to_string())
                                .unwrap_or_else(|| "".to_string()),
                            c
                        );
                        println!("Stack State: {:?}", stack);
                        let new_score = match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                        total_score += new_score;
                        break;
                    }
                }
            }
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

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

        assert_eq!(26397, result);
    }
}
