use std::collections::HashSet;
use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day6.txt").expect("Error reading the file")
}

fn input(string: &str) -> Vec<Vec<HashSet<char>>> {
    string
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.chars().collect()).collect())
        .collect()
}

fn part1(input: &[Vec<HashSet<char>>]) -> usize {
    input.iter().map(|sets| union_all(sets).len()).sum()
}

fn part2(input: &[Vec<HashSet<char>>]) -> usize {
    input.iter().map(|sets| intersection_all(sets).len()).sum()
}

fn union_all(sets: &[HashSet<char>]) -> HashSet<char> {
    sets.iter().fold(HashSet::new(), |result, set| {
        result.union(set).cloned().collect()
    })
}

fn intersection_all(sets: &[HashSet<char>]) -> HashSet<char> {
    sets.iter().fold(sets[0].clone(), |result, set| {
        result.intersection(set).cloned().collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            11,
            part1(&input(
                &"
abc

a
b
c

ab
ac

a
a
a
a

b
            "
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            6,
            part2(&input(
                &"
abc

a
b
c

ab
ac

a
a
a
a

b
            "
            ))
        );
    }
}
