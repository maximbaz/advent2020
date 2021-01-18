use super::super::*;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Task;

impl Solution for Task {
    type Input = Vec<i64>;
    type Output = u64;

    fn parse_input(&self, input: String) -> Self::Input {
        input.trim().lines().flat_map(str::parse).collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        match prepare(&input)
            .windows(2)
            .fold((0, 0), |(ones, threes), chunk| match chunk[1] - chunk[0] {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes),
            }) {
            (ones, threes) => ones * threes,
        }
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        count(0, &prepare(&input)[1..], &mut HashMap::new())
    }
}

fn prepare(data: &[i64]) -> Vec<i64> {
    let mut data = data.iter().cloned().sorted().collect_vec();
    data.insert(0, 0);
    data.push(data[data.len() - 1] + 3);
    data
}

fn count(prev: i64, input: &[i64], cache: &mut HashMap<(i64, usize), u64>) -> u64 {
    let cache_key = (prev, input.len());

    match cache.get(&cache_key) {
        Some(&result) => result,
        None => {
            let diff = input[0] - prev;

            let result = if input.len() == 1 {
                (diff <= 3) as u64
            } else {
                match diff {
                    1 | 2 => count(prev, &input[1..], cache) + count(input[0], &input[1..], cache),
                    3 => count(input[0], &input[1..], cache),
                    _ => 0,
                }
            };

            cache.insert(cache_key, result);

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(35, Task.part1(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));

        assert_eq!(
            220,
            Task.part1(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, Task.part2(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]));

        assert_eq!(
            19208,
            Task.part2(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ])
        );
    }
}
