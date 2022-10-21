use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<Vec<u32>>;
    type Output = u32;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        let (gamma, epsilon) = (0..input[0].len())
            .map(|i| input.iter().map(|inner| inner[i]).collect_vec())
            .map(|line| line.iter().sum::<u32>() / ((line.len() as u32 + 1) / 2))
            .map(|v| (v, 1 - v))
            .fold((0, 0), |(gamma, epsilon), (most, least)| {
                (gamma << 1 | most, epsilon << 1 | least)
            });
        gamma * epsilon
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        vec![true, false]
            .iter()
            .filter_map(|&criteria| {
                (0..input[0].len())
                    .scan(input.clone(), |data, i| {
                        let half = (data.len() + 1) / 2;
                        let more_ones = data.iter().filter(|line| line[i] == 1).count() >= half;
                        data.retain(|line| line[i] == (more_ones == criteria) as u32);
                        data.first().cloned()
                    })
                    .last()
            })
            .map(|bin| bin.iter().fold(0, |result, bit| result << 1 | bit))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            198,
            Task.part1(
                Task.parse_input(
                    "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            230,
            Task.part2(
                Task.parse_input(
                    "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
                    "
                    .to_string()
                )
            )
        );
    }
}
