use super::Solution;

pub struct DayTwo;

impl DayTwo {
    fn get_movements(input: String) -> Vec<(String, i32)> {
        // Split by line
        let movements = input.lines().filter(|s| !s.is_empty()).map(|s| {
            let mut splitter = s.split_ascii_whitespace();
            (
                splitter.next().unwrap().to_string(),
                splitter.next().unwrap().parse::<i32>().unwrap(),
            )
        });

        movements.collect()
    }
}

impl Solution for DayTwo {
    type Output = i32;

    fn input() -> String {
        include_str!("./inputs/2.txt").to_string()
    }

    fn solve_first<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let mut horizontal = 0;
        let mut depth = 0;

        for (instruction, movement) in Self::get_movements(input.to_string()) {
            match instruction.as_str() {
                "up" => depth -= movement,
                "down" => depth += movement,
                "forward" => horizontal += movement,
                _ => panic!("Undefined instruction"),
            };
        }

        horizontal * depth
    }

    fn solve_second<S: AsRef<str>>(input: S) -> Self::Output {
        let input = input.as_ref();

        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (instruction, movement) in Self::get_movements(input.to_string()) {
            match instruction.as_str() {
                "up" => aim -= movement,
                "down" => aim += movement,
                "forward" => {
                    horizontal += movement;
                    depth += aim * movement;
                }
                _ => panic!("Undefined instruction"),
            };
        }

        horizontal * depth
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (1868935, 1965970888)
    }
}