use regex::Regex;
use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day2.txt").expect("Error reading the file")
}

struct PasswordWithPolicy<'a> {
    password: &'a str,
    letter: char,
    min: usize,
    max: usize,
}

fn input<'a>(string: &'a str) -> Vec<PasswordWithPolicy> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)").expect("invalid regex");
    }

    string
        .trim()
        .lines()
        .map(|line| {
            let caps = RE.captures(line).expect("invalid entry format");
            PasswordWithPolicy {
                min: caps[1].parse().expect("min value is not a number"),
                max: caps[2].parse().expect("max value is not a number"),
                letter: caps[3].parse().expect("letter is not one char"),
                password: caps.get(4).unwrap().as_str(),
            }
        })
        .collect()
}

fn part1(input: &[PasswordWithPolicy]) -> usize {
    input
        .iter()
        .filter(|p| (p.min..(p.max + 1)).contains(&p.password.matches(p.letter).count()))
        .count()
}

fn part2(input: &[PasswordWithPolicy]) -> usize {
    input
        .iter()
        .filter(|p| {
            let first = p.password.chars().nth(p.min - 1) == Some(p.letter);
            let second = p.password.chars().nth(p.max - 1) == Some(p.letter);
            first != second
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            2,
            part1(&input(
                &"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
            "
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            1,
            part2(&input(
                &"
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
            "
            ))
        );
    }
}
