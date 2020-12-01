use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
pub enum Day {
    Day1 = 1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

#[derive(FromPrimitive)]
pub enum Part {
    One = 1,
    Two,
}

pub struct Config {
    pub day: Day,
    pub part: Part,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Usage: ./app <day> <part>");
        }

        let day = match args[1].as_str().parse() {
            Ok(number) => match Day::from_i32(number) {
                Some(val) => val,
                None => return Err("Day must be between 1 and 25"),
            },
            Err(_) => return Err("Day must be an integer"),
        };

        let part = match args[2].as_str().parse() {
            Ok(number) => match Part::from_i32(number) {
                Some(val) => val,
                None => return Err("Part must be 1 or 2"),
            },
            Err(_) => return Err("Part must be integer"),
        };

        Ok(Config { day, part })
    }
}
