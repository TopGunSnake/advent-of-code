use std::fs;

use petgraph::{
    adj::Neighbors,
    dot,
    prelude::*,
    visit::{depth_first_search, Control, DfsEvent, Walker},
};

use itertools::Itertools;

fn main() {
    let filename = "day12_input.txt";
    let contents = fs::read_to_string(filename).expect("File read error");

    println!("{}", contents);

    let result = do_the_thing(&contents);

    println!("Result: {}", result);
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Cave<'a> {
    Small(&'a str),
    Big(&'a str),
    Start,
    End,
}

struct CaveWalker<'a> {
    stack: Vec<Cave<'a>>,
    small_visited: Vec<Cave<'a>>,
}

impl<'a> CaveWalker<'a> {
    fn new(start: Cave<'a>) -> Self {
        let stack = vec![start];
        Self {
            stack,
            small_visited: Vec::new(),
        }
    }
}

impl<'a> Walker<&UnGraphMap<Cave<'a>, bool>> for CaveWalker<'a> {
    type Item = Cave<'a>;

    fn walk_next(&mut self, context: &UnGraphMap<Cave<'a>, bool>) -> Option<Self::Item> {
        if let Some(cave) = self.stack.pop() {
            // We have a cave to explore!

            // If the cave is small, then we need to track it.
            if !matches!(cave, Cave::Small(_) | Cave::Start) {
                self.small_visited.push(cave);
            }

            // We need to get the possible caves to go to next.
            let neighbors = context.neighbors(cave);

            // If a cave we could explore is a small cave we've already seen, then we don't want to explore it.
            let neighbors = neighbors.filter(|cave| !self.small_visited.contains(cave));

            // Push the caves to our list of caves to explore
            self.stack.extend(neighbors);

            dbg!(&self.stack);
            dbg!(&self.small_visited);

            Some(cave)
        } else {
            // All out of caves
            None
        }
    }
}

fn do_the_thing(input: &str) -> u32 {
    let edges = input.lines().map(|s| {
        if let Some((a, b)) = s
            .split_terminator('-')
            .map(|s| {
                if s.contains("start") {
                    Cave::Start
                } else if s.contains("end") {
                    Cave::End
                } else if s.chars().all(|c| c.is_ascii_lowercase()) {
                    Cave::Small(s)
                } else {
                    Cave::Big(s)
                }
            })
            .collect_tuple()
        {
            (a, b)
        } else {
            dbg!(s);
            panic!("Malformed input!");
        }
    });
    let graph: UnGraphMap<Cave, bool> = UnGraphMap::from_edges(edges);
    // Traverse the graph with a backtracking algorithm
    println!("{:?}", dot::Dot::new(&graph));

    let start = Cave::Start;
    let end = Cave::End;
    let mut walker = CaveWalker::new(start);
    let mut count = 0;
    while let Some(cave) = walker.walk_next(&graph) {
        println!("Cave {:?} Visited.", cave);
        count += 1;
        // if cave == end {
        //     count += 1;
        // }
        if count > 10 {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example_small() {
        let input = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
        "};
        // println!("{}", example_string);
        let result = do_the_thing(input);
        assert_eq!(10, result);
    }

    #[test]
    fn test_example_medium() {
        let input = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
        "};
        // println!("{}", example_string);
        let result = do_the_thing(input);
        assert_eq!(19, result);
    }

    #[test]
    fn test_example_large() {
        let input = indoc! {"
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
        "};
        // println!("{}", example_string);
        let result = do_the_thing(input);
        assert_eq!(226, result);
    }
}
