use super::super::*;
use itertools::iproduct;
use std::convert::TryFrom;

pub struct Task;

pub type Grid = Vec<Vec<char>>;

impl Solution for Task {
    type Input = Grid;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        stabilize(input, |input, row, col| match input[row][col] {
            'L' if count_occupied(&get_adjacent(input, row, col)) == 0 => '#',
            '#' if count_occupied(&get_adjacent(input, row, col)) > 3 => 'L',
            c => c,
        })
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        stabilize(input, |input, row, col| match input[row][col] {
            'L' if count_occupied(&get_visible(input, row, col)) == 0 => '#',
            '#' if count_occupied(&get_visible(input, row, col)) > 4 => 'L',
            c => c,
        })
    }
}

fn stabilize<F: Fn(&Grid, usize, usize) -> char>(input: &Grid, model: F) -> usize {
    let mut occupied = 0;
    let mut data = input.clone();

    loop {
        data = data
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| model(&data, i, j))
                    .collect()
            })
            .collect();

        match count_all_occupied(&data) {
            n if n == occupied => return n,
            n => occupied = n,
        }
    }
}

fn get_adjacent(input: &Grid, i: usize, j: usize) -> Vec<&char> {
    iproduct!(-1..=1, -1..=1)
        .filter(|&dir| dir != (0, 0))
        .filter_map(|(di, dj)| try_get(input, i, di, j, dj, 1))
        .collect()
}

fn get_visible(input: &Grid, i: usize, j: usize) -> Vec<&char> {
    iproduct!(-1..=1, -1..=1)
        .filter(|&dir| dir != (0, 0))
        .filter_map(|(di, dj)| {
            (1..)
                .map(|mul| try_get(input, i, di, j, dj, mul))
                .find(|&c| c != Some(&'.'))
                .unwrap()
        })
        .collect()
}

fn try_get(input: &Grid, i: usize, di: i64, j: usize, dj: i64, mul: i64) -> Option<&char> {
    usize::try_from((i as i64) + di * mul).ok().and_then(|row| {
        usize::try_from((j as i64) + dj * mul)
            .ok()
            .and_then(|col| input.get(row).and_then(|r| r.get(col)))
    })
}

fn count_occupied(input: &[&char]) -> usize {
    input.iter().filter(|&&&c| c == '#').count()
}

fn count_all_occupied(input: &Grid) -> usize {
    count_occupied(&input.iter().flatten().collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            37,
            Task.part1(
                &Task.parse_input(
                    "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

            "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            26,
            Task.part2(
                &Task.parse_input(
                    "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

            "
                    .to_string()
                )
            )
        );
    }
}
