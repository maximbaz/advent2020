use super::super::*;
use regex::Regex;

pub struct Task;

pub struct PasswordWithPolicy {
    password: String,
    letter: char,
    min: usize,
    max: usize,
}

impl Solution for Task {
    type Input = Vec<PasswordWithPolicy>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").expect("invalid regex");
        }

        input
            .trim()
            .lines()
            .map(|line| {
                let caps = RE.captures(line).expect("invalid entry format");

                PasswordWithPolicy {
                    min: caps[1].parse().expect("min value is not a number"),
                    max: caps[2].parse().expect("max value is not a number"),
                    letter: caps[3].parse().expect("letter is not one char"),
                    password: caps[4].to_string(),
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|p| (p.min..=p.max).contains(&p.password.matches(p.letter).count()))
            .count()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(|p| {
                let first = p.password.chars().nth(p.min - 1) == Some(p.letter);
                let second = p.password.chars().nth(p.max - 1) == Some(p.letter);
                first != second
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            Task.part1(
                &Task.parse_input(
                    "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            1,
            Task.part2(
                &Task.parse_input(
                    "
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
                    "
                    .to_string()
                )
            )
        );
    }
}
