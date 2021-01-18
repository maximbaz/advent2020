use super::super::*;
use std::collections::HashSet;

pub struct Task;

impl Solution for Task {
    type Input = Vec<Vec<HashSet<char>>>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .split("\n\n")
            .map(|s| s.lines().map(|l| l.chars().collect()).collect())
            .collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        input.iter().map(|sets| union_all(sets).len()).sum()
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        input.iter().map(|sets| intersection_all(sets).len()).sum()
    }
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
            Task.part1(
                Task.parse_input(
                    "
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
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            6,
            Task.part2(
                Task.parse_input(
                    "
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
                    .to_string()
                )
            )
        );
    }
}
