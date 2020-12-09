use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day7.txt").expect("Error reading the file")
}

type BagsInside<'a> = HashMap<&'a str, usize>;
type Bags<'a> = HashMap<&'a str, BagsInside<'a>>;

fn input(string: &str) -> Bags {
    string.trim().lines().map(parse).collect()
}

fn parse(s: &str) -> (&str, BagsInside) {
    lazy_static! {
        static ref RE_BAG: Regex = Regex::new(r"^\w+ \w+").expect("invalid regex");
        static ref RE_CONTENTS: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").expect("invalid regex");
    }

    (
        RE_BAG.find(s).expect("invalid bag").as_str(),
        RE_CONTENTS
            .captures_iter(s)
            .map(|cap| {
                (
                    cap.get(2).expect("invalid bag contents").as_str(),
                    cap[1].parse().expect("bag count is not a number"),
                )
            })
            .collect(),
    )
}

fn check(input: &Bags, key: &str) -> bool {
    input
        .get(key)
        .map(|bag| bag.keys().any(|k| *k == "shiny gold" || check(input, k)))
        .unwrap_or_default()
}

fn count(input: &Bags, key: &str) -> usize {
    input
        .get(key)
        .map(|bag| bag.iter().fold(1, |sum, (k, n)| sum + n * count(input, k)))
        .unwrap_or(1)
}

fn part1(input: &Bags) -> usize {
    input.keys().filter(|k| check(input, k)).count()
}

fn part2(input: &Bags) -> usize {
    count(input, "shiny gold") - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            4,
            part1(&input(
                &"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
            "
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            32,
            part2(&input(
                &"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
            "
            ))
        );

        assert_eq!(
            126,
            part2(&input(
                &"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
            "
            ))
        );
    }
}
