use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<i32>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input.trim().lines().flat_map(str::parse).collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        solve(&input, 1)
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        solve(&input, 3)
    }
}

fn solve(input: &[i32], window_size: usize) -> usize {
    let sums = input.windows(window_size).map(|v| v.iter().sum::<i32>());
    sums.clone()
        .zip(sums.skip(1))
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            7,
            Task.part1(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            5,
            Task.part2(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        );
    }
}
