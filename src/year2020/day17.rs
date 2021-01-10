use super::super::*;
use itertools::iproduct;
use std::collections::HashSet;

pub struct Task;

pub type Point = (i32, i32, i32, i32);
pub type Dimension = HashSet<Point>;

impl Solution for Task {
    type Input = Dimension;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(j, _)| ((i as i32, j as i32, 0, 0)))
            })
            .flatten()
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        simulate(input.clone(), false)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        simulate(input.clone(), true)
    }
}

fn simulate(input: Dimension, has_four_dim: bool) -> usize {
    let initial_size = (input.len() as f64).sqrt() as i32;

    (1..=6)
        .fold(input, |dim, cycle| {
            iproduct!(
                (-cycle..=cycle + initial_size),
                (-cycle..=cycle + initial_size),
                (-cycle..=cycle + 1),
                (-cycle..=cycle + 1)
            )
            .filter_map(|p| get_state(&p, &dim, has_four_dim))
            .collect()
        })
        .len()
}

fn get_state(p: &Point, dimension: &Dimension, has_four_dim: bool) -> Option<Point> {
    let neighbors = iproduct!((-1..=1), (-1..=1), (-1..=1), (-1..=1))
        .filter(|&dp| dp != (0, 0, 0, 0))
        .filter(|&dp| has_four_dim || dp.3 == 0)
        .filter(|&dp| dimension.contains(&(p.0 + dp.0, p.1 + dp.1, p.2 + dp.2, p.3 + dp.3)))
        .count();

    match dimension.contains(p) {
        true if neighbors == 2 || neighbors == 3 => Some(*p),
        false if neighbors == 3 => Some(*p),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            112,
            Task.part1(
                &Task.parse_input(
                    "
.#.
..#
###
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            848,
            Task.part2(
                &Task.parse_input(
                    "
.#.
..#
###
                    "
                    .to_string()
                )
            )
        );
    }
}
