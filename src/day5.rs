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

fn input<'a>(string: &'a str) -> Vec<usize> {
    string.trim().lines().map(to_id).collect()
}

fn to_id(s: &str) -> usize {
    binary_search(&s[..7]) * 8 + binary_search(&s[7..])
}

fn binary_search(s: &str) -> usize {
    s.chars()
        .fold((0, (1 << s.len()) - 1), |(l, r), dir| {
            if dir == 'F' || dir == 'L' {
                (l, (l + r) / 2)
            } else {
                ((l + r) / 2 + 1, r)
            }
        })
        .0
}

fn part1(input: &[usize]) -> usize {
    *input.iter().max().unwrap()
}

fn part2(input: &[usize]) -> usize {
    let given = input.iter().cloned().collect::<HashSet<usize>>();

    iproduct!(1..127, 0..=7)
        .map(|(row, col)| row * 8 + col)
        .collect::<HashSet<usize>>()
        .difference(&given)
        .cloned()
        .filter(|v| given.contains(&(v - 1)) && given.contains(&(v + 1)))
        .next()
        .unwrap()
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
