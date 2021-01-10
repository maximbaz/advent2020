use super::super::*;
use regex::Regex;
use std::collections::HashMap;

pub struct Task;

pub struct Block {
    mask: String,
    mem: Vec<(usize, usize)>,
}

impl Solution for Task {
    type Input = Vec<Block>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").expect("invalid regex");
        }

        input
            .trim()
            .lines()
            .fold(vec![], |mut parsed, line| {
                if line.starts_with("mask") {
                    parsed.insert(
                        0,
                        Block {
                            mask: line[7..].to_string(),
                            mem: vec![],
                        },
                    );
                } else {
                    let caps = RE.captures(line).expect("invalid memory line format");
                    parsed[0].mem.push((
                        caps[1].parse().expect("address is not a number"),
                        caps[2].parse().expect("value is not a number"),
                    ));
                }
                parsed
            })
            .into_iter()
            .rev()
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .fold(HashMap::new(), |mut mem, block| {
                block.mem.iter().for_each(|(addr, val)| {
                    mem.insert(addr, val & block.mask_and() | block.mask_or());
                });
                mem
            })
            .values()
            .sum()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        input
            .iter()
            .fold(HashMap::new(), |mut mem, block| {
                block.masks().iter().for_each(|(mask_and, mask_or)| {
                    block.mem.iter().for_each(|(addr, val)| {
                        mem.insert(addr & mask_and | mask_or, *val);
                    });
                });
                mem
            })
            .values()
            .sum()
    }
}

impl Block {
    fn mask_and(&self) -> usize {
        usize::from_str_radix(&self.mask.replace("X", "1"), 2).unwrap()
    }

    fn mask_or(&self) -> usize {
        usize::from_str_radix(&self.mask.replace("X", "0"), 2).unwrap()
    }

    fn masks(&self) -> Vec<(usize, usize)> {
        self.mask.chars().fold(vec![(0, 0)], |res, c| match c {
            'X' => res
                .iter()
                .map(|(and, or)| (and << 1 | 1, or << 1 | 1))
                .chain(res.iter().map(|(and, or)| (and << 1, or << 1)))
                .collect_vec(),
            '1' => res
                .iter()
                .map(|(and, or)| (and << 1 | 1, or << 1 | 1))
                .collect_vec(),
            '0' => res
                .iter()
                .map(|(and, or)| (and << 1 | 1, or << 1))
                .collect_vec(),
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            165,
            Task.part1(
                &Task.parse_input(
                    "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            208,
            Task.part2(
                &Task.parse_input(
                    "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
                    "
                    .to_string()
                )
            )
        );
    }
}
