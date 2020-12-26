use super::super::*;

pub struct Task;

impl Solution for Task {
    type Input = Vec<(char, i64)>;
    type Output = i64;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|s| (s.chars().next().unwrap(), s[1..].parse().unwrap()))
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> Self::Output {
        match input
            .iter()
            .fold((0, 0, (0, 1)), |(i, j, waypoint), step| match *step {
                ('N', value) => (i - value, j, waypoint),
                ('S', value) => (i + value, j, waypoint),
                ('E', value) => (i, j + value, waypoint),
                ('W', value) => (i, j - value, waypoint),
                ('L', value) => (i, j, rotate_waypoint(waypoint, -value)),
                ('R', value) => (i, j, rotate_waypoint(waypoint, value)),
                ('F', value) => (i + waypoint.0 * value, j + waypoint.1 * value, waypoint),
                _ => unreachable!("invalid action"),
            }) {
            (i, j, _) => i.abs() + j.abs(),
        }
    }

    fn part2(&self, input: &Self::Input) -> Self::Output {
        match input
            .iter()
            .fold((0, 0, (-1, 10)), |(i, j, waypoint), step| match *step {
                ('N', value) => (i, j, (waypoint.0 - value, waypoint.1)),
                ('S', value) => (i, j, (waypoint.0 + value, waypoint.1)),
                ('E', value) => (i, j, (waypoint.0, waypoint.1 + value)),
                ('W', value) => (i, j, (waypoint.0, waypoint.1 - value)),
                ('L', value) => (i, j, rotate_waypoint(waypoint, -value)),
                ('R', value) => (i, j, rotate_waypoint(waypoint, value)),
                ('F', value) => (i + waypoint.0 * value, j + waypoint.1 * value, waypoint),
                _ => unreachable!("invalid action"),
            }) {
            (i, j, _) => i.abs() + j.abs(),
        }
    }
}

fn rotate_waypoint(waypoint: (i64, i64), deg: i64) -> (i64, i64) {
    match (deg + 360) % 360 {
        0 => (waypoint.0, waypoint.1),
        180 => (-waypoint.0, -waypoint.1),
        90 => (waypoint.1, -waypoint.0),
        270 => (-waypoint.1, waypoint.0),
        _ => unreachable!("invalid rotation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            25,
            Task.part1(&vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)])
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            286,
            Task.part2(&vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)])
        );
    }
}
