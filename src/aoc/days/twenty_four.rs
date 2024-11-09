use crate::aoc::Solution;

pub struct TwentyFour;

impl Solution for TwentyFour {
    type Output = u32;
    type Parsed = Vec<u32>;

    fn input() -> &'static str {
        include_str!("../inputs/24.txt")
    }

    fn parse_input(input: &'static str) -> Self::Parsed {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }

    fn solve_first(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn solve_second(_parsed: &Self::Parsed) -> Self::Output {
        0
    }

    fn expected_solutions() -> (Self::Output, Self::Output) {
        (0, 0)
    }
}
