use super::super::*;
use itertools::Itertools;

pub struct Task;

impl Solution for Task {
    type Input = (Vec<i64>, usize);
    type Output = i64;

    fn parse_input(&self, input: String) -> Self::Input {
        (input.trim().lines().flat_map(str::parse).collect(), 25)
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        let (xmas, preamble) = input;

        xmas.windows(preamble + 1)
            .find(|w| {
                !w[..*preamble]
                    .iter()
                    .tuple_combinations()
                    .any(|(a, b)| a + b == w[*preamble])
            })
            .map(|w| w[*preamble])
            .expect("no solution found")
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        let wrong = self.part1(input);
        let (xmas, _) = input;

        (2..xmas.len())
            .flat_map(|n| xmas.windows(n).find(|w| w.iter().sum::<i64>() == wrong))
            .next()
            .map(|w| w.iter().minmax().into_option().map(|(a, b)| a + b).unwrap())
            .expect("no solution found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            127,
            Task.part1(&(
                vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            62,
            Task.part2(&(
                vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ))
        );
    }
}
