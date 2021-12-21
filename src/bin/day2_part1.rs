use itertools::Itertools;
use std::fs;

#[derive(Default)]
struct SubmarineState {
    x: i32,
    y: i32,
}

impl SubmarineState {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Up(y) => self.y -= y,
            Command::Down(y) => self.y += y,
            Command::Forward(x) => self.x += x,
        };
    }
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn parse_command(command: &str) -> Self {
        let (direction, distance) = command.split_whitespace().collect_tuple().unwrap();
        let distance = distance.parse::<i32>().unwrap();

        match direction {
            "up" => Command::Up(distance),
            "forward" => Command::Forward(distance),
            "down" => Command::Down(distance),
            _ => panic!("invalid input"),
        }
    }
}

fn main() {
    let filename = "day2_input.txt";
    let input_commands = fs::read_to_string(filename).expect("File read error");

    println!("{}", input_commands);

    let result = travel_simulation(&input_commands);

    println!("Final result: {}", result);
}

fn travel_simulation(input_commands: &str) -> i32 {
    let mut pos = SubmarineState::default();
    for command in input_commands.lines() {
        let c = Command::parse_command(command);
        pos.execute_command(c);
    }
    pos.x * pos.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input_commands = indoc! {"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"};

        let result = travel_simulation(input_commands);

        assert_eq!(150, result);
    }
}
