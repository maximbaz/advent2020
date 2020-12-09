use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day5.txt").expect("Error reading the file")
}

fn input(string: &str) -> Vec<usize> {
    string.trim().lines().map(to_id).collect()
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

fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap_or(&0)
}

fn part2(input: &[usize]) -> usize {
    let given = input.iter().cloned().collect::<HashSet<usize>>();

    *iproduct!(1..127, 0..=7)
        .map(|(row, col)| row * 8 + col)
        .collect::<HashSet<usize>>()
        .difference(&given)
        .filter(|&v| given.contains(&(v - 1)) && given.contains(&(v + 1)))
        .next()
        .unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            820,
            part1(&input(
                &"
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL
            "
            ))
        );
    }
}
