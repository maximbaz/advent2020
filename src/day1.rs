use itertools::Itertools;
use std::fs;

pub fn run_part1() -> i32 {
    part1(&input())
}

pub fn run_part2() -> i32 {
    part2(&input())
}

fn part1(input: &[i32]) -> i32 {
    solve(input, 2)
}

fn part2(input: &[i32]) -> i32 {
    solve(input, 3)
}

fn input() -> Vec<i32> {
    fs::read_to_string("input/day1.txt")
        .expect("Error reading the file")
        .trim()
        .lines()
        .flat_map(str::parse)
        .collect()
}

fn solve(input: &[i32], num_comb: usize) -> i32 {
    input
        .iter()
        .cloned()
        .combinations(num_comb)
        .find(|comb| comb.iter().sum::<i32>() == 2020)
        .map(|comb| comb.iter().product())
        .expect("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(20100, part1(&vec![2010, 15, 10]));
        assert_eq!(514579, part1(&vec![1721, 979, 366, 299, 675, 1456]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(50250, part2(&vec![2010, 5, 10, 5]));
        assert_eq!(241861950, part2(&vec![1721, 979, 366, 299, 675, 1456]));
    }
}
