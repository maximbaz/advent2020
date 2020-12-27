use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<usize>;
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .split(",")
            .map(|s| s.parse().expect("invalid number"))
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        solve(input, 2020)
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        solve(input, 30_000_000)
    }
}

fn solve(input: &Vec<usize>, stop: usize) -> usize {
    let mut mem = vec![0; stop];
    for turn in 0..input.len() - 1 {
        mem[input[turn]] = turn + 1;
    }

    (input.len()..stop).fold(*input.last().unwrap(), |last, turn| {
        let last_turn = mem[last];
        let next = if last_turn == 0 { 0 } else { turn - last_turn };
        mem[last] = turn;
        next
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(436, Task.part1(&vec![0, 3, 6]));
        assert_eq!(1, Task.part1(&vec![1, 3, 2]));
        assert_eq!(10, Task.part1(&vec![2, 1, 3]));
        assert_eq!(27, Task.part1(&vec![1, 2, 3]));
        assert_eq!(78, Task.part1(&vec![2, 3, 1]));
        assert_eq!(438, Task.part1(&vec![3, 2, 1]));
        assert_eq!(1836, Task.part1(&vec![3, 1, 2]));
    }
}
