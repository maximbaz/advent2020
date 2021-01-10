use super::super::*;
use itertools::iproduct;
use std::collections::HashSet;

pub struct Task;

impl Solution for Task {
    type Input = Vec<usize>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input.trim().lines().map(to_id).collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        *input.iter().max().expect("no solution found")
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let given = input.iter().cloned().collect::<HashSet<usize>>();

        *iproduct!(1..127, 0..=7)
            .map(|(row, col)| row * 8 + col)
            .collect::<HashSet<usize>>()
            .difference(&given)
            .find(|&v| given.contains(&(v - 1)) && given.contains(&(v + 1)))
            .expect("no solution found")
    }
}

fn to_id(s: &str) -> usize {
    usize::from_str_radix(
        &s.replace("F", "0")
            .replace("L", "0")
            .replace("B", "1")
            .replace("R", "1"),
        2,
    )
    .expect("invalid seat string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            820,
            Task.part1(
                &Task.parse_input(
                    "
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
                    "
                    .to_string()
                )
            )
        );
    }
}
