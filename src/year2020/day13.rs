use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = (usize, Vec<usize>);
    type Output = usize;

    fn parse_input(&self, input: String) -> Self::Input {
        let lines = input.trim().lines().collect_vec();

        (
            lines[0].parse().expect("invalid timestamp"),
            lines[1]
                .split(',')
                .map(|n| n.parse().unwrap_or(0))
                .collect_vec(),
        )
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        input
            .1
            .iter()
            .filter(|&&bus| bus != 0)
            .map(|bus| (bus - input.0 % bus, bus))
            .min()
            .map(|(wait, bus)| wait * bus)
            .unwrap()
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        input
            .1
            .iter()
            .enumerate()
            .filter(|(_, &bus)| bus != 0)
            .fold((0, 1), |(time, step), (offset, bus)| {
                (
                    (time..)
                        .step_by(step)
                        .find(|time| (time + offset) % bus == 0)
                        .unwrap(),
                    step * bus,
                )
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(295, Task.part1(&(939, vec![7, 13, 0, 0, 59, 0, 31, 19])));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            1068781,
            Task.part2(&(939, vec![7, 13, 0, 0, 59, 0, 31, 19]))
        );
        assert_eq!(3417, Task.part2(&(939, vec![17, 0, 13, 19])));
        assert_eq!(754018, Task.part2(&(939, vec![67, 7, 59, 61])));
        assert_eq!(779210, Task.part2(&(939, vec![67, 0, 7, 59, 61])));
        assert_eq!(1261476, Task.part2(&(939, vec![67, 7, 0, 59, 61])));
        assert_eq!(1202161486, Task.part2(&(939, vec![1789, 37, 47, 1889])));
    }
}
