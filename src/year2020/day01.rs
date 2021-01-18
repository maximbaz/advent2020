use super::super::*;
use itertools::Itertools;

pub struct Task;

impl Solution for Task {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: String) -> Self::Input {
        input.trim().lines().flat_map(str::parse).collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        solve(&input, 2)
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        solve(&input, 3)
    }
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
        assert_eq!(20100, Task.part1(vec![2010, 15, 10]));
        assert_eq!(514579, Task.part1(vec![1721, 979, 366, 299, 675, 1456]));
    }

    #[test]
    fn test_part2() {
        assert_eq!(50250, Task.part2(vec![2010, 5, 10, 5]));
        assert_eq!(241861950, Task.part2(vec![1721, 979, 366, 299, 675, 1456]));
    }
}
