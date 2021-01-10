use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<Vec<bool>>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.to_owned().chars().map(|c| c == '#').collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input, 3, 1)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(right, down)| solve(input, *right, *down))
            .product()
    }
}

fn solve(input: &Vec<Vec<bool>>, right: usize, down: usize) -> usize {
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
            Task.part1(
                &Task.parse_input(
                    "
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
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            336,
            Task.part2(
                &Task.parse_input(
                    "
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
                    .to_string()
                )
            )
        );
    }
}
