use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

pub fn run_part1() -> i32 {
    part1(&input(&read_file()))
}

pub fn run_part2() -> i32 {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day8.txt").expect("Error reading the file")
}

type Op<'a> = (&'a str, i32);

fn input(string: &str) -> Vec<Op> {
    string
        .trim()
        .lines()
        .map(|s| {
            let p = s.split(' ').collect_vec();
            (p[0], p[1].parse().expect("value is not a number"))
        })
        .collect()
}

fn solve(input: &Vec<Op>) -> (i32, bool) {
    let mut visited = HashSet::new();
    let mut acc = 0;
    let mut addr = 0;

    while addr < input.len() && visited.insert(addr) {
        let mut jmp = 1;
        match input[addr] {
            ("acc", n) => acc += n,
            ("jmp", n) => jmp = n,
            _ => (),
        }
        addr = ((addr as i32) + jmp) as usize;
    }

    (acc, addr == input.len())
}

fn part1(input: &Vec<Op>) -> i32 {
    solve(input).0
}

fn part2(input: &Vec<Op>) -> i32 {
    (0..input.len())
        .map(|i| {
            let mut swapped = input.clone();
            swapped[i] = match swapped[i] {
                ("jmp", n) => ("nop", n),
                ("nop", n) => ("jmp", n),
                e => e,
            };
            solve(&swapped)
        })
        .find(|(_, terminated)| *terminated)
        .expect("no solution found")
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            5,
            part1(&input(
                &"
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
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            8,
            part2(&input(
                &"
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
            ))
        );
    }
}
