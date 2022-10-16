use super::super::*;

pub struct Task;

pub enum Command {
    Down(i32),
    Up(i32),
    Forward(i32),
}

impl Solution for Task {
    type Input = Vec<Command>;
    type Output = i32;

    fn parse_input(&self, input: String) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|s| {
                let p = s.split(' ').collect_vec();
                let val = p[1].parse().expect("value is not a number");
                match p[0] {
                    "down" => Command::Down(val),
                    "up" => Command::Up(val),
                    "forward" => Command::Forward(val),
                    _ => unreachable!("unsupported command"),
                }
            })
            .collect()
    }

    fn part1(&self, input: Self::Input) -> Self::Output {
        let (h, d) = input.iter().fold((0, 0), |(h, d), cmd| match cmd {
            Command::Down(val) => (h, d + val),
            Command::Up(val) => (h, d - val),
            Command::Forward(val) => (h + val, d),
        });
        h * d
    }

    fn part2(&self, input: Self::Input) -> Self::Output {
        let (h, d, _) = input.iter().fold((0, 0, 0), |(h, d, a), cmd| match cmd {
            Command::Down(val) => (h, d, a + val),
            Command::Up(val) => (h, d, a - val),
            Command::Forward(val) => (h + val, d + a * val, a),
        });
        h * d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            150,
            Task.part1(
                Task.parse_input(
                    "
forward 5
down 5
forward 8
up 3
down 8
forward 2
                    "
                    .to_string()
                )
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            900,
            Task.part2(
                Task.parse_input(
                    "
forward 5
down 5
forward 8
up 3
down 8
forward 2
                    "
                    .to_string()
                )
            )
        );
    }
}
