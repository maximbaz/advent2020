use std::fs;

pub fn run_part1() -> usize {
    part1(&input(&read_file()))
}

pub fn run_part2() -> usize {
    part2(&input(&read_file()))
}

fn read_file() -> String {
    fs::read_to_string("input/day3.txt").expect("Error reading the file")
}

type Map = Vec<Vec<bool>>;

fn input<'a>(string: &'a str) -> Map {
    string
        .trim()
        .lines()
        .map(|line| line.to_owned().chars().map(|c| c == '#').collect())
        .collect()
}

fn part1(input: &Map) -> usize {
    solve(input, 3, 1)
}

fn part2(input: &Map) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .cloned()
        .map(|(right, down)| solve(input, right, down))
        .product()
}

fn solve(input: &Map, right: usize, down: usize) -> usize {
    input
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, row)| row[(i * right) % row.len()] as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            7,
            part1(&input(
                &"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
                .to_owned()
            ))
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            336,
            part2(&input(
                &"
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
                .to_owned()
            ))
        );
    }
}
