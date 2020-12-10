#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::env;
use std::fmt::Display;
use std::fs;
use std::time::Instant;

mod year2020;

pub trait Solution {
    type Input;
    type Output: Display;

    fn parse_input(&self, input: String) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output;
    fn part2(&self, input: &Self::Input) -> Self::Output;

    fn read_input(&self, year: u16, day: u8) -> String {
        fs::read_to_string(format!("input/{}/{:02}", year, day)).expect("Error reading the file")
    }

    fn solve(&self, year: u16, day: u8, part: u8) -> String {
        let input = &self.parse_input(self.read_input(year, day));
        if part == 1 {
            self.part1(input).to_string()
        } else {
            self.part2(input).to_string()
        }
    }
}

fn main() {
    let args = env::args().collect_vec();
    let year = args.get(1).map(|x| x.parse().expect("invalid number"));
    let day = args.get(2).map(|x| x.parse().expect("invalid number"));
    let part = args.get(3).map(|x| x.parse().expect("invalid number"));

    (2015..=2020)
        .cartesian_product((1..=25).cartesian_product(1..=2))
        .filter(|(y, (d, p))| {
            *y == year.unwrap_or(*y) && *d == day.unwrap_or(*d) && *p == part.unwrap_or(*p)
        })
        .for_each(|(year, (day, part))| match solve(year, day, part) {
            Some((answer, took)) => println!(
                "{} day {:02} part {} => {:15} (took {}ms)",
                year, day, part, answer, took,
            ),
            None => (),
        });
}

fn solve(year: u16, day: u8, part: u8) -> Option<(String, u128)> {
    match year {
        2020 => match day {
            1 => Some(measure(|| year2020::day01::Task.solve(year, day, part))),
            2 => Some(measure(|| year2020::day02::Task.solve(year, day, part))),
            3 => Some(measure(|| year2020::day03::Task.solve(year, day, part))),
            4 => Some(measure(|| year2020::day04::Task.solve(year, day, part))),
            5 => Some(measure(|| year2020::day05::Task.solve(year, day, part))),
            6 => Some(measure(|| year2020::day06::Task.solve(year, day, part))),
            7 => Some(measure(|| year2020::day07::Task.solve(year, day, part))),
            8 => Some(measure(|| year2020::day08::Task.solve(year, day, part))),
            9 => Some(measure(|| year2020::day09::Task.solve(year, day, part))),
            _ => None,
        },
        _ => None,
    }
}

fn measure<F: Fn() -> String>(f: F) -> (String, u128) {
    let now = Instant::now();
    (f(), now.elapsed().as_millis())
}
