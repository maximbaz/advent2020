use itertools::Itertools;
use std::fs;

pub fn run_part1() -> i64 {
    part1(&input(&read_file()), 25)
}

pub fn run_part2() -> i64 {
    part2(&input(&read_file()), 25)
}

fn read_file() -> String {
    fs::read_to_string("input/day9.txt").expect("Error reading the file")
}

fn input<'a>(string: &'a str) -> Vec<i64> {
    string
        .trim()
        .lines()
        .map(|v| v.parse().expect("invalid number"))
        .collect()
}

fn part1(input: &[i64], preamble: usize) -> i64 {
    input
        .windows(preamble + 1)
        .find(|w| {
            !w[..preamble]
                .iter()
                .combinations(2)
                .any(|p| p[0] + p[1] == w[preamble])
        })
        .map(|w| w[preamble])
        .expect("no solution found")
}

fn part2(input: &[i64], preamble: usize) -> i64 {
    let wrong = part1(input, preamble);

    (2..input.len())
        .flat_map(|n| input.windows(n).find(|w| w.iter().sum::<i64>() == wrong))
        .next()
        .map(|w| w.iter().min().unwrap() + w.iter().max().unwrap())
        .expect("no solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            127,
            part1(
                &vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            62,
            part2(
                &vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            )
        );
    }
}
