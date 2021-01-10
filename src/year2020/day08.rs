use super::super::*;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Task;

type Ops = Vec<(String, i32)>;

impl Solution for Task {
    type Input = Ops;
    type Output = i32;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|s| {
                let p = s.split(' ').collect_vec();
                (
                    p[0].to_string(),
                    p[1].parse().expect("value is not a number"),
                )
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input).0
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        (0..input.len())
            .map(|i| {
                let mut swapped = input.clone();
                swapped[i] = match (swapped[i].0.as_str(), swapped[i].1) {
                    ("jmp", n) => ("nop".to_string(), n),
                    ("nop", n) => ("jmp".to_string(), n),
                    (op, n) => (op.to_string(), n),
                };
                solve(&swapped)
            })
            .find(|(_, terminated)| *terminated)
            .expect("no solution found")
            .0
    }
}

fn solve(input: &Ops) -> (i32, bool) {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut addr = 0;

    while addr < input.len() && visited.insert(addr) {
        let mut jmp = 1;
        match (input[addr].0.as_str(), input[addr].1) {
            ("acc", n) => acc += n,
            ("jmp", n) => jmp = n,
            _ => (),
        }
        addr = ((addr as i32) + jmp) as usize;
    }

    (acc, addr == input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            5,
            Task.part1(
                &Task.parse_input(
                    "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            8,
            Task.part2(
                &Task.parse_input(
                    "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
                    "
                    .to_string()
                )
            )
        );
    }
}
